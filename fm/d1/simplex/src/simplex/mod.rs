//! This module handles execution of the actual simplex algorithm itself.

mod tableau;
#[cfg(test)]
mod tests;

use self::tableau::Tableau;
use crate::{lin_prog::system::LinProgSystem, Frac};
use color_eyre::Result;
use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap, fmt};
use tracing::{info, instrument};

/// The different types of variables that can be used in solving linear programming problems.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum VariableType<'v> {
    /// An original variable from the [`LinProgSystem`].
    Original(&'v str),

    /// A slack variable used in simplex.
    Slack(usize),

    /// A surplus variable used in simplex.
    Surplus(usize),

    /// An artificial variable used in simplex.
    Artificial(usize),
}

impl<'v> fmt::Display for VariableType<'v> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Original(name) => write!(f, "{name}"),
            Self::Slack(num) => write!(f, "sl#{num}"),
            Self::Surplus(num) => write!(f, "su#{num}"),
            Self::Artificial(num) => write!(f, "ar#{num}"),
        }
    }
}

impl<'v> PartialOrd for VariableType<'v> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'v> Ord for VariableType<'v> {
    fn cmp(&self, other: &Self) -> Ordering {
        use VariableType::*;

        match (*self, *other) {
            (Original(a), Original(b)) => a.cmp(b),
            (Slack(a), Slack(b)) => a.cmp(&b),
            (Surplus(a), Surplus(b)) => a.cmp(&b),
            (Artificial(a), Artificial(b)) => a.cmp(&b),

            // Original < Slack < Surplus < Artificial
            (Original(_), _) => Ordering::Less,
            (_, Original(_)) => Ordering::Greater,
            (Slack(_), Surplus(_)) => Ordering::Less,
            (Slack(_), Artificial(_)) => Ordering::Less,
            (Surplus(_), Slack(_)) => Ordering::Greater,
            (Surplus(_), Artificial(_)) => Ordering::Less,
            (Artificial(_), Surplus(_)) => Ordering::Greater,
            (Artificial(_), Slack(_)) => Ordering::Greater,
        }
    }
}

/// An equation with variables on the left (including slack variables) and a constant on the right.
#[derive(Clone, Debug, PartialEq)]
pub struct Equation<'v> {
    /// The variables on the LHS. The tuples are `(coefficient, variable_name)`.
    variables: Vec<(Frac, VariableType<'v>)>,

    /// The constant that the variables are equal to.
    constant: Frac,
}

/// A solution to a linear programming problem.
#[derive(Clone, Debug, PartialEq)]
pub struct SolutionSet<'v> {
    /// The value of the objective function for the optimal point.
    objective_function_value: Frac,

    /// The values of the variables at the optimal point.
    variable_values: HashMap<VariableType<'v>, Frac>,
}

impl<'v> fmt::Display for SolutionSet<'v> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\nObjFunc# = {}", self.objective_function_value)?;
        for (var, value) in self
            .variable_values
            .iter()
            .sorted_by_key(|&(var_type, _)| var_type)
        {
            write!(f, "\n{var} = {value}")?;
        }
        Ok(())
    }
}

/// Solve the given linear programming system using simplex tableaux.
#[instrument(skip(system))]
pub fn solve_with_simplex_tableaux<'v>(system: &'v LinProgSystem) -> Result<SolutionSet<'v>> {
    let mut tableau: Tableau = Tableau::create_initial(system)?;
    info!(%tableau, "Initial tableau");
    while tableau.negatives_in_bottom_row() {
        tableau.do_iteration()?;
    }

    Ok(tableau.get_solution())
}
