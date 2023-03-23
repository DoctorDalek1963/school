//! This module handles linear programming systems.

mod expression;

use self::expression::Expression;
use color_eyre::{Report, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

/// The internal representation of the variable RegEx. This string is used in multiple RegExes, so
/// I factored it out to here.
#[doc(hidden)]
pub(crate) const _VARIABLE_REGEX_INTERNAL: &str = "[a-zA-Z][a-zA-Z0-9_]*";

lazy_static! {
    /// The RegEx used to validate variables. See [`validate_variable`].
    static ref VARIABLE_REGEX_ANCHORED: Regex = Regex::new(&format!("^{}$", _VARIABLE_REGEX_INTERNAL)).unwrap();
}

/// A collection of named variables.
pub struct Variables(HashSet<String>);

/// Validate the given variable by trimming it and checking it against the [`VARIABLE_REGEX`].
fn validate_variable(var: &str) -> Result<&str> {
    let var = var.trim();
    if VARIABLE_REGEX_ANCHORED.is_match(var) {
        Ok(var)
    } else {
        Err(Report::msg(format!("Invalid variable name {:?}", var)))
    }
}

/// The objective function for the [`LinProgSystem`].
enum ObjectiveFunction<'v> {
    /// Minimise the expression.
    Minimise(Expression<'v>),

    /// Maximise the expression.
    Maximise(Expression<'v>),
}

/// Comparison operators.
enum Comparison {
    LessThan,
    LessThanOrEqual,
    Equal,
    GreaterThan,
    GreaterThanOrEqual,
}

/// A constraint in terms of variables, a comparison operator, and a constant.
struct Constraint<'v> {
    /// The LHS expression in terms of the variables.
    var_expression: Expression<'v>,

    /// The comparison operator.
    comparison: Comparison,

    /// The constant to compare to.
    constant: f32,
}

/// A linear programming system, with a set of variables, objective function, and a set of contraints.
pub struct LinProgSystem<'v> {
    /// The variable set for the system. Every variable must be listed here for validation.
    variables: Variables,

    /// The objective function - to maximise or minimise a given expression.
    objective_function: ObjectiveFunction<'v>,

    /// The constraints to optimise for.
    constraints: Vec<Constraint<'v>>,
}

impl<'v> LinProgSystem<'v> {
    pub fn build_from_user() -> Result<Self> {
        use inquire::Text;

        let variables: HashSet<String> =
            Text::new("Please enter all your named variables, separated by spaces:")
                .prompt()?
                .split(" ")
                .filter(|&s| !s.is_empty())
                .map(|var| validate_variable(var).map(|s| s.to_string()))
                .collect::<Result<HashSet<String>>>()?;

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_variable_test() {
        assert_eq!(validate_variable("a").unwrap(), "a");
        assert_eq!(validate_variable(" xA").unwrap(), "xA");
        assert_eq!(validate_variable("x2 ").unwrap(), "x2");
        assert_eq!(validate_variable(" x_1 ").unwrap(), "x_1");
        assert_eq!(validate_variable("  fruit_cakes ").unwrap(), "fruit_cakes");

        assert!(validate_variable("").is_err());
        assert!(validate_variable("John Smith").is_err());
        assert!(validate_variable("bad variable name").is_err());
        assert!(validate_variable("@").is_err());
    }
}
