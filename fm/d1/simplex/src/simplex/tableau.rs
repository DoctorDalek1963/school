//! This module handles the tableaux. Tableau is singular; tableaux is plural.

use super::{Equation, VariableType};
use crate::lin_prog::system::LinProgSystem;
use std::{fmt, iter};
use tabled::{builder::Builder, Style};

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
    pub fn create_initial(
        system: &'v LinProgSystem,
        variables: &'v Vec<(VariableType, f32)>,
        equations: &'v Vec<Equation>,
    ) -> Self {
        let column_labels = variables
            .iter()
            .map(|&(var, _)| var.to_string())
            .chain(iter::once("Value".to_string()))
            .collect();

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

        Self {
            column_labels,
            rows,
        }
    }
}
