//! This module handles expressions in terms of variables with no constant terms.

use crate::{lin_prog::Variables, Frac};
use color_eyre::Result;
use fraction::Zero;
use inquire::Text;
use itertools::Itertools;
use std::{collections::HashMap, fmt};

pub(crate) mod parse;

/// An expression written as a series of variables with coefficients. There are no constants.
///
/// The string slices should reference a [`Variables`] instance.
#[derive(Clone, Debug, PartialEq)]
pub struct Expression<'v>(pub(crate) Vec<(Frac, &'v str)>);

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

impl<'v> Expression<'v> {
    /// Algebraically simplify the expression.
    pub fn simplify(self) -> Self {
        Self(
            self.0
                .into_iter()
                // Fold the values into a map
                .fold(HashMap::<&'v str, Frac>::new(), |acc, (num, var)| {
                    let mut map = acc;
                    match map.get_mut(var) {
                        Some(n) => *n += num,
                        None => {
                            map.insert(var, num);
                        }
                    };
                    map
                })
                .into_iter()
                // Swap the values in the tuple
                .map(|(var, num)| (num, var))
                // Filter out zeroes
                .filter(|&(num, _)| num != Frac::zero())
                // Sort them by variable name for consistency
                .sorted_by_key(|&(_, var)| var)
                .collect(),
        )
    }

    /// Evaluate the expression for the given variables.
    pub(crate) fn evaluate(&self, vars: &[(&'v str, Frac)]) -> Frac {
        self
            .0
            .iter()
            .map(|&(coeff, exp_var)| {
                let (_, value) = *vars
                    .iter()
                    .find(|&(v, _)| *v == exp_var)
                    .expect("We should be able to find every variable in the expression in the set of given variables");
                coeff * value
            }).sum()
    }

    /// Build an expression from user input with `inquire`.
    ///
    /// This method uses the given prompt for the first attempt, and then uses "Please try again:"
    /// on all subsequent attempts, printing the error in `inquire`'s "help message".
    pub fn build_from_user(prompt: &str, vars: &'v Variables) -> Result<Self> {
        let mut input = Text::new(prompt).prompt()?;

        loop {
            match Expression::parse(&input, vars) {
                Ok(exp) => return Ok(exp),
                Err(e) => {
                    input = Text::new("Please try again:")
                        .with_initial_value(&input)
                        .with_help_message(&format!("Error: {e}"))
                        .prompt()?;
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simplify_test() {
        assert_eq!(
            Expression(vec![(2.into(), "a"), (3.into(), "a")])
                .simplify()
                .0,
            vec![(5.into(), "a")]
        );
        assert_eq!(
            Expression(vec![
                (2.into(), "a"),
                (Frac::new(3u32, 10u32), "b"),
                (-Frac::new(1u32, 1u32), "a")
            ])
            .simplify()
            .0,
            vec![(1.into(), "a"), (Frac::new(3u32, 10u32), "b")]
        );
        assert_eq!(
            Expression(vec![
                (1.into(), "a"),
                (1.into(), "a"),
                (Frac::new(35u32, 10u32), "b"),
                (-Frac::new(2u32, 1u32), "a")
            ])
            .simplify()
            .0,
            vec![(Frac::new(35u32, 10u32), "b")]
        );
        assert_eq!(
            Expression(vec![
                (Frac::new(23u32, 10u32), "x"),
                (-Frac::new(2u32, 10u32), "y"),
                (Frac::new(46u32, 10u32), "z")
            ])
            .simplify()
            .0,
            vec![
                (Frac::new(23u32, 10u32), "x"),
                (-Frac::new(2u32, 10u32), "y"),
                (Frac::new(46u32, 10u32), "z")
            ]
        );
    }
}
