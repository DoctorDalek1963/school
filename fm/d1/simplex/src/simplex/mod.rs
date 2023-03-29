//! This module handles execution of the actual simplex algorithm itself.

#[cfg(test)]
mod tests;

use crate::lin_prog::{comparison::Comparison, system::LinProgSystem};
use color_eyre::{Report, Result};
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

    todo!()
}
