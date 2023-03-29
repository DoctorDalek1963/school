//! This module handles linear programming systems. See [`LinProgSystem`].

use super::{
    config::Config, constraint::Constraint, validate_variable, ObjectiveFunction, Variables,
};
use color_eyre::Result;
use inquire::{InquireError, Select, Text};
use ouroboros::self_referencing;
use std::{collections::HashSet, fmt};
use tracing::{debug, instrument};

/// A linear programming system, with a set of variables, objective function, and a set of contraints.
#[self_referencing(pub_extras)]
pub struct LinProgSystem {
    /// The variable set for the system. Every variable must be listed here for validation.
    pub variables: Variables,

    /// The config for the system.
    pub config: Config,

    /// The objective function - to maximise or minimise a given expression.
    #[borrows(variables)]
    #[not_covariant]
    pub objective_function: ObjectiveFunction<'this>,

    /// The constraints to optimise for.
    #[borrows(variables)]
    #[not_covariant]
    pub constraints: Vec<Constraint<'this>>,
}

impl fmt::Debug for LinProgSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("LinProgSystem");
        debug_struct.field("variables", self.borrow_variables());
        debug_struct.field("config", self.borrow_config());
        self.with_objective_function(|obj_func| debug_struct.field("objective_function", obj_func));
        self.with_constraints(|cons| debug_struct.field("constraints", cons));
        debug_struct.finish()
    }
}

impl LinProgSystem {
    /// Build a system from an `inquire` prompt.
    #[instrument]
    pub fn build_from_user() -> Result<Self> {
        let variables = Variables(
            Text::new("Please enter all your named variables, separated by spaces:")
                .prompt()?
                .split(' ')
                .filter(|&s| !s.is_empty())
                .map(|var| validate_variable(var).map(ToString::to_string))
                .collect::<Result<HashSet<String>>>()?,
        );
        debug!(?variables);

        let config = Config::build_from_user()?;
        debug!(?config);

        let system = LinProgSystemBuilder {
            variables,
            config,
            objective_function_builder: |variables: &Variables| {
                let objective_function = ObjectiveFunction::build_from_user(variables)
                    .expect("Building objective function from user should not fail")
                    .simplify();
                debug!(?objective_function);
                objective_function
            },
            constraints_builder: |variables: &Variables| {
                let mut constraints = Vec::new();

                loop {
                    let mut input = match Text::new("Please enter a constraint inequality:")
                        .with_help_message(
                            "The constant must be on the RHS; use <= for ≤ and >= for ≥",
                        )
                        .prompt()
                    {
                        Ok(x) => x,
                        Err(
                            InquireError::OperationCanceled | InquireError::OperationInterrupted,
                        ) => {
                            if constraints.is_empty() {
                                println!("You must have at least one constraint inequality");
                                continue;
                            } else {
                                break;
                            }
                        }
                        Err(e) => panic!("inquire::Text should not fail: {e:?}"),
                    };

                    'input_loop: loop {
                        match Constraint::nom_parse(&input, variables) {
                            Ok((_, cons)) => {
                                constraints.push(cons.simplify());
                                break 'input_loop;
                            }
                            Err(e) => {
                                input = match Text::new("Please try again:")
                                    .with_initial_value(&input)
                                    .with_help_message(
                                        "The constant must be on the RHS; use <= for ≤ and >= for ≥",
                                    )
                                    .with_help_message(&format!("Error: {e}"))
                                    .prompt()
                                {
                                    Ok(x) => x,
                                    Err(
                                        InquireError::OperationCanceled | InquireError::OperationInterrupted,
                                    ) => {
                                        if constraints.is_empty() {
                                            println!("You must have at least one constraint inequality");
                                            continue;
                                        } else {
                                            break;
                                        }
                                    }
                                    Err(e) => panic!("inquire::Text should not fail: {e:?}"),
                                };
                            }
                        }
                    }

                    match Select::new(
                        "Would you like to add another constraint?",
                        vec!["Yes", "No"],
                    )
                    .prompt()
                    .expect("inquire::Select should not fail")
                    {
                        "Yes" => continue,
                        "No" => break,
                        _ => unreachable!(
                            "inquire::Select should only yield the values given in the vec"
                        ),
                    };
                }

                debug!(?constraints);
                constraints
            },
        }
        .build();

        debug!("{:#?}", system);
        Ok(system)
    }
}
