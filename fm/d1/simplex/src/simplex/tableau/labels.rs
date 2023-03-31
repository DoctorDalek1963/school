//! This module contains labels for rows and columns in tableaux.

use crate::simplex::VariableType;
use std::fmt;

/// A label to use for a row in the tableau.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum RowLabel<'v> {
    /// A variable. See [`VariableType`].
    Variable(VariableType<'v>),

    /// The objective function.
    ObjectiveFunction,
}

impl<'v> fmt::Display for RowLabel<'v> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Variable(var) => write!(f, "{var}"),
            Self::ObjectiveFunction => write!(f, "ObjFunc#"),
        }
    }
}

impl<'v> TryFrom<ColumnLabel<'v>> for RowLabel<'v> {
    type Error = &'static str;

    fn try_from(value: ColumnLabel<'v>) -> std::result::Result<Self, Self::Error> {
        match value {
            ColumnLabel::BasicString(_) => {
                Err("Unable to convert ColumnLabel::BasicString to RowLabel")
            }
            ColumnLabel::Variable(v) => Ok(Self::Variable(v)),
        }
    }
}

/// A label to use for a column in the tableau.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum ColumnLabel<'v> {
    /// A basic string, like for "Value", "θ", or "Row ops".
    BasicString(String),

    /// The name of a variable.
    Variable(VariableType<'v>),
}

impl<'v> From<String> for ColumnLabel<'v> {
    fn from(value: String) -> Self {
        Self::BasicString(value)
    }
}

impl<'v> From<&str> for ColumnLabel<'v> {
    fn from(value: &str) -> Self {
        Self::BasicString(value.to_string())
    }
}

impl<'v> From<VariableType<'v>> for ColumnLabel<'v> {
    fn from(value: VariableType<'v>) -> Self {
        Self::Variable(value)
    }
}

impl<'v> fmt::Display for ColumnLabel<'v> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColumnLabel::BasicString(s) => write!(f, "{s}"),
            ColumnLabel::Variable(v) => write!(f, "{v}"),
        }
    }
}
