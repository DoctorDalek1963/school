//! This module handles parsing and using expressions.

use super::Variables;
use crate::lin_prog::_VARIABLE_REGEX_INTERNAL;
use color_eyre::{Report, Result};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, multispace0},
    error::ParseError,
    multi::separated_list1,
    number::complete::float,
    sequence::delimited,
};
use nom_regex::str::re_find;
use regex::Regex;
use std::fmt;

/// An expression written as a series of variables with coefficients. There are no constants.
///
/// The string slices should reference a [`Variables`] instance.
#[derive(Clone, Debug, PartialEq)]
pub struct Expression<'v>(Vec<(f32, &'v str)>);

impl<'v> fmt::Display for Expression<'v> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|&(coeff, var)| format!("{coeff}{var}"))
                .join(" + ")
        )
    }
}

/// This custom error type allows me to propagate undefined variable errors up through the
/// expression parser without having to abuse nom's own error types.
#[derive(Debug, thiserror::Error, PartialEq)]
pub(crate) enum ExpressionCustomError<'s, E> {
    #[error("nom internal error")]
    NomError(nom::Err<E>),

    #[error("undefined variable name {0:?}")]
    UndefinedVariable(&'s str),

    #[error("bad punctuation {0:?}")]
    BadPunctuation(String),
}

// Convert from a nom error to mine using `?`.
impl<'s, E> From<nom::Err<E>> for ExpressionCustomError<'s, E> {
    fn from(value: nom::Err<E>) -> Self {
        Self::NomError(value)
    }
}

// Allow my error to be used inside nom's own error type. I need to wrap it in nom's error type to
// allow it be used like an error type in [`IResult`].
impl<'s, I> ParseError<I> for ExpressionCustomError<'s, nom::error::Error<I>> {
    fn from_error_kind(input: I, code: nom::error::ErrorKind) -> Self {
        Self::NomError(nom::Err::Error(nom::error::Error { input, code }))
    }

    fn append(input: I, kind: nom::error::ErrorKind, other: Self) -> Self {
        // I don't know what to do here, so I'm just gonna return the other error.
        other
    }
}

impl<'v> Expression<'v> {
    pub(crate) fn nom_parse<'i: 'v>(
        input: &'i str,
        vars: &'v Variables,
    ) -> Result<
        (&'i str, Vec<(f32, &'v str)>),
        nom::Err<ExpressionCustomError<'i, nom::error::Error<&'i str>>>,
    > {
        let regex_disallowed_chars = Regex::new(r"[^a-zA-Z0-9.\s_+-]").unwrap();

        if let Ok((_, punctuation)) =
            re_find::<'i, nom::error::Error<&'i str>>(regex_disallowed_chars.clone())(input)
        {
            return Err(nom::Err::Failure(ExpressionCustomError::BadPunctuation(
                punctuation.to_string(),
            )));
        }

        separated_list1(
            delimited(multispace0, tag("+"), multispace0),
            // This closure parses a single term
            move |input| -> Result<
                (&'i str, (f32, &'v str)),
                nom::Err<ExpressionCustomError<'i, nom::error::Error<&'i str>>>,
            > {
                // If we've got any unconsumed punctuation at this point, then it's bad punctuation
                if let Ok((_, punctuation)) =
                    char::<&'i str, nom::error::Error<&'i str>>('+')(input)
                {
                    return Err(nom::Err::Failure(ExpressionCustomError::BadPunctuation(
                        punctuation.to_string(),
                    )));
                }

                let (input, coeff) = match float(input) {
                    Ok((i, c)) => (i, c),

                    // No float found, so assume it's a 1
                    Err(nom::Err::Error(nom::error::Error {
                        input: _,
                        code: nom::error::ErrorKind::Float,
                    })) => (input, 1.),

                    // In the case of a different error, just wrap and propagate
                    Err(e) => return Err(nom::Err::Failure(ExpressionCustomError::NomError(e))),
                };

                // Find a variable
                let (input, var) = re_find(
                    Regex::new(&format!("^{}", _VARIABLE_REGEX_INTERNAL))
                        .unwrap()
                        .clone(),
                )(input)?;

                // Make sure the variable is valid
                if !vars.0.contains(var) {
                    return Err(nom::Err::Failure(ExpressionCustomError::UndefinedVariable(
                        var,
                    )));
                }

                Ok((input, (coeff, var)))
            },
        )(input)
    }

    /// Parse an expression from the given input, using the given set of defined variables.
    pub fn parse<'i: 'v>(input: &'i str, vars: &'v Variables) -> Result<Self> {
        let parse_result = Self::nom_parse(input, vars);

        match parse_result {
            Ok((text, terms)) => {
                if text.trim().is_empty() {
                    Ok(Self(terms))
                } else {
                    // TODO: This is fine when the whole input is the expression, but I'll need
                    // something smarter later
                    Err(Report::msg("Parser failed before finishing parsing"))
                }
            }
            Err(e) => Err(Report::msg(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expression_nom_parse_test() {
        let variables = Variables(
            ["a", "b", "c", "d", "e"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );

        assert_eq!(
            Expression::nom_parse("a+b", &variables),
            Ok(("", vec![(1., "a"), (1., "b")]))
        );
        assert_eq!(
            Expression::nom_parse("2.3a + -1.2b   +4.63c", &variables),
            Ok(("", vec![(2.3, "a"), (-1.2, "b"), (4.63, "c")]))
        );
        assert_eq!(
            Expression::nom_parse("3a+2a", &variables),
            Ok(("", vec![(3., "a"), (2., "a")]))
        );
        assert_eq!(
            Expression::nom_parse("-1.2a + 19b  ", &variables),
            Ok(("  ", vec![(-1.2, "a"), (19., "b")]))
        );

        assert!(
            matches!(
                Expression::nom_parse("", &variables),
                Err(nom::Err::Error(ExpressionCustomError::NomError(_)))
            ),
            "Empty string"
        );
        assert!(
            matches!(
                Expression::nom_parse("ab", &variables),
                Err(nom::Err::Failure(ExpressionCustomError::UndefinedVariable(
                    "ab"
                )))
            ),
            "Multiplying separate variables (read as one variable ab)"
        );
        assert!(
            matches!(
                Expression::nom_parse("a++b", &variables),
                Err(nom::Err::Failure(ExpressionCustomError::BadPunctuation(
                    punc
                ))) if punc == "+"
            ),
            "Double + symbol"
        );
        assert!(
            matches!(
                Expression::nom_parse("a*b", &variables),
                Err(nom::Err::Failure(ExpressionCustomError::BadPunctuation(
                    punc
                ))) if punc == "*"
            ),
            "* symbol"
        );
        assert!(
            matches!(
                Expression::nom_parse("a/b", &variables),
                Err(nom::Err::Failure(ExpressionCustomError::BadPunctuation(
                    punc
                ))) if punc == "/"
            ),
            "/ symbol"
        );
        assert!(
            matches!(
                Expression::nom_parse("a+b^2", &variables),
                Err(nom::Err::Failure(ExpressionCustomError::BadPunctuation(
                    punc
                ))) if punc == "^"
            ),
            "Squaring a variable"
        );
        assert!(
            matches!(
                Expression::nom_parse("dead", &variables),
                Err(nom::Err::Failure(ExpressionCustomError::UndefinedVariable(
                    "dead"
                )))
            ),
            "Multiplying several variables (another undefined variable)"
        );
        assert!(
            matches!(
                Expression::nom_parse("+", &variables),
                Err(nom::Err::Failure(ExpressionCustomError::BadPunctuation(
                    punc
                ))) if punc == "+"
            ),
            "Single plus with no variables"
        );
        assert!(
            matches!(
                Expression::nom_parse("a+z", &variables),
                Err(nom::Err::Failure(ExpressionCustomError::UndefinedVariable(
                    "z"
                )))
            ),
            "Undefined variable z"
        );
    }

    #[test]
    fn expression_parse_test() {
        let variables = Variables(
            ["a", "b", "c", "d", "e"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );

        assert_eq!(
            Expression::parse("a+b", &variables).unwrap(),
            Expression(vec![(1., "a"), (1., "b")])
        );
        assert_eq!(
            Expression::parse("2.3a + -1.2b   +4.63c", &variables).unwrap(),
            Expression(vec![(2.3, "a"), (-1.2, "b"), (4.63, "c")])
        );
        assert_eq!(
            Expression::parse("3a+2a", &variables).unwrap(),
            Expression(vec![(3., "a"), (2., "a")])
        );
        assert_eq!(
            Expression::parse("-1.2a + 19b  ", &variables).unwrap(),
            Expression(vec![(-1.2, "a"), (19., "b")])
        );

        assert!(Expression::parse("", &variables).is_err(), "Empty string");
        assert!(
            Expression::parse("ab", &variables).is_err(),
            "Multiplying separate variables"
        );
        assert!(
            Expression::parse("a++b", &variables).is_err(),
            "Double + symbol"
        );
        assert!(
            Expression::parse("a+b^2", &variables).is_err(),
            "Squaring a variable"
        );
        assert!(
            Expression::parse("dead", &variables).is_err(),
            "Multiplying several variables"
        );
        assert!(
            Expression::parse("+", &variables).is_err(),
            "Single plus with no variables"
        );
        assert_eq!(
            Expression::parse("a+z", &variables)
                .err()
                .unwrap()
                .to_string(),
            "Parsing Failure: UndefinedVariable(\"z\")",
            "Undefined variable name"
        );
    }
}
