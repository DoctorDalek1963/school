//! This module handles the tableaux. Tableau is singular; tableaux is plural.

use super::{Equation, VariableType};
use crate::lin_prog::{comparison::Comparison, system::LinProgSystem};
use color_eyre::{Report, Result};
use itertools::Itertools;
use std::{fmt, iter};
use tabled::{builder::Builder, Style};
use tracing::{debug, error, instrument};

/// A label to use for a row in the tableau.
#[derive(Clone, Debug, PartialEq)]
enum RowLabel<'v> {
    /// A variable. See [`VariableType`].
    Variable(VariableType<'v>),

    /// The objective function.
    ObjectiveFunction,
}

impl<'v> fmt::Display for RowLabel<'v> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Variable(var) => write!(f, "{var}"),
            Self::ObjectiveFunction => write!(f, "ObjFunc#"),
        }
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
}

impl fmt::Display for TableauNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Simple(n) => n.to_string(),
            Self::Theta(Some(n)) => n.to_string(),
            Self::Theta(None) => String::new(),
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
    column_labels: Vec<String>,

    /// The rows of the table.
    rows: Vec<(RowLabel<'v>, Vec<TableauNumber>)>,

    /// The index of the value column.
    value_idx: usize,

    /// The index of the theta column.
    theta_idx: usize,
}

impl<'v> fmt::Display for Tableau<'v> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = Builder::default();
        builder.add_record(
            iter::once("Basic var".to_string())
                .chain(self.column_labels.iter().map(|s| s.to_owned())),
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
            .map(|&(var, _)| var.to_string())
            .chain(["Value".to_string(), "θ".to_string()].into_iter())
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
            // TODO: Add theta and row operations
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
                        .map(|n| TableauNumber::Simple(n))
                        .chain(iter::once(TableauNumber::Theta(None)))
                        .collect()
                ))
            .collect();

        let value_idx = variables.len();
        let theta_idx = value_idx + 1;

        Ok(Self {
            column_labels,
            rows,
            value_idx,
            theta_idx,
        })
    }

    /// Return a reference to the bottom row of the table.
    fn bottom_row(&self) -> &(RowLabel<'v>, Vec<TableauNumber>) {
        self.rows.last().expect("There should be a bottom row")
    }

    /// Check if there are any negative numbers in the bottom row of the tableau.
    fn negatives_in_bottom_row(&self) -> bool {
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
            .fold(
                (0, 0.),
                |(acc_idx, acc_min), (this_idx, &this_num)| match this_num {
                    TableauNumber::Simple(n) => {
                        if n < acc_min {
                            (this_idx, n)
                        } else {
                            (acc_idx, acc_min)
                        }
                    }
                    _ => (acc_idx, acc_min),
                },
            )
            .0
    }

    /// Populate this tableau with theta values.
    pub fn populate_theta_values(&mut self) {
        let pivot_col = self.find_pivot_column();
        for (label, numbers) in self.rows.iter_mut() {
            match label {
                RowLabel::Variable(_) => {
                    numbers[self.theta_idx] = TableauNumber::Theta(Some(
                        numbers[self.value_idx].simple_num() / numbers[pivot_col].simple_num(),
                    ))
                }
                RowLabel::ObjectiveFunction => (),
            }
        }
    }
}
