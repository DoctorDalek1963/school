use crate::{
    lin_prog::{
        config::Config, constraint::Constraint, expression::Expression,
        system::LinProgSystemBuilder, ObjectiveFunction, Variables,
    },
    simplex::{solve_with_simplex_tableaux, SolutionSet, VariableType},
};
use std::collections::HashMap;

#[test]
#[ignore = "simplex tableaux not yet implemented"]
fn solve_with_simplex_tableaux_test() {
    assert_eq!(
        solve_with_simplex_tableaux(
            &LinProgSystemBuilder {
                variables: Variables::from(["x", "y", "z"]),
                config: Config::default(),
                objective_function_builder: |vars| {
                    ObjectiveFunction::Maximise(
                        Expression::nom_parse("10x + 12y + 8z", vars).unwrap().1,
                    )
                },
                constraints_builder: |vars| vec![
                    Constraint::nom_parse("2x + 2y <= 5", vars).unwrap().1,
                    Constraint::nom_parse("5x + 3y + 4z <= 15", vars).unwrap().1,
                ]
            }
            .build()
        )
        .unwrap(),
        SolutionSet {
            objective_function_value: 45.,
            variable_values: HashMap::from([
                (VariableType::Original("x"), 0.),
                (VariableType::Original("y"), 5. / 2.),
                (VariableType::Original("z"), 15. / 8.),
                (VariableType::Slack(0), 0.),
                (VariableType::Slack(1), 0.),
            ])
        },
        "Ch 7 Example 7 or 10"
    );

    assert_eq!(
        solve_with_simplex_tableaux(
            &LinProgSystemBuilder {
                variables: Variables::from(["x", "y"]),
                config: Config::default(),
                objective_function_builder: |vars| {
                    ObjectiveFunction::Maximise(Expression::nom_parse("3x + 2y", vars).unwrap().1)
                },
                constraints_builder: |vars| vec![
                    Constraint::nom_parse("5x + 7y ≤ 70", vars).unwrap().1,
                    Constraint::nom_parse("10x + 3y ≤ 60", vars).unwrap().1,
                ]
            }
            .build()
        )
        .unwrap(),
        SolutionSet {
            objective_function_value: 26.,
            variable_values: HashMap::from([
                (VariableType::Original("x"), 42. / 11.),
                (VariableType::Original("y"), 80. / 11.),
                (VariableType::Slack(0), 0.),
                (VariableType::Slack(1), 0.),
            ])
        },
        "Ch 7 Example 8"
    );

    assert_eq!(
        solve_with_simplex_tableaux(
            &LinProgSystemBuilder {
                variables: Variables::from(["x", "y"]),
                config: Config::default(),
                objective_function_builder: |vars| {
                    ObjectiveFunction::Minimise(Expression::nom_parse("3x - y", vars).unwrap().1)
                },
                constraints_builder: |vars| vec![
                    Constraint::nom_parse("2x + y ≤ 12", vars).unwrap().1,
                    Constraint::nom_parse("x + 4y <= 8", vars).unwrap().1,
                ]
            }
            .build()
        )
        .unwrap(),
        SolutionSet {
            objective_function_value: -2.,
            variable_values: HashMap::from([
                (VariableType::Original("x"), 0.),
                (VariableType::Original("y"), 2.),
                (VariableType::Slack(0), 10.),
                (VariableType::Slack(1), 0.),
            ])
        },
        "Ch 7 Example 9 (minimise)"
    );

    assert_eq!(
        solve_with_simplex_tableaux(
            &LinProgSystemBuilder {
                variables: Variables::from(["x", "y", "z"]),
                config: Config::default(),
                objective_function_builder: |vars| {
                    ObjectiveFunction::Minimise(
                        Expression::nom_parse("3x + 4y - 5z", vars).unwrap().1,
                    )
                },
                constraints_builder: |vars| vec![
                    Constraint::nom_parse("2x - 3y + 2z <= 4", vars).unwrap().1,
                    Constraint::nom_parse("x + 2y + 4z <= 8", vars).unwrap().1,
                    Constraint::nom_parse("y - z <= 6", vars).unwrap().1,
                ]
            }
            .build()
        )
        .unwrap(),
        SolutionSet {
            objective_function_value: -2.,
            variable_values: HashMap::from([
                (VariableType::Original("x"), 32. / 7.),
                (VariableType::Original("y"), 12. / 7.),
                (VariableType::Original("z"), 0.),
                (VariableType::Slack(0), 0.),
                (VariableType::Slack(1), 0.),
                (VariableType::Slack(2), 30. / 7.),
            ])
        },
        "Ch 7 Example 11"
    );
}
