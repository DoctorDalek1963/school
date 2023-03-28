//! This crate exists to help solve linear programming problems and is used as a CLI app.
//!
//! Throughout the crate, `LinProg` is used as an abbreviation for "linear programming".

#![cfg_attr(debug_assertions, allow(unused_variables, dead_code))]

mod lin_prog;

use self::lin_prog::system::LinProgSystem;
use color_eyre::Result;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let system = LinProgSystem::build_from_user()?;

    Ok(())
}
