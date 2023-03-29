//! This module handles linear programming systems.

pub mod comparison;
pub mod config;
pub mod constraint;
pub mod expression;
pub mod system;

use self::{comparison::Comparison, expression::Expression};
use color_eyre::{Report, Result};
use inquire::Select;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashSet, fmt};
use tracing::instrument;

/// The internal representation of the variable RegEx. This string is used in multiple RegExes, so
/// I factored it out to here.
#[doc(hidden)]
pub const _VARIABLE_REGEX_INTERNAL: &str = "[a-zA-Z][a-zA-Z0-9_]*";

lazy_static! {
    /// The RegEx used to validate variables. See [`validate_variable`].
    static ref VARIABLE_REGEX_ANCHORED: Regex = Regex::new(&format!("^{_VARIABLE_REGEX_INTERNAL}$")).unwrap();
}

/// A collection of named variables.
#[derive(Clone, Debug, PartialEq)]
pub struct Variables(HashSet<String>);

/// Validate the given variable by trimming it and checking it against the [`VARIABLE_REGEX`].
fn validate_variable(var: &str) -> Result<&str> {
    let var = var.trim();
    if VARIABLE_REGEX_ANCHORED.is_match(var) {
        Ok(var)
    } else {
        Err(Report::msg(format!("Invalid variable name {var:?}")))
    }
}

/// The objective function for the [`LinProgSystem`].
#[derive(Clone, Debug, PartialEq)]
enum ObjectiveFunction<'v> {
    /// Minimise the expression.
    Minimise(Expression<'v>),

    /// Maximise the expression.
    Maximise(Expression<'v>),
}

impl<'v> fmt::Display for ObjectiveFunction<'v> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (word, expression) = match self {
            ObjectiveFunction::Minimise(exp) => ("Minimise", exp),
            ObjectiveFunction::Maximise(exp) => ("Maximise", exp),
        };
        write!(f, "{word} {expression}")
    }
}

impl<'v> ObjectiveFunction<'v> {
    /// Build an objective function from user input using `inquire`.
    #[instrument]
    pub fn build_from_user(variables: &'v Variables) -> Result<Self> {
        let min_max = Select::new(
            "Please select a type of objective function:",
            vec!["Minimise", "Maximise"],
        )
        .prompt()
        .expect("inquire::Select should not fail");

        let expression = Expression::build_from_user(
            &format!("Please enter the expression to {}:", min_max.to_lowercase()),
            variables,
        )?;

        Ok(match min_max {
            "Minimise" => Self::Minimise(expression),
            "Maximise" => Self::Maximise(expression),
            _ => unreachable!("Selected text should only be 'Minimise' or 'Maximise'"),
        })
    }

    /// Simplify the objective function.
    pub fn simplify(self) -> Self {
        match self {
            Self::Minimise(exp) => Self::Minimise(exp.simplify()),
            Self::Maximise(exp) => Self::Maximise(exp.simplify()),
        }
    }
}

/// Parse a float without the `2.34e12` type of syntax. This function is adapted from `nom`'s
/// original float parsing system.
fn parse_float_no_e(input: &str) -> nom::IResult<&str, f32> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{char, digit1},
        combinator::{map, opt, recognize},
        sequence::{pair, tuple},
        ParseTo,
    };

    let (input, mut num) = recognize(tuple((
        opt(alt((char('+'), char('-')))),
        alt((
            map(tuple((digit1, opt(pair(char('.'), opt(digit1))))), |_| ()),
            map(tuple((char('.'), digit1)), |_| ()),
            map(tag(""), |_| ()),
        )),
    )))(input)?;

    if num == "-" {
        num = "-1";
    }

    match num.parse_to() {
        Some(f) => Ok((input, f)),
        None => Ok((input, 1.)),
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

    #[test]
    fn parse_float_no_e_test() {
        assert_eq!(parse_float_no_e("1"), Ok(("", 1.)));
        assert_eq!(parse_float_no_e("1.2 "), Ok((" ", 1.2)));
        assert_eq!(parse_float_no_e(".3d"), Ok(("d", 0.3)));
        assert_eq!(parse_float_no_e("-1"), Ok(("", -1.)));
        assert_eq!(parse_float_no_e("-2.3-"), Ok(("-", -2.3)));
        assert_eq!(parse_float_no_e("-.4"), Ok(("", -0.4)));
        assert_eq!(parse_float_no_e("-0.4"), Ok(("", -0.4)));
        assert_eq!(
            parse_float_no_e("16 other stuff"),
            Ok((" other stuff", 16.))
        );
        assert_eq!(parse_float_no_e("-"), Ok(("", -1.)));
        assert_eq!(parse_float_no_e("b"), Ok(("b", 1.)));
    }
}
