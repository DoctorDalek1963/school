//! This module handles const and non-const expressions.

pub mod const_expression;
pub mod simple_expression;

pub use const_expression::ConstExpression;
pub use simple_expression::Expression;
