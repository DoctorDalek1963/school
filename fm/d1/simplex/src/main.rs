//! This crate exists to help solve linear programming problems and is used as a CLI app.
//!
//! Throughout the crate, `LinProg` is used as an abbreviation for "linear programming".

#![cfg_attr(debug_assertions, allow(unused_variables, dead_code))]

mod lin_prog;
mod simplex;

use self::{lin_prog::system::LinProgSystem, simplex::solve_with_simplex_tableaux};
use color_eyre::Result;
use fraction::GenericFraction;
use tracing::info;

type Frac = GenericFraction<u32>;

fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}

fn main() -> Result<()> {
    install_tracing();
    color_eyre::install()?;

    let system = LinProgSystem::build_from_user()?;
    let solution = solve_with_simplex_tableaux(&system)?;
    info!(%solution, "Solution found!");

    Ok(())
}
