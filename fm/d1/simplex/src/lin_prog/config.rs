//! This module handles config for the [`LinProgSystem`].

use color_eyre::Result;
use inquire::MultiSelect;
use tracing::instrument;

/// A simple config struct to handle options for the [`LinProgSystem`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Config {
    /// Does this system require integer solutions?
    pub integer_solutions: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            integer_solutions: false,
        }
    }
}

impl Config {
    /// Build the config from user input using `inquire`.
    #[instrument]
    pub fn build_from_user() -> Result<Self> {
        const INTEGER_SOLUTIONS: &str = "Require integer solutions";

        let selected = MultiSelect::new(
            "Please enable or disable configurations:",
            vec![INTEGER_SOLUTIONS],
        )
        .with_default(&[])
        .prompt()?;

        Ok(Self {
            integer_solutions: selected.contains(&INTEGER_SOLUTIONS),
        })
    }
}
