//! This module handles constraints, which express how variable expressions relate to contants.

use super::{
    expression::ExpressionCustomError, parse_float_no_e, Comparison, Expression, Variables,
};
use nom::character::complete::multispace0;
use std::fmt;

/// A constraint in terms of variables, a comparison operator, and a constant.
#[derive(Clone, Debug, PartialEq)]
pub struct Constraint<'v> {
    /// The LHS expression in terms of the variables.
    pub var_expression: Expression<'v>,

    /// The comparison operator.
    pub comparison: Comparison,

    /// The constant to compare to.
    pub constant: f32,
}

impl<'v> fmt::Display for Constraint<'v> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.var_expression, self.comparison, self.constant
        )
    }
}

impl<'v> Constraint<'v> {
    /// Parse a constraint from the input using `nom`.
    pub fn nom_parse<'i>(
        input: &'i str,
        vars: &'v Variables,
    ) -> Result<(&'i str, Self), nom::Err<ExpressionCustomError<'i, nom::error::Error<&'i str>>>>
    {
        let (input, var_expression) = Expression::nom_parse(input, vars)?;
        let (input, _) = multispace0(input)?;
        let (input, comparison) = match Comparison::nom_parse(input) {
            Ok(x) => Ok(x),
            Err(e) => Err(nom::Err::Error(ExpressionCustomError::NomError(e))),
        }?;
        let (input, _) = multispace0(input)?;
        let (input, constant) = match parse_float_no_e(input) {
            Ok(x) => Ok(x),
            Err(e) => Err(nom::Err::Error(ExpressionCustomError::NomError(e))),
        }?;

        Ok((
            input,
            Constraint {
                var_expression,
                comparison,
                constant,
            },
        ))
    }

    /// Simplify the constraint.
    pub fn simplify(self) -> Self {
        Self {
            var_expression: self.var_expression.simplify(),
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constraint_parse_test() {
        let variables = Variables(
            ["a", "b", "c", "d", "e"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );

        assert_eq!(
            Constraint::nom_parse("2a + 3b <= 10", &variables),
            Ok((
                "",
                Constraint {
                    var_expression: Expression(vec![(2., "a"), (3., "b")]),
                    comparison: Comparison::LessThanOrEqual,
                    constant: 10.
                }
            ))
        );

        assert_eq!(
            Constraint::nom_parse("2a + 3b + -1.4c <= 15", &variables),
            Ok((
                "",
                Constraint {
                    var_expression: Expression(vec![(2., "a"), (3., "b"), (-1.4, "c")]),
                    comparison: Comparison::LessThanOrEqual,
                    constant: 15.
                }
            ))
        );

        assert_eq!(
            Constraint::nom_parse("2a + 3b  -  13.25c>= 196.0", &variables),
            Ok((
                "",
                Constraint {
                    var_expression: Expression(vec![(2., "a"), (3., "b"), (-13.25, "c")]),
                    comparison: Comparison::GreaterThanOrEqual,
                    constant: 196.
                }
            ))
        );

        assert_eq!(
            Constraint::nom_parse("2e + 3e - 1 e > -15", &variables),
            Ok((
                "",
                Constraint {
                    var_expression: Expression(vec![(2., "e"), (3., "e"), (-1., "e")]),
                    comparison: Comparison::GreaterThan,
                    constant: -15.
                }
            ))
        );

        assert_eq!(
            Constraint::nom_parse("-14a + 13d + 2a ≤ 1", &variables),
            Ok((
                "",
                Constraint {
                    var_expression: Expression(vec![(-14., "a"), (13., "d"), (2., "a")]),
                    comparison: Comparison::LessThanOrEqual,
                    constant: 1.
                }
            ))
        );

        assert_eq!(
            Constraint::nom_parse("2a + 3b <= 10", &variables),
            Ok((
                "",
                Constraint {
                    var_expression: Expression(vec![(2., "a"), (3., "b")]),
                    comparison: Comparison::LessThanOrEqual,
                    constant: 10.
                }
            ))
        );
    }
}
