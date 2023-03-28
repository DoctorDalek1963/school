use super::{constraint::Constraint, ObjectiveFunction, Variables};
use crate::lin_prog::validate_variable;
use color_eyre::Result;
use inquire::{Confirm, Text};
use ouroboros::self_referencing;
use std::{collections::HashSet, fmt};
use tracing::{debug, instrument};

/// A linear programming system, with a set of variables, objective function, and a set of contraints.
#[self_referencing]
pub struct LinProgSystem {
    /// The variable set for the system. Every variable must be listed here for validation.
    variables: Variables,

    /// The objective function - to maximise or minimise a given expression.
    #[borrows(variables)]
    #[not_covariant]
    objective_function: ObjectiveFunction<'this>,

    /// The constraints to optimise for.
    #[borrows(variables)]
    #[not_covariant]
    constraints: Vec<Constraint<'this>>,
}

impl fmt::Debug for LinProgSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("LinProgSystem");
        debug_struct.field("variables", self.borrow_variables());
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
                .split(" ")
                .filter(|&s| !s.is_empty())
                .map(|var| validate_variable(var).map(|s| s.to_string()))
                .collect::<Result<HashSet<String>>>()?,
        );
        debug!(?variables);

        let system = LinProgSystemBuilder {
            variables,
            objective_function_builder: |variables: &Variables| {
                let objective_function = ObjectiveFunction::build_from_user(&variables)
                    .expect("Building objective function from user should not fail");
                debug!(?objective_function);
                objective_function
            },
            constraints_builder: |variables: &Variables| {
                let mut constraints = Vec::new();

                loop {
                    let input = match Text::new("Please enter a constraint inequality:")
                        .with_help_message(
                            "The constant must be on the RHS; use <= for ≤ and >= for ≥",
                        )
                        .prompt()
                    {
                        Ok(x) => x,
                        Err(_) => continue,
                    };

                    let (_, constraint) = Constraint::nom_parse(&input, &variables)
                        .expect("Parsing the constraint should not fail");
                    constraints.push(constraint);

                    if let Ok(false) =
                        Confirm::new("Would you like to add another constraint?").prompt()
                    {
                        break;
                    }
                }

                constraints
            },
        }
        .build();
        debug!(?system);
        Ok(system)
    }
}
