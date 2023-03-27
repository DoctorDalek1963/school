//! This module handles linear programming systems.

mod comparison;
mod constraint;
mod expression;

use self::{comparison::Comparison, constraint::Constraint, expression::Expression};
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
#[derive(Clone, Debug, PartialEq)]
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

fn parse_float_no_e(input: &str) -> nom::IResult<&str, f32> {
    use nom::{
        branch::alt,
        character::complete::{char, digit1},
        combinator::{map, opt, recognize},
        sequence::{pair, tuple},
        ParseTo,
    };

    let (input, num) = recognize(tuple((
        opt(alt((char('+'), char('-')))),
        alt((
            map(tuple((digit1, opt(pair(char('.'), opt(digit1))))), |_| ()),
            map(tuple((char('.'), digit1)), |_| ()),
        )),
    )))(input)?;

    match num.parse_to() {
        Some(f) => Ok((input, f)),
        None => Err(nom::Err::Error(nom::error::Error {
            input,
            code: nom::error::ErrorKind::Float,
        })),
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
    }
}
