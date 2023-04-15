use crate::{
    lin_prog::{
        config::Config, constraint::Constraint, expression::Expression,
        system::LinProgSystemBuilder, ObjectiveFunction, Variables,
    },
    simplex::{solve_with_simplex_tableaux, tableau::Tableau, SolutionSet, VariableType},
    Frac,
};
use fraction::Zero;
use std::collections::HashMap;
use tracing_test::traced_test;

#[test]
#[traced_test]
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
            objective_function_value: Frac::new(45u32, 1u32),
            variable_values: HashMap::from([
                (VariableType::Original("x"), Frac::zero()),
                (VariableType::Original("y"), Frac::new(5u32, 2u32)),
                (VariableType::Original("z"), Frac::new(15u32, 8u32)),
                (VariableType::Slack(0), Frac::zero()),
                (VariableType::Slack(1), Frac::zero()),
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
            objective_function_value: 26.into(),
            variable_values: HashMap::from([
                (VariableType::Original("x"), Frac::new(42u32, 11u32)),
                (VariableType::Original("y"), Frac::new(80u32, 11u32)),
                (VariableType::Slack(0), Frac::zero()),
                (VariableType::Slack(1), Frac::zero()),
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
            objective_function_value: -Frac::new(2u32, 1u32),
            variable_values: HashMap::from([
                (VariableType::Original("x"), Frac::zero()),
                (VariableType::Original("y"), 2.into()),
                (VariableType::Slack(0), 10.into()),
                (VariableType::Slack(1), Frac::zero()),
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
                    ObjectiveFunction::Maximise(
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
            objective_function_value: Frac::new(144u32, 7u32),
            variable_values: HashMap::from([
                (VariableType::Original("x"), Frac::new(32u32, 7u32)),
                (VariableType::Original("y"), Frac::new(12u32, 7u32)),
                (VariableType::Original("z"), Frac::zero()),
                (VariableType::Slack(0), Frac::zero()),
                (VariableType::Slack(1), Frac::zero()),
                (VariableType::Slack(2), Frac::new(30u32, 7u32)),
            ])
        },
        "Ch 7 Example 11"
    );
}

#[test]
#[traced_test]
fn solve_with_simplex_tableaux_integer_solutions_test() {
    assert_eq!(
        solve_with_simplex_tableaux(
            &LinProgSystemBuilder {
                variables: Variables::from(["x", "y"]),
                config: Config {
                    integer_solutions: true,
                },
                objective_function_builder: |vars| {
                    ObjectiveFunction::Maximise(Expression::nom_parse("3x + 2y", vars).unwrap().1)
                },
                constraints_builder: |vars| {
                    vec![
                        Constraint::nom_parse("5x + 7y <= 70", vars).unwrap().1,
                        Constraint::nom_parse("10x + 3y <= 60", vars).unwrap().1,
                    ]
                },
            }
            .build()
        )
        .unwrap(),
        SolutionSet {
            objective_function_value: 23.into(),
            variable_values: HashMap::from([
                (VariableType::Original("x"), 3.into()),
                (VariableType::Original("y"), 7.into()),
            ])
        },
        "Ch 7 Example 12"
    );

    assert_eq!(
        solve_with_simplex_tableaux(
            &LinProgSystemBuilder {
                variables: Variables::from(["x", "y", "z"]),
                config: Config {
                    integer_solutions: true,
                },
                objective_function_builder: |vars| {
                    ObjectiveFunction::Maximise(
                        Expression::nom_parse("10x + 12y + 8z", vars).unwrap().1,
                    )
                },
                constraints_builder: |vars| {
                    vec![
                        Constraint::nom_parse("2x + 2y <= 5", vars).unwrap().1,
                        Constraint::nom_parse("5x + 3y + 4z <= 15", vars).unwrap().1,
                    ]
                },
            }
            .build()
        )
        .unwrap(),
        SolutionSet {
            objective_function_value: 40.into(),
            variable_values: HashMap::from([
                (VariableType::Original("x"), Frac::zero()),
                (VariableType::Original("y"), 2.into()),
                (VariableType::Original("z"), 2.into()),
            ])
        },
        "Ch 7 Example 12"
    );
}

#[test]
#[traced_test]
fn create_initial_tableau_test() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        Tableau::create_initial(
            &LinProgSystemBuilder {
                variables: Variables::from(["x", "y", "z"]),
                config: Config::default(),
                objective_function_builder: |vars| ObjectiveFunction::Maximise(
                    Expression::nom_parse("3x + 5y - z", vars).unwrap().1
                ),
                constraints_builder: |vars| vec![
                    Constraint::nom_parse("x - 2y + 10z <= 100", vars)
                        .unwrap()
                        .1,
                    Constraint::nom_parse("2x + y - 13z ≤ 34", vars).unwrap().1,
                    Constraint::nom_parse("3x + 4x - 7y + 3z <= 400", vars)
                        .unwrap()
                        .1
                        .simplify(),
                ]
            }
            .build(),
        )
        .unwrap()
        .to_string(),
        r#"
┌───────────┬────┬────┬─────┬──────┬──────┬──────┬───────┬───┬────────┐
│ Basic var │ x  │ y  │ z   │ sl#0 │ sl#1 │ sl#2 │ Value │ θ │ Row op │
├───────────┼────┼────┼─────┼──────┼──────┼──────┼───────┼───┼────────┤
│ sl#0      │ 1  │ -2 │ 10  │ 1    │ 0    │ 0    │ 100   │   │        │
├───────────┼────┼────┼─────┼──────┼──────┼──────┼───────┼───┼────────┤
│ sl#1      │ 2  │ 1  │ -13 │ 0    │ 1    │ 0    │ 34    │   │        │
├───────────┼────┼────┼─────┼──────┼──────┼──────┼───────┼───┼────────┤
│ sl#2      │ 7  │ -7 │ 3   │ 0    │ 0    │ 1    │ 400   │   │        │
├───────────┼────┼────┼─────┼──────┼──────┼──────┼───────┼───┼────────┤
│ ObjFunc#  │ -3 │ -5 │ 1   │ 0    │ 0    │ 0    │ 0     │   │        │
└───────────┴────┴────┴─────┴──────┴──────┴──────┴───────┴───┴────────┘"#
    );

    assert_eq!(
        Tableau::create_initial(
            &LinProgSystemBuilder {
                variables: Variables::from(["x", "y", "z", "w"]),
                config: Config::default(),
                objective_function_builder: |vars| ObjectiveFunction::Maximise(
                    Expression::nom_parse("3x + 5y - z + 1.5w", vars).unwrap().1
                ),
                constraints_builder: |vars| vec![
                    Constraint::nom_parse("x - 10z <= 100", vars).unwrap().1,
                    Constraint::nom_parse("w <= 19", vars).unwrap().1,
                    Constraint::nom_parse("2w - 3z + x <= 12.2", vars)
                        .unwrap()
                        .1,
                    Constraint::nom_parse("3y + 3x + 2z - 0.2w <= 250", vars)
                        .unwrap()
                        .1
                ]
            }
            .build(),
        )
        .unwrap()
        .to_string(),
        r#"
┌───────────┬──────┬────┬────┬─────┬──────┬──────┬──────┬──────┬───────┬───┬────────┐
│ Basic var │ w    │ x  │ y  │ z   │ sl#0 │ sl#1 │ sl#2 │ sl#3 │ Value │ θ │ Row op │
├───────────┼──────┼────┼────┼─────┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ sl#0      │ 0    │ 1  │ 0  │ -10 │ 1    │ 0    │ 0    │ 0    │ 100   │   │        │
├───────────┼──────┼────┼────┼─────┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ sl#1      │ 1    │ 0  │ 0  │ 0   │ 0    │ 1    │ 0    │ 0    │ 19    │   │        │
├───────────┼──────┼────┼────┼─────┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ sl#2      │ 2    │ 1  │ 0  │ -3  │ 0    │ 0    │ 1    │ 0    │ 61/5  │   │        │
├───────────┼──────┼────┼────┼─────┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ sl#3      │ -1/5 │ 3  │ 3  │ 2   │ 0    │ 0    │ 0    │ 1    │ 250   │   │        │
├───────────┼──────┼────┼────┼─────┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ ObjFunc#  │ -3/2 │ -3 │ -5 │ 1   │ 0    │ 0    │ 0    │ 0    │ 0     │   │        │
└───────────┴──────┴────┴────┴─────┴──────┴──────┴──────┴──────┴───────┴───┴────────┘"#
    );
}

#[test]
#[traced_test]
fn tableau_iteration_test() {
    use pretty_assertions::assert_eq;

    let system = &LinProgSystemBuilder {
        variables: Variables::from(["x", "y"]),
        config: Config::default(),
        objective_function_builder: |vars| {
            ObjectiveFunction::Maximise(Expression::nom_parse("3x + 2y", vars).unwrap().1)
        },
        constraints_builder: |vars| {
            vec![
                Constraint::nom_parse("5x + 7y <= 70", vars).unwrap().1,
                Constraint::nom_parse("10x + 3y <= 60", vars).unwrap().1,
            ]
        },
    }
    .build();
    let mut tableau = Tableau::create_initial(&system).unwrap();

    assert_eq!(
        tableau.to_string(),
        r#"
┌───────────┬────┬────┬──────┬──────┬───────┬───┬────────┐
│ Basic var │ x  │ y  │ sl#0 │ sl#1 │ Value │ θ │ Row op │
├───────────┼────┼────┼──────┼──────┼───────┼───┼────────┤
│ sl#0      │ 5  │ 7  │ 1    │ 0    │ 70    │   │        │
├───────────┼────┼────┼──────┼──────┼───────┼───┼────────┤
│ sl#1      │ 10 │ 3  │ 0    │ 1    │ 60    │   │        │
├───────────┼────┼────┼──────┼──────┼───────┼───┼────────┤
│ ObjFunc#  │ -3 │ -2 │ 0    │ 0    │ 0     │   │        │
└───────────┴────┴────┴──────┴──────┴───────┴───┴────────┘"#,
        "Ch 7 Example 8 initial"
    );

    tableau.do_iteration();
    assert_eq!(
        tableau.to_string(),
        r#"
┌───────────┬───┬────────┬──────┬──────┬───────┬───┬────────┐
│ Basic var │ x │ y      │ sl#0 │ sl#1 │ Value │ θ │ Row op │
├───────────┼───┼────────┼──────┼──────┼───────┼───┼────────┤
│ sl#0      │ 0 │ 11/2   │ 1    │ -1/2 │ 40    │   │        │
├───────────┼───┼────────┼──────┼──────┼───────┼───┼────────┤
│ x         │ 1 │ 3/10   │ 0    │ 1/10 │ 6     │   │        │
├───────────┼───┼────────┼──────┼──────┼───────┼───┼────────┤
│ ObjFunc#  │ 0 │ -11/10 │ 0    │ 3/10 │ 18    │   │        │
└───────────┴───┴────────┴──────┴──────┴───────┴───┴────────┘"#,
        "Ch 7 Example 8 after 1 complete iteration"
    );

    // The floating point error is noticeable in this one. Those two values in the bottom row that
    // look like 0.2 should be exactly 0.2
    tableau.do_iteration();
    assert_eq!(
        tableau.to_string(),
        r#"
┌───────────┬───┬───┬───────┬───────┬───────┬───┬────────┐
│ Basic var │ x │ y │ sl#0  │ sl#1  │ Value │ θ │ Row op │
├───────────┼───┼───┼───────┼───────┼───────┼───┼────────┤
│ y         │ 0 │ 1 │ 2/11  │ -1/11 │ 80/11 │   │        │
├───────────┼───┼───┼───────┼───────┼───────┼───┼────────┤
│ x         │ 1 │ 0 │ -3/55 │ 7/55  │ 42/11 │   │        │
├───────────┼───┼───┼───────┼───────┼───────┼───┼────────┤
│ ObjFunc#  │ 0 │ 0 │ 1/5   │ 1/5   │ 26    │   │        │
└───────────┴───┴───┴───────┴───────┴───────┴───┴────────┘"#,
        "Ch 7 Example 8 after 2 complete iterations"
    );
}
