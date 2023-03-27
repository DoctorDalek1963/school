//! This module deals with comparison operators, as used in inequalities.

use std::fmt;

use nom::{branch::alt, bytes::complete::tag, IResult, Parser};

/// Comparison operators.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Comparison {
    LessThan,
    LessThanOrEqual,
    Equal,
    GreaterThan,
    GreaterThanOrEqual,
}

impl fmt::Display for Comparison {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::LessThan => "<",
                Self::LessThanOrEqual => "≤",
                Self::Equal => "=",
                Self::GreaterThan => ">",
                Self::GreaterThanOrEqual => "≥",
            }
        )
    }
}

impl Comparison {
    /// Parse a comparison operator with nom.
    pub fn nom_parse(input: &str) -> IResult<&str, Self> {
        alt((
            tag("=").map(|_| Self::Equal),
            tag("≤").map(|_| Self::LessThanOrEqual),
            tag("≥").map(|_| Self::GreaterThanOrEqual),
            tag("<=").map(|_| Self::LessThanOrEqual),
            tag("<").map(|_| Self::LessThan),
            tag(">=").map(|_| Self::GreaterThanOrEqual),
            tag(">").map(|_| Self::GreaterThan),
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comparison_parse_test() {
        assert_eq!(Comparison::nom_parse("="), Ok(("", Comparison::Equal)));
        assert_eq!(
            Comparison::nom_parse("<="),
            Ok(("", Comparison::LessThanOrEqual))
        );
        assert_eq!(
            Comparison::nom_parse("≥ 4"),
            Ok((" 4", Comparison::GreaterThanOrEqual))
        );
        assert_eq!(
            Comparison::nom_parse("> 3.2"),
            Ok((" 3.2", Comparison::GreaterThan))
        );
        assert_eq!(
            Comparison::nom_parse("< 12"),
            Ok((" 12", Comparison::LessThan))
        );
        assert_eq!(
            Comparison::nom_parse("=< 10"),
            Ok(("< 10", Comparison::Equal))
        );
    }
}
