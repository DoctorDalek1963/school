//! This module handles the tableaux. Tableau is singular; tableaux is plural.

mod labels;

use self::labels::{ColumnLabel, RowLabel};
use crate::{
    lin_prog::{
        comparison::Comparison,
        expression::{const_expression::VariableOrConst, ConstExpression},
        system::LinProgSystem,
        ObjectiveFunction,
    },
    simplex::{Equation, SolutionSet, VariableType},
    Frac,
};
use color_eyre::{Report, Result};
use fraction::Zero;
use itertools::Itertools;
use std::{collections::HashMap, fmt, iter};
use tabled::{builder::Builder, Style};
use thiserror::Error;
use tracing::{debug, error, info, instrument};

/// There is no feasible solution for the given [`LinProgSystem`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub struct NoFeasibleSolution;

impl fmt::Display for NoFeasibleSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "No feasible solution for the given system")
    }
}

/// The operation to be applied to a particular row.
#[derive(Clone, Copy, Debug, PartialEq)]
enum RowOperation {
    /// No-op; do nothing.
    Nop,

    /// Multiply every number in the row by a constant.
    MulConst(Frac),

    /// Add a multiple of another row to this row. The other row should always be the pivot row.
    ///
    /// The `usize` here is the index of the row, so it starts at 0. When printing it with
    /// [`Display`], we increment it.
    AddRow(Frac, usize),
}

impl fmt::Display for RowOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RowOperation::Nop => "Nop".to_string(),
            RowOperation::MulConst(n) => format!("×{n}"),
            RowOperation::AddRow(n, idx) => {
                if *n > Frac::zero() {
                    format!("+{n} R{}", idx + 1)
                } else {
                    format!("{n} R{}", idx + 1)
                }
            }
        };
        write!(f, "{s}")
    }
}

/// A number to use in a tableau. This is used to allow certain values (like theta) to be optional,
/// as well as allowing for the row operation columns.
#[derive(Clone, Copy, Debug, PartialEq)]
enum TableauNumber {
    /// A simple number.
    Simple(Frac),

    /// A theta value, which will not exist at first, since it must be populated later.
    Theta(Option<Frac>),

    /// The operation to be applied to the row.
    RowOperation(Option<RowOperation>),
}

impl fmt::Display for TableauNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Simple(n) => n.to_string(),
            Self::Theta(Some(n)) => n.to_string(),
            Self::RowOperation(Some(op)) => op.to_string(),
            Self::Theta(None) | Self::RowOperation(None) => String::new(),
        };
        write!(f, "{s}")
    }
}

impl TableauNumber {
    /// Return the number if this is a simple tableau number, otherwise panic.
    ///
    /// This method should only be used if you know the number is simple.
    fn simple_num(&self) -> &Frac {
        match self {
            Self::Simple(n) => n,
            x => panic!("TableauNumber::simple_num() called on a non-simple number: {x:?}"),
        }
    }
}

/// A single tableau for simplex tableaux.
#[derive(Clone, Debug)]
pub struct Tableau<'v> {
    /// The titles of the columns.
    column_labels: Vec<ColumnLabel<'v>>,

    /// The rows of the table.
    rows: Vec<(RowLabel<'v>, Vec<TableauNumber>)>,

    /// Hold a reference to the system to check against constraints at the end.
    ///
    /// Ouroboros prevents us from holding a direct reference to the constraints.
    system: &'v LinProgSystem,

    /// Whether to minimise the objective function rather than the default of maximising it.
    minimise: bool,

    /// Whether we need integer solutions.
    integer_solutions: bool,

    /// The index of the value column.
    value_idx: usize,

    /// The index of the theta column.
    theta_idx: usize,

    /// The index of the row ops column.
    row_ops_idx: usize,
}

impl<'v> fmt::Display for Tableau<'v> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = Builder::default();
        builder.add_record(
            iter::once("Basic var".into()).chain(self.column_labels.iter().map(|s| s.to_string())),
        );

        for (label, nums) in &self.rows {
            builder.add_record(
                iter::once(label.to_string()).chain(nums.iter().map(|n| n.to_string())),
            );
        }

        let table = builder.build().with(Style::modern()).to_string();

        write!(f, "\n{table}")
    }
}

impl<'v> Tableau<'v> {
    /// Generate the initial tableau for the given system with its variables and equations.
    #[instrument(skip(system))]
    pub fn create_initial(system: &'v LinProgSystem) -> Result<Self> {
        let minimise = system
            .with_objective_function(|obj_func| matches!(obj_func, ObjectiveFunction::Minimise(_)));

        // Convert the original variables from the system into [`VariableType::Original`] variables.
        // This HashMap maps variables to their current values. These values will change during the
        // execution of the algorithm.
        let mut variables: Vec<(VariableType<'v>, Frac)> = system
            .borrow_variables()
            .0
            .iter()
            .sorted() // Alphabetically
            .map(|s| (VariableType::Original(s), Frac::zero()))
            .collect();

        debug!(?variables);

        let mut slack_counter = 0;
        let mut surplus_counter = 0;
        let mut artificial_counter = 0;
        let mut equations = vec![];

        // Convert the constraints to equations, creating necessary slack variables
        system.with_constraints(|cons| {
            for constraint in cons {
                match constraint.comparison {
                    Comparison::LessThanOrEqual => {
                        // When creating a new slack variable, we need to increment the counter for the
                        // next one and add it to the simplex variables set, with a starting value of the
                        // constant, since the original variables start at 0
                        let slack = VariableType::Slack(slack_counter);
                        slack_counter += 1;
                        variables.push((slack, constraint.constant));

                        // Convert the old variables from the constraint into the required type and add the
                        // slack variable for this equation
                        let eqn_variables = constraint
                            .var_expression
                            .0
                            .iter()
                            .map(|&(coeff, var)| (coeff, VariableType::Original(var)))
                            .chain(iter::once((1.into(), slack)))
                            .collect();

                        // Add the equation to the vec
                        equations.push(Equation {
                            variables: eqn_variables,
                            constant: constraint.constant,
                        });
                    }
                    Comparison::GreaterThanOrEqual => {
                        let surplus = VariableType::Surplus(surplus_counter);
                        surplus_counter += 1;
                        // The surplus variable starts at 0
                        variables.push((surplus, Frac::zero()));

                        let artificial = VariableType::Artificial(artificial_counter);
                        artificial_counter += 1;
                        // The artificial variable starts at the constraint's constant
                        variables.push((artificial, constraint.constant));

                        let eqn_variables = constraint
                            .var_expression
                            .0
                            .iter()
                            .map(|&(coeff, var)| (coeff, VariableType::Original(var)))
                            .chain(
                                [(-Frac::new(1u32, 1u32), surplus), (1.into(), artificial)]
                                    .into_iter(),
                            )
                            .collect();

                        equations.push(Equation {
                            variables: eqn_variables,
                            constant: constraint.constant,
                        })
                    }
                    _ => {
                        error!(
                            comparison = ?constraint.comparison,
                            %constraint,
                            "Unsupported comparison in constraint"
                        );
                        return Err(Report::msg(format!(
                            "Unsupported comparison in constraint: {constraint:?}",
                        )));
                    }
                };
            }
            Ok(())
        })?;

        debug!(?equations);

        // Sort the variables by type, so it goes original, slack, surplus, artificial
        variables.sort_by_key(|&(var, _)| var);

        let column_labels = variables
            .iter()
            .map(|&(var, _)| var.into())
            .chain(["Value".into(), "θ".into(), "Row op".into()].into_iter())
            .collect();

        // Each row has n + 3 columns, where n is the number of variables. We have a column for each
        // variable, a column for the value, a column for theta, and a column for the row operation
        let rows = variables
            .iter()
            // Filter the variables to just the slack, surplus, and artificial variables. These are
            // the basic variables at the start
            .filter_map(|&(var, _)| match var {
                VariableType::Original(_) | VariableType::Surplus(_) => None,
                VariableType::Slack(_) | VariableType::Artificial(_) => {
                    Some(RowLabel::Variable(var))
                },
            })
            .map(|label| {
                (
                    label.clone(),
                    equations
                        .iter()
                        .find_map(|equation| {
                            // Try to find a term with the variable in this row of the table. We don't care about
                            // that term right now, so we discard it. But the ? at the end means that if we can't
                            // find a term with the required variable, then we return from the closure early
                            equation
                                .variables
                                .iter()
                                .find(|&&(_, var)| RowLabel::Variable(var) == label)?;

                            // If we get here, then we know this is the right equation. We need to extract the
                            // coefficients IN THE RIGHT ORDER, so we iter the variables and find the
                            // coefficient of each one
                            Some(
                                variables
                                    .iter()
                                    .map(|&(var, _)| {
                                        equation
                                            .variables
                                            .iter()
                                            .find_map(|&(n, eq_var)| if eq_var == var { Some(n) } else { None })
                                            .unwrap_or(Frac::zero())
                                    })
                                    .collect::<Vec<_>>()
                            )
                        })
                        .expect("There should be at least one equation which contains this slack variable")
                )
            })
            // Now add the value to the end
            .map(|(label, mut coeffs)| {
                coeffs.push(
                    variables
                        .iter()
                        .find(|&(var, _)| RowLabel::Variable(*var) == label)
                        .unwrap()
                        .1
                );
                (label, coeffs)
            })
            // Now add the final row for the objective function
            .chain(
                iter::once((
                    RowLabel::ObjectiveFunction,
                    system.with_objective_function(|obj_func| {
                        // Go through the variables and find the coefficient of each one in the
                        // objective function
                        variables
                            .iter()
                            .map(|(var, _)| {
                                obj_func
                                    .expression()
                                    .0
                                    .iter()
                                    .find_map(|&(coeff, of_var)| {
                                        if VariableType::Original(of_var) == *var {
                                            if minimise {
                                                Some(coeff)
                                            } else {
                                                Some(-coeff)
                                            }
                                        } else {
                                            None
                                        }
                                    })
                                    .unwrap_or(Frac::zero())
                            })
                            // And add the value
                            .chain(iter::once(Frac::zero()))
                            .collect()
                    })
                ))
            )
            // Add the row for the new objective function for the first stage of the two stage
            // simplex if necessary
            .chain(
                {
                    let iterator: Box<dyn Iterator<Item = (RowLabel, Vec<Frac>)>> =
                        if surplus_counter > 0 || artificial_counter > 0 {
                            Box::new(iter::once((
                                RowLabel::TwoStageArtificial,
                                {
                                    // We want a new objective function I = -sum(artificials)
                                    let new_obj_func: ConstExpression<'_, VariableType<'_>> = equations
                                        .iter()
                                        // Filter equations down to just those containing
                                        // artificial variables
                                        .filter(|&eq| {
                                            eq.variables
                                                .iter()
                                                .find(|&(_, var)| matches!(var, VariableType::Artificial(_)))
                                                .is_some()
                                        })
                                        // Solve for each artificial variable
                                        .map(|eq| ConstExpression(
                                            iter::once(VariableOrConst::Constant(eq.constant))
                                                .chain(
                                                    eq.variables
                                                        .iter()
                                                        // Filter out artificials, since we're
                                                        // solving for the artificials
                                                        .filter(|&(_, var)| !matches!(var, VariableType::Artificial(_)))
                                                        .map(|(coeff, var)| VariableOrConst::Variable(-coeff, var))
                                                )
                                                .collect()
                                        ))
                                        // Sum the expressions
                                        .sum();

                                    // Negate the expression sum
                                    let new_obj_func = -new_obj_func;

                                    variables
                                        .iter()
                                        .map(|(variable, _)| {
                                            new_obj_func.0
                                                .iter()
                                                // Find this variable in the new objective function
                                                .find_map(|var_or_const| match var_or_const {
                                                    VariableOrConst::Variable(num, var) if *var == variable => Some(-*num),
                                                    _ => None,
                                                })
                                                .unwrap_or(Frac::zero())
                                        })
                                        // Add the value to the end
                                        .chain(iter::once(
                                            new_obj_func
                                                .constant()
                                                .unwrap_or(Frac::zero())
                                        ))
                                        .collect()
                                }
                            )))
                        } else {
                            Box::new(iter::empty())
                        };
                    iterator
                }
            )
            // Convert the numbers to the right type and add the theta values
            .map(|(label, nums)| (
                    label,
                    nums
                        .into_iter()
                        .map(TableauNumber::Simple)
                        .chain([TableauNumber::Theta(None), TableauNumber::RowOperation(None)].into_iter())
                        .collect()
                ))
            .collect();

        let value_idx = variables.len();

        Ok(Self {
            column_labels,
            rows,
            system,
            minimise,
            integer_solutions: system.borrow_config().integer_solutions,
            value_idx,
            theta_idx: value_idx + 1,
            row_ops_idx: value_idx + 2,
        })
    }

    /// Return a reference to the bottom row of the table.
    fn bottom_row(&self) -> &(RowLabel<'v>, Vec<TableauNumber>) {
        self.rows.last().expect("There should be a bottom row")
    }

    /// Return the values in the theta column.
    fn theta_column(&self) -> Vec<Option<Frac>> {
        self.rows
            .iter()
            .map(|(_, numbers)| match numbers[self.theta_idx] {
                TableauNumber::Theta(n) => n,
                other => panic!("The theta column must only contain theta values, not {other:?}"),
            })
            .collect()
    }

    /// Check if there are any negative numbers in the bottom row of the tableau.
    pub fn negatives_in_bottom_row(&self) -> bool {
        self.bottom_row()
            .1
            .iter()
            .take(self.value_idx)
            .any(|&n| match n {
                TableauNumber::Simple(n) => n < Frac::zero(),
                _ => false,
            })
    }

    /// Return the index of the pivot column. This is calculated by finding the most negative
    /// number in the bottom row.
    fn find_pivot_column(&self) -> usize {
        self.bottom_row()
            .1
            .iter()
            .enumerate()
            .take(self.value_idx)
            .filter_map(|(idx, num)| match num {
                TableauNumber::Simple(n) if *n < Frac::zero() => Some((idx, *n)),
                _ => None,
            })
            .fold(
                (0, Frac::zero()),
                |(acc_idx, acc_min), (this_idx, this_num)| {
                    if this_num < acc_min {
                        (this_idx, this_num)
                    } else {
                        (acc_idx, acc_min)
                    }
                },
            )
            .0
    }

    /// Return the index of the pivot row. This is calculated by finding the smallest positive
    /// theta value.
    fn find_pivot_row(&self) -> usize {
        self.theta_column()
            .iter()
            .enumerate()
            .filter_map(|(idx, &theta)| match theta {
                Some(n) if n > Frac::zero() => Some((idx, n)),
                _ => None,
            })
            .fold(
                (0, Frac::infinity()),
                |(acc_idx, acc_min), (this_idx, this_num)| {
                    if this_num < acc_min {
                        (this_idx, this_num)
                    } else {
                        (acc_idx, acc_min)
                    }
                },
            )
            .0
    }

    /// Populate this tableau with theta values.
    fn populate_theta_values(&mut self) {
        let pivot_col = self.find_pivot_column();
        for (label, numbers) in &mut self.rows {
            match label {
                RowLabel::Variable(_) => {
                    numbers[self.theta_idx] = TableauNumber::Theta(Some(
                        numbers[self.value_idx].simple_num() / numbers[pivot_col].simple_num(),
                    ));
                }
                RowLabel::ObjectiveFunction | RowLabel::TwoStageArtificial => (),
            }
        }
    }

    /// Change the label of the pivot row to be that of the pivot column.
    fn change_pivot_row_label(&mut self) {
        let pivot_col = self.find_pivot_column();
        let pivot_row = self.find_pivot_row();

        self.rows[pivot_row].0 = self.column_labels[pivot_col]
            .clone()
            .try_into()
            .expect("The pivot column should have a variable label");
    }

    /// Populate this tableau with row operations.
    fn populate_row_ops(&mut self) {
        let pivot_col = self.find_pivot_column();
        let pivot_row = self.find_pivot_row();

        for (idx, (_label, nums)) in self.rows.iter_mut().enumerate() {
            if idx == pivot_row {
                nums[self.row_ops_idx] = TableauNumber::RowOperation(Some(RowOperation::MulConst(
                    nums[pivot_col].simple_num().recip(),
                )));
            } else {
                let row_op_coeff = -nums[pivot_col].simple_num();
                nums[self.row_ops_idx] = if row_op_coeff != Frac::zero() {
                    TableauNumber::RowOperation(Some(RowOperation::AddRow(row_op_coeff, pivot_row)))
                } else {
                    TableauNumber::RowOperation(Some(RowOperation::Nop))
                };
            }
        }
    }

    /// Perform the row operations that were previously calculated, and then clear the theta and
    /// row op columns.
    fn perform_row_ops(&mut self) {
        // First pass to apply row op to the pivot row
        for (_label, nums) in &mut self.rows {
            let row_op = match nums[self.row_ops_idx] {
                TableauNumber::RowOperation(Some(op)) => op,
                other => panic!("The row op must exist at this point and not be {other:?}"),
            };

            match row_op {
                RowOperation::Nop => (),
                // Multiply by a constant. This should only appear in the pivot row
                RowOperation::MulConst(multiplier) => {
                    for number in nums.iter_mut() {
                        if let TableauNumber::Simple(n) = number {
                            *n *= multiplier;
                        }
                    }
                }
                // Do nothing on this pass
                RowOperation::AddRow(_, _) => (),
            };
        }

        let pivot_row = self.find_pivot_row();
        let (_, pivot_row_nums) = self.rows[pivot_row].clone();

        // Second pass to apply other row ops and clear theta and row op columns
        for (_label, nums) in self.rows.iter_mut() {
            let row_op = match nums[self.row_ops_idx] {
                TableauNumber::RowOperation(Some(op)) => op,
                other => panic!("The row op must exist at this point and not be {other:?}"),
            };

            match row_op {
                // We currently only support adding multiples of the pivot row due to borrowing
                // problems
                RowOperation::AddRow(multiplier, idx) => {
                    assert_eq!(
                        idx, pivot_row,
                        "The index of the row to add must be the same as the pivot row"
                    );

                    for (num, other_num) in nums.iter_mut().zip(pivot_row_nums.iter()) {
                        if let TableauNumber::Simple(n) = num {
                            *n += multiplier * *other_num.simple_num();
                        }
                    }
                }
                // These have already been dealt with
                RowOperation::Nop | RowOperation::MulConst(_) => (),
            }

            // Clear the theta and row op columns
            nums[self.theta_idx] = TableauNumber::Theta(None);
            nums[self.row_ops_idx] = TableauNumber::RowOperation(None);
        }
    }

    /// Do a single iteration of the simplex tableaux algorithm.
    #[instrument(skip(self))]
    pub fn do_iteration(&mut self) -> Result<(), NoFeasibleSolution> {
        self.populate_theta_values();
        debug!(%self, "After populating theta values");

        self.change_pivot_row_label();
        self.populate_row_ops();
        debug!(%self, "After populating row ops and changing pivot row label");

        self.perform_row_ops();
        info!(%self, "After performing row ops");

        // If there are no negatives in the bottom row, then we need to check the value
        let bottom_row = self.bottom_row();
        if bottom_row.0 == RowLabel::TwoStageArtificial && !self.negatives_in_bottom_row() {
            // The value needs to be zero. If it is, then we can continue and find the optimal
            // solution. Otherwise, there is no feasible solution
            if bottom_row.1[self.value_idx] == TableauNumber::Simple(Frac::zero()) {
                // Remove the bottom row
                self.rows.remove(self.rows.len() - 1);

                // Get the indices of the artificial variable columns
                let artificial_indices = self
                    .column_labels
                    .iter()
                    .enumerate()
                    .filter(|&(_, label)| {
                        matches!(label, ColumnLabel::Variable(VariableType::Artificial(_)))
                    })
                    .map(|(idx, _)| idx)
                    .collect::<Vec<_>>();

                // Remove the artificial variables from the rows of the tableau
                for (_label, numbers) in self.rows.iter_mut() {
                    *numbers = numbers
                        .iter_mut()
                        .enumerate()
                        .filter(|(idx, _)| !artificial_indices.contains(idx))
                        .map(|(_, num)| *num)
                        .collect();
                }

                // Keep all the non-artificial variables in the column labels
                self.column_labels.retain(|label| {
                    !matches!(label, ColumnLabel::Variable(VariableType::Artificial(_)))
                });

                // Update the internal indices
                let offset = artificial_indices.len();
                self.value_idx -= offset;
                self.theta_idx -= offset;
                self.row_ops_idx -= offset;

                debug!(%self, "After removing TwoStageAr#");
            } else {
                error!(err = %NoFeasibleSolution {});
                return Err(NoFeasibleSolution);
            }
        }

        Ok(())
    }

    pub fn get_solution(self) -> SolutionSet<'v> {
        if self.negatives_in_bottom_row() {
            panic!("There must not be negatives in the bottom row when getting the solution");
        }

        // Find the value of the objective function.
        let mut objective_function_value = *self
            .rows
            .iter()
            .find(|&(label, _)| matches!(label, RowLabel::ObjectiveFunction))
            .expect("The objective function must have a value")
            .1[self.value_idx]
            .simple_num();

        if self.minimise {
            objective_function_value *= -1.;
        }

        let variable_values: HashMap<VariableType, Frac> = self
            // Get the variables from the column labels
            .column_labels
            .into_iter()
            .filter_map(|label| match label {
                ColumnLabel::Variable(var) => Some(var),
                _ => None,
            })
            // Find the values for each basic variable, defaulting to 0 if there's no row for them
            .map(|var| {
                (
                    var,
                    self.rows
                        .iter()
                        .find_map(|(row_label, nums)| {
                            if *row_label == RowLabel::Variable(var) {
                                Some(*nums[self.value_idx].simple_num())
                            } else {
                                None
                            }
                        })
                        .unwrap_or(Frac::zero()),
                )
            })
            .collect();

        if !self.integer_solutions {
            SolutionSet {
                objective_function_value,
                variable_values,
            }
        } else {
            let variable_options: HashMap<&str, (Frac, Frac)> = variable_values
                .iter()

                // We only care about the original variables here
                .filter_map(|(&var, &num)| match var {
                    VariableType::Original(v) => Some((v, num)),
                    _ => None,
                })

                .map(|(var, num)| (var, (num.floor(), num.ceil())))
                .collect();
            debug!(?variable_options);

            let var_count = variable_options.len();
            let points_around_optimal = variable_options
                .into_iter()

                // Split the interior tuples and flatten so we get a tuple for each possibility
                .map(|(var, (a, b))| [(var, a), (var, b)])
                .flatten()

                // Filter out negatives
                .filter(|&(_, num)| num >= Frac::zero())

                // Find all the permutations and get rid of any with duplicated variables like
                // [("x", 3), ("x", 4)]
                .permutations(var_count)
                .map(|possibility| {
                    possibility
                        .into_iter()
                        .unique_by(|&(var, _)| var)
                        .collect_vec()
                })
                .filter(|possibility| possibility.len() == var_count)

                // Sort the variables in each possibility and eliminate duplicates
                .map(|possibility| {
                    possibility.into_iter()
                        .sorted_by_key(|(var, _)| var.to_string()).collect_vec()
                })
                .unique_by(|possibility| format!("{possibility:?}"))
                .collect_vec();
            debug!(?points_around_optimal);

            let in_feasible_region = self.system.with_constraints(|cons| {
                points_around_optimal
                    .into_iter()
                    // Filter to get just the possibilities that satisfy every constraint
                    .filter(|possibility| {
                        cons.iter()
                            .all(|con| {
                                con.test(
                                    &possibility
                                    .iter()
                                    .map(|&tuple| tuple)
                                    .collect_vec()
                                )
                            })
                    })
                    .collect_vec()
            });
            debug!(?in_feasible_region);

            let (vars, objective_function_value) =
                self.system.with_objective_function(|obj_func| {
                    in_feasible_region
                    .into_iter()
                    .map(|possibility| {
                        let value = obj_func.expression().evaluate(&possibility);
                        (possibility, value)
                    })
                    //.max_by_key(|&(vars, value)| value)
                    .fold((vec![], Frac::zero()), |(acc_vars, acc_value), (cur_vars, cur_value)| {
                        if cur_value > acc_value {
                            (cur_vars, cur_value)
                        } else {
                            (acc_vars, acc_value)
                        }
                    })
                });
            let variable_values = vars
                .into_iter()
                .map(|(var, num)| (VariableType::Original(var), num))
                .collect();

            debug!(?objective_function_value, ?variable_values);

            SolutionSet {
                objective_function_value,
                variable_values,
            }
        }
    }
}
