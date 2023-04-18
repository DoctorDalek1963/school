//! This module handles const expressions, which can include constant terms as well as variable
//! terms.

use crate::Frac;
use fraction::Zero;
use itertools::Itertools;
use std::{
    cmp::Ord,
    collections::HashMap,
    hash::Hash,
    iter::{self, Sum},
    ops::{Add, Neg},
};

/// A variable with a coefficient, or a constant.
#[derive(Clone, Debug, PartialEq, Hash)]
pub enum VariableOrConst<'v, T: ?Sized + Ord + Hash> {
    /// A variable term with a coefficient.
    Variable(Frac, &'v T),

    /// A constant term.
    Constant(Frac),
}

/// An expression of variable terms and constant terms.
#[derive(Clone, Debug, PartialEq)]
pub struct ConstExpression<'v, T: ?Sized + Ord + Hash>(pub Vec<VariableOrConst<'v, T>>);

impl<'v, T: ?Sized + Ord + Hash> ConstExpression<'v, T> {
    /// Algebraically simplify the expression.
    pub fn simplify(self) -> Self {
        let (constant, variables) = self.0
            .into_iter()
            // Fold the values into a constant and a map of variable terms
            .fold((Frac::zero(), HashMap::<&'v T, Frac>::new()), |acc, variable_or_const| {
                let (mut constant, mut map) = acc;
                match variable_or_const {
                    VariableOrConst::Variable(num, var) => match map.get_mut(var) {
                        Some(n) => *n += num,
                        None => {
                            map.insert(var, num);
                        }
                    }
                    VariableOrConst::Constant(num) => constant += num,
                };
                (constant, map)
            });

        let variables = variables.into_iter()
            // Swap the values in the tuple
            .map(|(var, num)| (num, var))
            // Filter out zeroes
            .filter(|&(num, _)| num != Frac::zero())
            // Sort them by variable name for consistency
            .sorted_by_key(|&(_, var)| var)
            .map(|(coeff, var)| VariableOrConst::Variable(coeff, var));

        Self(
            iter::once(VariableOrConst::Constant(constant))
                .chain(variables)
                // Filter out the constant if it's 0
                .filter(|var_or_const| match var_or_const {
                    VariableOrConst::Constant(x) if *x == Frac::zero() => false,
                    _ => true,
                })
                .collect(),
        )
    }

    /// Return the optional constant term in the expression.
    pub fn constant(&self) -> Option<Frac> {
        self.0.iter().find_map(|var_or_const| match var_or_const {
            VariableOrConst::Constant(num) => Some(*num),
            _ => None,
        })
    }
}

impl<'v, T: ?Sized + Ord + Hash> Add for ConstExpression<'v, T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.into_iter().chain(rhs.0.into_iter()).collect()).simplify()
    }
}

impl<'v, T: ?Sized + Ord + Hash> Neg for ConstExpression<'v, T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        use VariableOrConst::*;

        Self(
            self.0
                .into_iter()
                .map(|var_or_const| match var_or_const {
                    Variable(num, var) => Variable(-num, var),
                    Constant(num) => Constant(-num),
                })
                .collect(),
        )
        .simplify()
    }
}

impl<'v, T: ?Sized + Ord + Hash> Sum for ConstExpression<'v, T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(ConstExpression(vec![]), |acc, expression| acc + expression)
            .simplify()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use VariableOrConst::*;

    #[test]
    fn simplify_from_variable_expression_test() {
        assert_eq!(
            ConstExpression(vec![Variable(2.into(), "a"), Variable(3.into(), "a")])
                .simplify()
                .0,
            vec![Variable(5.into(), "a")]
        );
        assert_eq!(
            ConstExpression(vec![
                Variable(2.into(), "a"),
                Variable(Frac::new(3u32, 10u32), "b"),
                Variable(-Frac::new(1u32, 1u32), "a")
            ])
            .simplify()
            .0,
            vec![
                Variable(1.into(), "a"),
                Variable(Frac::new(3u32, 10u32), "b")
            ]
        );
        assert_eq!(
            ConstExpression(vec![
                Variable(1.into(), "a"),
                Variable(1.into(), "a"),
                Variable(Frac::new(35u32, 10u32), "b"),
                Variable(-Frac::new(2u32, 1u32), "a")
            ])
            .simplify()
            .0,
            vec![Variable(Frac::new(35u32, 10u32), "b")]
        );
        assert_eq!(
            ConstExpression(vec![
                Variable(Frac::new(23u32, 10u32), "x"),
                Variable(-Frac::new(2u32, 10u32), "y"),
                Variable(Frac::new(46u32, 10u32), "z")
            ])
            .simplify()
            .0,
            vec![
                Variable(Frac::new(23u32, 10u32), "x"),
                Variable(-Frac::new(2u32, 10u32), "y"),
                Variable(Frac::new(46u32, 10u32), "z")
            ]
        );
    }

    #[test]
    fn simplify_test() {
        assert_eq!(
            ConstExpression(vec![
                Constant(15.into()),
                Variable(3.into(), "a"),
                Variable(Frac::new(1u32, 2u32), "a")
            ])
            .simplify()
            .0,
            vec![Constant(15.into()), Variable(Frac::new(7u32, 2u32), "a")]
        );
        assert_eq!(
            ConstExpression(vec![
                Constant(15.into()),
                Variable(3.into(), "a"),
                Variable(Frac::new(5u32, 2u32), "a"),
                Constant(Frac::new(12u32, 13u32))
            ])
            .simplify()
            .0,
            vec![
                Constant(Frac::new(207u32, 13u32)),
                Variable(Frac::new(11u32, 2u32), "a")
            ]
        );
        assert_eq!(
            ConstExpression(vec![
                Constant(15.into()),
                Variable(3.into(), "a"),
                Variable(Frac::new(5u32, 2u32), "b"),
                Constant(Frac::new(12u32, 13u32))
            ])
            .simplify()
            .0,
            vec![
                Constant(Frac::new(207u32, 13u32)),
                Variable(3.into(), "a"),
                Variable(Frac::new(5u32, 2u32), "b")
            ]
        );
        assert_eq!(
            ConstExpression(vec![
                Constant(-Frac::new(4u32, 3u32)),
                Variable(Frac::new(3u32, 7u32), "a"),
                Variable(Frac::new(5u32, 3u32), "b"),
                Constant(Frac::new(4u32, 3u32))
            ])
            .simplify()
            .0,
            vec![
                Variable(Frac::new(3u32, 7u32), "a"),
                Variable(Frac::new(5u32, 3u32), "b")
            ]
        );
    }

    #[test]
    fn add_trait_test() {
        assert_eq!(
            ConstExpression(vec![
                Constant(15.into()),
                Variable(3.into(), "a"),
                Variable(Frac::new(1u32, 2u32), "a")
            ]) + ConstExpression(vec![
                Constant(-Frac::new(4u32, 3u32)),
                Variable(Frac::new(3u32, 7u32), "a"),
                Variable(Frac::new(5u32, 3u32), "b")
            ]),
            ConstExpression(vec![
                Constant(Frac::new(41u32, 3u32)),
                Variable(Frac::new(55u32, 14u32), "a"),
                Variable(Frac::new(5u32, 3u32), "b")
            ])
        );
        assert_eq!(
            ConstExpression(vec![
                Constant(-Frac::new(13u32, 5u32)),
                Variable(Frac::new(1u32, 2u32), "b")
            ]) + ConstExpression(vec![
                Constant(-Frac::new(4u32, 3u32)),
                Variable(Frac::new(3u32, 7u32), "a"),
                Variable(Frac::new(5u32, 3u32), "b")
            ]),
            ConstExpression(vec![
                Constant(-Frac::new(59u32, 15u32)),
                Variable(Frac::new(3u32, 7u32), "a"),
                Variable(Frac::new(13u32, 6u32), "b")
            ])
        );
    }

    #[test]
    fn neg_trait_test() {
        assert_eq!(
            -ConstExpression(vec![
                Constant(15.into()),
                Variable(3.into(), "a"),
                Variable(Frac::new(1u32, 2u32), "a")
            ]),
            ConstExpression(vec![
                Constant(-Frac::new(15u32, 1u32)),
                Variable(-Frac::new(7u32, 2u32), "a")
            ])
        );
        assert_eq!(
            -ConstExpression(vec![
                Constant(-Frac::new(59u32, 15u32)),
                Variable(Frac::new(3u32, 7u32), "a"),
                Variable(Frac::new(13u32, 6u32), "b")
            ]),
            ConstExpression(vec![
                Constant(Frac::new(59u32, 15u32)),
                Variable(-Frac::new(3u32, 7u32), "a"),
                Variable(-Frac::new(13u32, 6u32), "b")
            ]),
        );
    }
}
