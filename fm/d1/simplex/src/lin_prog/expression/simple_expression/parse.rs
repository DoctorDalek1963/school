//! This module handles parsing expressions.

use super::Expression;
use crate::{
    lin_prog::{parse_float_no_e, validate_variable, Variables, _VARIABLE_REGEX_INTERNAL},
    Frac,
};
use color_eyre::{Report, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0},
    error::ParseError,
    sequence::pair,
    IResult,
};
use nom_regex::str::re_find;
use regex::Regex;

/// This custom error type allows me to propagate undefined variable errors up through the
/// expression parser without having to abuse nom's own error types.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ExpressionCustomParseError<'s, E> {
    /// An error resulting from `nom`.
    #[error("nom internal error")]
    NomError(nom::Err<E>),

    /// An undefined variable.
    #[error("undefined variable name {0:?}")]
    UndefinedVariable(&'s str),

    /// Bad punctuation in the expression, like "a*b".
    #[error("bad punctuation {0:?}")]
    BadPunctuation(String),
}

// Convert from a nom error to mine using `?`.
impl<'s, E> From<nom::Err<E>> for ExpressionCustomParseError<'s, E> {
    fn from(value: nom::Err<E>) -> Self {
        Self::NomError(value)
    }
}

impl<'s, I> From<nom::error::Error<I>> for ExpressionCustomParseError<'s, nom::error::Error<I>> {
    fn from(value: nom::error::Error<I>) -> Self {
        Self::NomError(nom::Err::Error(value))
    }
}

// Allow my error to be used inside nom's own error type. I need to wrap it in nom's error type to
// allow it be used like an error type in [`IResult`].
impl<'s, I> ParseError<I> for ExpressionCustomParseError<'s, nom::error::Error<I>> {
    fn from_error_kind(input: I, code: nom::error::ErrorKind) -> Self {
        Self::NomError(nom::Err::Error(nom::error::Error { input, code }))
    }

    fn append(_input: I, _kind: nom::error::ErrorKind, other: Self) -> Self {
        // I don't know what to do here, so I'm just gonna return the other error.
        other
    }
}

impl<'v> Expression<'v> {
    /// Parse an expression from the input using `nom`.
    pub(crate) fn nom_parse<'i>(
        input: &'i str,
        vars: &'v Variables,
    ) -> Result<(&'i str, Self), nom::Err<ExpressionCustomParseError<'i, nom::error::Error<&'i str>>>>
    {
        let regex_disallowed_chars = Regex::new(r"[^a-zA-Z0-9.\s_<>=≤≥+-]").unwrap();

        if let Ok((_, punctuation)) =
            re_find::<'i, nom::error::Error<&'i str>>(regex_disallowed_chars)(input)
        {
            return Err(nom::Err::Failure(
                ExpressionCustomParseError::BadPunctuation(punctuation.to_string()),
            ));
        }

        let (input, expressions) = custom_separated_list1(
            |input| -> Result<
                (&str, ()),
                nom::Err<ExpressionCustomParseError<'i, nom::error::Error<&'i str>>>,
            > {
                let (input, _) = multispace0(input)?;
                let (input2, plus_minus) = alt((tag("+"), tag("-")))(input)?;
                match plus_minus {
                    "+" => {
                        let (input, _) = multispace0(input2)?;
                        Ok((input, ()))
                    }
                    "-" => Ok((input, ())),
                    _ => unreachable!("Only + or - should be matched by the parser"),
                }
            },
            // This closure parses a single term
            move |input| -> Result<
                (&'i str, (Frac, &'v str)),
                nom::Err<ExpressionCustomParseError<'i, nom::error::Error<&'i str>>>,
            > {
                // If we've got any unconsumed punctuation at this point, then it's bad punctuation
                if let Ok((_, punctuation)) =
                    char::<&'i str, nom::error::Error<&'i str>>('+')(input)
                {
                    return Err(nom::Err::Failure(
                        ExpressionCustomParseError::BadPunctuation(punctuation.to_string()),
                    ));
                }

                let (input, coeff) = match parse_float(input) {
                    Ok((i, c)) => (i, c),

                    // No float found, so assume it's a 1
                    Err(nom::Err::Error(nom::error::Error {
                        input: _,
                        code: nom::error::ErrorKind::Float | nom::error::ErrorKind::Char,
                    })) => (input, 1.),

                    // In the case of a different error, just wrap and propagate
                    Err(e) => {
                        return Err(nom::Err::Failure(ExpressionCustomParseError::NomError(e)))
                    }
                };

                // Find a variable
                let (input, var) = re_find(
                    Regex::new(&format!(r"^\s*{_VARIABLE_REGEX_INTERNAL}")).unwrap(),
                )(input)?;
                let var = validate_variable(var).map_err(|_| {
                    nom::Err::Failure(ExpressionCustomParseError::UndefinedVariable(var))
                })?;

                // Make sure the variable is valid
                match vars.0.get(var) {
                    Some(v) => Ok((input, (coeff.into(), v))),
                    None => Err(nom::Err::Failure(
                        ExpressionCustomParseError::UndefinedVariable(var),
                    )),
                }
            },
        )(input)?;

        Ok((input, Expression(expressions)))
    }

    /// Parse an expression from the given input, using the given set of defined variables.
    pub fn parse<'i>(input: &'i str, vars: &'v Variables) -> Result<Self> {
        let parse_result = Self::nom_parse(input, vars);

        match parse_result {
            Ok((text, exp)) => {
                if text.trim().is_empty() {
                    Ok(exp)
                } else {
                    // TODO: This is fine when the whole input is the expression, but I'll need
                    // something smarter later
                    Err(Report::msg(concat!(
                        "Parser failed before finishing parsing; ",
                        "use nom_parse() to parse incrementally"
                    )))
                }
            }
            Err(e) => Err(Report::msg(e.to_string())),
        }
    }
}

/// Parse a float as part of an expression, allowing for whitespace between `-` and the number.
fn parse_float(input: &str) -> IResult<&str, f32> {
    let (input, _) = multispace0(input)?;
    let (input, negative) =
        match pair(tag::<&str, &str, nom::error::Error<&str>>("-"), multispace0)(input) {
            Ok((new_input, (_negative, _space))) => (new_input, true),
            Err(_) => (input, false),
        };
    let (input, _) = multispace0(input)?;
    let (input, num) = parse_float_no_e(input)?;
    Ok((input, if negative { num * -1. } else { num }))
}

/// A custom version of [`nom::multi_separated_list1()`] which allows instances where the `sep`
/// parser consumes nothing.
fn custom_separated_list1<I, O, O2, E, F, G>(
    mut sep: G,
    mut f: F,
) -> impl FnMut(I) -> IResult<I, Vec<O>, E>
where
    I: Clone + nom::InputLength,
    F: nom::Parser<I, O, E>,
    G: nom::Parser<I, O2, E>,
    E: ParseError<I>,
{
    move |mut i: I| {
        let mut res = Vec::new();

        // Parse the first element
        match f.parse(i.clone()) {
            Err(e) => return Err(e),
            Ok((i1, o)) => {
                res.push(o);
                i = i1;
            }
        }

        loop {
            // let len = i.input_len();
            match sep.parse(i.clone()) {
                Err(nom::Err::Error(_)) => return Ok((i, res)),
                Err(e) => return Err(e),
                Ok((i1, _)) => {
                    // It's okay in this particular instance for the sep parser to consume nothing

                    // infinite loop check: the parser must always consume
                    //if i1.input_len() == len {
                    //return Err(Err::Error(E::from_error_kind(i1, ErrorKind::SeparatedList)));
                    //}

                    match f.parse(i1.clone()) {
                        Err(nom::Err::Error(_)) => return Ok((i, res)),
                        Err(e) => return Err(e),
                        Ok((i2, o)) => {
                            res.push(o);
                            i = i2;
                        }
                    }
                }
            }
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
            Ok(("", Expression(vec![(1.into(), "a"), (1.into(), "b")])))
        );
        assert_eq!(
            Expression::nom_parse("2.3a + -1.2b   +4.63c", &variables),
            Ok((
                "",
                Expression(vec![
                    (Frac::new(23u32, 10u32), "a"),
                    (-Frac::new(12u32, 10u32), "b"),
                    (Frac::new(463u32, 100u32), "c")
                ])
            ))
        );
        assert_eq!(
            Expression::nom_parse("3a+2a", &variables),
            Ok(("", Expression(vec![(3.into(), "a"), (2.into(), "a")])))
        );
        assert_eq!(
            Expression::nom_parse("-1.2a + 19b  ", &variables),
            Ok((
                "  ",
                Expression(vec![(-Frac::new(6u32, 5u32), "a"), (19.into(), "b")])
            ))
        );
        assert_eq!(
            Expression::nom_parse("2e + 3e - 1 e", &variables),
            Ok((
                "",
                Expression(vec![
                    (2.into(), "e"),
                    (3.into(), "e"),
                    (-Frac::new(1u32, 1u32), "e")
                ])
            ))
        );
        assert_eq!(
            Expression::nom_parse("2a-b", &variables),
            Ok((
                "",
                Expression(vec![(2.into(), "a"), (-Frac::new(1u32, 1u32), "b")])
            ))
        );

        assert!(
            matches!(
                Expression::nom_parse("", &variables),
                Err(nom::Err::Error(ExpressionCustomParseError::NomError(_)))
            ),
            "Empty string"
        );
        assert!(
            matches!(
                Expression::nom_parse("ab", &variables),
                Err(nom::Err::Failure(
                    ExpressionCustomParseError::UndefinedVariable("ab")
                ))
            ),
            "Multiplying separate variables (read as one variable ab)"
        );
        assert!(
            matches!(
                Expression::nom_parse("a++b", &variables),
                Err(nom::Err::Failure(ExpressionCustomParseError::BadPunctuation(
                    punc
                ))) if punc == "+"
            ),
            "Double + symbol"
        );
        assert!(
            matches!(
                Expression::nom_parse("a*b", &variables),
                Err(nom::Err::Failure(ExpressionCustomParseError::BadPunctuation(
                    punc
                ))) if punc == "*"
            ),
            "* symbol"
        );
        assert!(
            matches!(
                Expression::nom_parse("a/b", &variables),
                Err(nom::Err::Failure(ExpressionCustomParseError::BadPunctuation(
                    punc
                ))) if punc == "/"
            ),
            "/ symbol"
        );
        assert!(
            matches!(
                Expression::nom_parse("a+b^2", &variables),
                Err(nom::Err::Failure(ExpressionCustomParseError::BadPunctuation(
                    punc
                ))) if punc == "^"
            ),
            "Squaring a variable"
        );
        assert!(
            matches!(
                Expression::nom_parse("dead", &variables),
                Err(nom::Err::Failure(
                    ExpressionCustomParseError::UndefinedVariable("dead")
                ))
            ),
            "Multiplying several variables (another undefined variable)"
        );
        assert!(
            matches!(
                Expression::nom_parse("+", &variables),
                Err(nom::Err::Failure(ExpressionCustomParseError::BadPunctuation(
                    punc
                ))) if punc == "+"
            ),
            "Single plus with no variables"
        );
        assert!(
            matches!(
                Expression::nom_parse("a+z", &variables),
                Err(nom::Err::Failure(
                    ExpressionCustomParseError::UndefinedVariable("z")
                ))
            ),
            "Undefined variable z"
        );
    }
}
