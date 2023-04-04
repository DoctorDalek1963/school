//! This module deals with comparison operators, as used in inequalities.

use nom::{branch::alt, bytes::complete::tag, IResult, Parser};
use std::fmt;

/// Comparison operators.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Comparison {
    /// Less than (<).
    LessThan,

    /// Less than or equal (≤ or <=).
    LessThanOrEqual,

    /// Equal (=).
    Equal,

    /// Greater than (>).
    GreaterThan,

    /// Greater than or equal (≥ or >=).
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

    /// Compare the given values with the comparison operator.
    pub fn compare<T: PartialOrd>(&self, lhs: &T, rhs: &T) -> bool {
        match self {
            Self::LessThan => lhs.lt(rhs),
            Self::LessThanOrEqual => lhs.le(rhs),
            Self::Equal => lhs.eq(rhs),
            Self::GreaterThan => lhs.gt(rhs),
            Self::GreaterThanOrEqual => lhs.ge(rhs),
        }
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
