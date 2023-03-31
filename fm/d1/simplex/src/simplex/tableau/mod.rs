//! This module handles the tableaux. Tableau is singular; tableaux is plural.

mod labels;

use self::labels::{ColumnLabel, RowLabel};
use super::SolutionSet;
use crate::{
    lin_prog::{comparison::Comparison, system::LinProgSystem},
    simplex::{Equation, VariableType},
};
use color_eyre::{Report, Result};
use itertools::Itertools;
use std::{fmt, iter};
use tabled::{builder::Builder, Style};
use tracing::{debug, error, info, instrument};

/// The operation to be applied to a particular row.
#[derive(Clone, Copy, Debug, PartialEq)]
enum RowOperation {
    /// No-op; do nothing.
    Nop,

    /// Multiply every number in the row by a constant.
    MulConst(f32),

    /// Add a multiple of another row to this row. The other row should always be the pivot row.
    ///
    /// The `usize` here is the index of the row, so it starts at 0. When printing it with
    /// [`Display`], we increment it.
    AddRow(f32, usize),
}

impl fmt::Display for RowOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RowOperation::Nop => "Nop".to_string(),
            RowOperation::MulConst(n) => format!("× {n}"),
            RowOperation::AddRow(n, idx) => {
                if *n > 0. {
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
    Simple(f32),

    /// A theta value, which will not exist at first, since it must be populated later.
    Theta(Option<f32>),

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
    fn simple_num(&self) -> &f32 {
        match self {
            Self::Simple(n) => n,
            x => panic!("TableauNumber::simple_num() called on a non-simple number: {x:?}"),
        }
    }
}

/// A single tableau for simplex tableaux.
#[derive(Clone, Debug, PartialEq)]
pub struct Tableau<'v> {
    /// The titles of the columns.
    column_labels: Vec<ColumnLabel<'v>>,

    /// The rows of the table.
    rows: Vec<(RowLabel<'v>, Vec<TableauNumber>)>,

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
        // Convert the original variables from the system into [`VariableType::Original`] variables.
        // This HashMap maps variables to their current values. These values will change during the
        // execution of the algorithm.
        let mut variables: Vec<(VariableType<'v>, f32)> = system
            .borrow_variables()
            .0
            .iter()
            .sorted() // Alphabetically
            .map(|s| (VariableType::Original(s), 0.))
            .collect();

        debug!(?variables);

        let mut slack_counter = 0;
        let mut equations = vec![];

        // Convert the constraints to equations, creating necessary slack variables
        system.with_constraints(|cons| {
            for constraint in cons {
                if constraint.comparison == Comparison::LessThanOrEqual {
                    // When creating a new slack variable, we need to increment the counter for the
                    // next one and add it to the simplex variables set, with a starting value of the
                    // constant, since the original variables start at 0
                    let slack = VariableType::Slack(slack_counter);
                    slack_counter += 1;
                    variables.push((slack, constraint.constant));

                    // Convert the old variables from the constraint into the required type and add the
                    // slack variable for this equation
                    let variables = constraint
                        .var_expression
                        .0
                        .iter()
                        .map(|&(coeff, var)| (coeff, VariableType::Original(var)))
                        .chain(std::iter::once((1., slack)))
                        .collect();

                    // Add the equation to the vec
                    equations.push(Equation {
                        variables,
                        constant: constraint.constant,
                    });
                } else {
                    error!(
                        comparison = ?constraint.comparison,
                        %constraint,
                        "Unsupported comparison in constraint"
                    );
                    return Err(Report::msg(format!(
                        "Simplex tableaux currently only supports ≤ inequalities: {constraint:?}",
                    )));
                }
            }
            Ok(())
        })?;

        debug!(?equations);

        let column_labels = variables
            .iter()
            .map(|&(var, _)| var.into())
            .chain(["Value".into(), "θ".into(), "Row op".into()].into_iter())
            .collect();

        // Each row has n + 3 columns, where n is the number of variables. We have a column for each
        // variable, a column for the value, a column for theta, and a column for the row operation
        let rows = variables
            .iter()
            // Filter the variables to just the slacks. These are the basic variables at the start
            .filter_map(|&(var, _)| match var {
                VariableType::Slack(_) => Some(RowLabel::Variable(var)),
                VariableType::Original(_) => None
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
                                            .unwrap_or(0.)
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
                                            Some(-coeff)
                                        } else {
                                            None
                                        }
                                    })
                                    .unwrap_or(0.)
                            })
                            // And add the value
                            .chain(iter::once(0.))
                            .collect()
                    })
                ))
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
    fn theta_column(&self) -> Vec<Option<f32>> {
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
        self.bottom_row().1.iter().any(|&n| match n {
            TableauNumber::Simple(n) => n < 0.,
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
            .filter_map(|(idx, num)| match num {
                TableauNumber::Simple(n) if *n < 0. => Some((idx, *n)),
                _ => None,
            })
            .fold((0, 0.), |(acc_idx, acc_min), (this_idx, this_num)| {
                if this_num < acc_min {
                    (this_idx, this_num)
                } else {
                    (acc_idx, acc_min)
                }
            })
            .0
    }

    /// Return the index of the pivot row. This is calculated by finding the smallest positive
    /// theta value.
    fn find_pivot_row(&self) -> usize {
        self.theta_column()
            .iter()
            .enumerate()
            .filter_map(|(idx, &theta)| match theta {
                Some(n) if n > 0. => Some((idx, n)),
                _ => None,
            })
            .fold(
                (0, f32::INFINITY),
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
                RowLabel::ObjectiveFunction => (),
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
            .expect("The pivot column should have a variable lable");
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
                nums[self.row_ops_idx] = if row_op_coeff != 0. {
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
                            *n += multiplier * other_num.simple_num();
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
    pub fn do_iteration(&mut self) {
        self.populate_theta_values();
        info!(%self, "After populating theta values");

        self.change_pivot_row_label();
        self.populate_row_ops();
        info!(%self, "After populating row ops and changing pivot row label");

        self.perform_row_ops();
        info!(%self, "After performing row ops");
    }

    pub fn get_solution(self) -> SolutionSet<'v> {
        if self.negatives_in_bottom_row() {
            panic!("There must not be negatives in the bottom row when getting the solution");
        }

        // Find the value of the objective function.
        let objective_function_value = *self
            .rows
            .iter()
            .find(|&(label, _)| matches!(label, RowLabel::ObjectiveFunction))
            .expect("The objective function must have a value")
            .1[self.value_idx]
            .simple_num();

        let variable_values = self
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
                        .unwrap_or(0.),
                )
            })
            .collect();

        SolutionSet {
            objective_function_value,
            variable_values,
        }
    }
}
