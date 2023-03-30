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

/// A single tableau for simplex tableaux.
#[derive(Clone, Debug, PartialEq)]
pub struct Tableau<'v> {
    /// The titles of the columns.
    column_labels: Vec<String>,

    /// The rows of the table.
    rows: Vec<(RowLabel<'v>, Vec<f32>)>,
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
    #[instrument]
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
                    error!(comparison = ?constraint.comparison, %constraint, "Unsupported comparison in constraint");
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
            .chain(iter::once("Value".to_string()))
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
            .collect();

        Ok(Self {
            column_labels,
            rows,
        })
    }
}
