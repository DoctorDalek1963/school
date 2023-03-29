//! This module handles config for the [`LinProgSystem`].

use color_eyre::Result;
use inquire::MultiSelect;
use tracing::instrument;

/// A simple config struct to handle options for the [`LinProgSystem`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Config {
    /// Whether to include the non-negativity constraints (x, y, z ≥ 0).
    pub include_non_negativity: bool,

    /// Does this system require integer solutions?
    pub integer_solutions: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            include_non_negativity: true,
            integer_solutions: false,
        }
    }
}

impl Config {
    /// Build the config from user input using `inquire`.
    #[instrument]
    pub fn build_from_user() -> Result<Self> {
        const NON_NEGATIVITY: &str = "Include non-negativity";
        const INTEGER_SOLUTIONS: &str = "Require integer solutions";

        let selected = MultiSelect::new(
            "Please enable or disable configurations:",
            vec![NON_NEGATIVITY, INTEGER_SOLUTIONS],
        )
        .with_default(&[0])
        .prompt()?;

        Ok(Self {
            include_non_negativity: selected.contains(&NON_NEGATIVITY),
            integer_solutions: selected.contains(&INTEGER_SOLUTIONS),
        })
    }
}
