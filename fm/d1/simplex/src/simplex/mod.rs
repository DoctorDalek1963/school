//! This module handles execution of the actual simplex algorithm itself.

#[cfg(test)]
mod tests;

use crate::lin_prog::{comparison::Comparison, system::LinProgSystem};
use color_eyre::{Report, Result};
use itertools::Itertools;
use std::collections::HashMap;
use tracing::{debug, error, instrument};

/// The different types of variables that can be used in solving linear programming problems.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum VariableType<'v> {
    /// An original variable from the [`LinProgSystem`].
    Original(&'v str),

    /// A slack variable used in simplex.
    Slack(usize),
}

/// An equation with variables on the left (including slack variables) and a constant on the right.
#[derive(Clone, Debug, PartialEq)]
struct Equation<'v> {
    /// The variables on the LHS. The tuples are `(coefficient, variable_name)`.
    variables: Vec<(f32, VariableType<'v>)>,

    /// The constant that the variables are equal to.
    constant: f32,
}

/// A solution to a linear programming problem.
#[derive(Clone, Debug, PartialEq)]
pub struct SolutionSet<'v> {
    /// The value of the objective function for the optimal point.
    objective_function_value: f32,

    /// The values of the variables at the optimal point.
    variable_values: HashMap<VariableType<'v>, f32>,
}

/// A label to use for a row in the tableau.
#[derive(Clone, Debug, PartialEq)]
enum RowLabel<'v> {
    /// A variable. See [`VariableType`].
    Variable(VariableType<'v>),

    /// The objective function.
    ObjectiveFunction,
}

/// Solve the given linear programming system using simplex tableaux.
#[instrument(skip(system))]
pub fn solve_with_simplex_tableaux<'v>(system: &'v LinProgSystem) -> Result<SolutionSet<'v>> {
    let mut slack_counter = 0;

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

    let mut equations = vec![];

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
                    .chain([(1., slack)].into_iter())
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

    debug!(?variables, ?equations);

    // Each row has n + 3 columns, where n is the number of variables. We have a column for each
    // variable, a column for the value, a column for theta, and a column for the row operation
    let initial_rows: Vec<(RowLabel, Vec<f32>)> = variables
        .iter()
        // Filter the variables to just the slacks. These are the basic variables at the start
        .filter_map(|&(var, _)| match var {
            VariableType::Slack(_) => Some(RowLabel::Variable(var)),
            _ => None
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
            [(
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
                        .chain([0.].into_iter())
                        .collect()
                })
            )]
            .into_iter()
        )
        .collect();

    debug!(?initial_rows);

    todo!()
}
