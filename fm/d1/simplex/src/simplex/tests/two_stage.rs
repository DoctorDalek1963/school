use crate::{
    lin_prog::{
        config::Config, constraint::Constraint, expression::Expression,
        system::LinProgSystemBuilder, ObjectiveFunction, Variables,
    },
    simplex::{
        solve_with_simplex_tableaux,
        tableau::{NoFeasibleSolution, Tableau},
        SolutionSet, VariableType,
    },
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
                        Expression::nom_parse("3x - 2y + z", vars).unwrap().1,
                    )
                },
                constraints_builder: |vars| {
                    vec![
                        Constraint::nom_parse("x + y + 2z <= 10", vars).unwrap().1,
                        Constraint::nom_parse("2x - 3y + z ≥ 5", vars).unwrap().1,
                        Constraint::nom_parse("x + y >= 8", vars).unwrap().1,
                    ]
                },
            }
            .build()
        )
        .unwrap(),
        SolutionSet {
            objective_function_value: 30.into(),
            variable_values: HashMap::from([
                (VariableType::Original("x"), 10.into()),
                (VariableType::Original("y"), Frac::zero()),
                (VariableType::Original("z"), Frac::zero()),
                (VariableType::Slack(0), Frac::zero()),
                (VariableType::Surplus(0), 15.into()),
                (VariableType::Surplus(1), 2.into()),
            ])
        },
        "Ch 7 Example 15"
    );

    assert!(
        solve_with_simplex_tableaux(
            &LinProgSystemBuilder {
                variables: Variables::from(["x", "y", "z"]),
                config: Config::default(),
                objective_function_builder: |vars| {
                    ObjectiveFunction::Maximise(
                        Expression::nom_parse("3x - 2y + z", vars).unwrap().1,
                    )
                },
                constraints_builder: |vars| {
                    vec![
                        Constraint::nom_parse("x + y + 2z <= 8", vars).unwrap().1,
                        Constraint::nom_parse("2x - 3y + z ≥ 5", vars).unwrap().1,
                        Constraint::nom_parse("x + y >= 10", vars).unwrap().1,
                    ]
                },
            }
            .build()
        )
        .is_err_and(|err| err
            .to_string()
            .contains("No feasible solution for the given system")),
        "Ch 7 Example 16"
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
                    Expression::nom_parse("3x - 2y + z", vars).unwrap().1
                ),
                constraints_builder: |vars| vec![
                    Constraint::nom_parse("x + y + 2z <= 10", vars).unwrap().1,
                    Constraint::nom_parse("2x - 3y + z ≥ 5", vars).unwrap().1,
                    Constraint::nom_parse("x + y >= 8", vars).unwrap().1,
                ]
            }
            .build()
        )
        .unwrap()
        .to_string(),
        r#"
┌─────────────┬────┬────┬────┬──────┬──────┬──────┬──────┬──────┬───────┬───┬────────┐
│ Basic var   │ x  │ y  │ z  │ sl#0 │ su#0 │ su#1 │ ar#0 │ ar#1 │ Value │ θ │ Row op │
├─────────────┼────┼────┼────┼──────┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ sl#0        │ 1  │ 1  │ 2  │ 1    │ 0    │ 0    │ 0    │ 0    │ 10    │   │        │
├─────────────┼────┼────┼────┼──────┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ ar#0        │ 2  │ -3 │ 1  │ 0    │ -1   │ 0    │ 1    │ 0    │ 5     │   │        │
├─────────────┼────┼────┼────┼──────┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ ar#1        │ 1  │ 1  │ 0  │ 0    │ 0    │ -1   │ 0    │ 1    │ 8     │   │        │
├─────────────┼────┼────┼────┼──────┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ ObjFunc#    │ -3 │ 2  │ -1 │ 0    │ 0    │ 0    │ 0    │ 0    │ 0     │   │        │
├─────────────┼────┼────┼────┼──────┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ TwoStageAr# │ -3 │ 2  │ -1 │ 0    │ 1    │ 1    │ 0    │ 0    │ -13   │   │        │
└─────────────┴────┴────┴────┴──────┴──────┴──────┴──────┴──────┴───────┴───┴────────┘"#,
        "Ch 7 Example 15"
    );

    assert_eq!(
        Tableau::create_initial(
            &LinProgSystemBuilder {
                variables: Variables::from(["x", "y", "z"]),
                config: Config::default(),
                objective_function_builder: |vars| ObjectiveFunction::Maximise(
                    Expression::nom_parse("3x - 2y + z", vars).unwrap().1
                ),
                constraints_builder: |vars| vec![
                    Constraint::nom_parse("x + y + 2z <= 8", vars).unwrap().1,
                    Constraint::nom_parse("2x - 3y + z ≥ 5", vars).unwrap().1,
                    Constraint::nom_parse("x + y >= 10", vars).unwrap().1,
                ]
            }
            .build()
        )
        .unwrap()
        .to_string(),
        r#"
┌─────────────┬────┬────┬────┬──────┬──────┬──────┬──────┬──────┬───────┬───┬────────┐
│ Basic var   │ x  │ y  │ z  │ sl#0 │ su#0 │ su#1 │ ar#0 │ ar#1 │ Value │ θ │ Row op │
├─────────────┼────┼────┼────┼──────┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ sl#0        │ 1  │ 1  │ 2  │ 1    │ 0    │ 0    │ 0    │ 0    │ 8     │   │        │
├─────────────┼────┼────┼────┼──────┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ ar#0        │ 2  │ -3 │ 1  │ 0    │ -1   │ 0    │ 1    │ 0    │ 5     │   │        │
├─────────────┼────┼────┼────┼──────┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ ar#1        │ 1  │ 1  │ 0  │ 0    │ 0    │ -1   │ 0    │ 1    │ 10    │   │        │
├─────────────┼────┼────┼────┼──────┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ ObjFunc#    │ -3 │ 2  │ -1 │ 0    │ 0    │ 0    │ 0    │ 0    │ 0     │   │        │
├─────────────┼────┼────┼────┼──────┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ TwoStageAr# │ -3 │ 2  │ -1 │ 0    │ 1    │ 1    │ 0    │ 0    │ -15   │   │        │
└─────────────┴────┴────┴────┴──────┴──────┴──────┴──────┴──────┴───────┴───┴────────┘"#,
        "Ch 7 Example 16"
    );
}

#[test]
#[traced_test]
fn tableau_iteration_test() -> Result<(), NoFeasibleSolution> {
    use pretty_assertions::assert_eq;

    let system = LinProgSystemBuilder {
        variables: Variables::from(["x", "y", "z"]),
        config: Config::default(),
        objective_function_builder: |vars| {
            ObjectiveFunction::Maximise(Expression::nom_parse("3x - 2y + z", vars).unwrap().1)
        },
        constraints_builder: |vars| {
            vec![
                Constraint::nom_parse("x + y + 2z <= 10", vars).unwrap().1,
                Constraint::nom_parse("2x - 3y + z ≥ 5", vars).unwrap().1,
                Constraint::nom_parse("x + y >= 8", vars).unwrap().1,
            ]
        },
    }
    .build();
    let mut tableau = Tableau::create_initial(&system).unwrap();

    assert_eq!(
        tableau.to_string(),
        r#"
┌─────────────┬────┬────┬────┬──────┬──────┬──────┬──────┬──────┬───────┬───┬────────┐
│ Basic var   │ x  │ y  │ z  │ sl#0 │ su#0 │ su#1 │ ar#0 │ ar#1 │ Value │ θ │ Row op │
├─────────────┼────┼────┼────┼──────┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ sl#0        │ 1  │ 1  │ 2  │ 1    │ 0    │ 0    │ 0    │ 0    │ 10    │   │        │
├─────────────┼────┼────┼────┼──────┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ ar#0        │ 2  │ -3 │ 1  │ 0    │ -1   │ 0    │ 1    │ 0    │ 5     │   │        │
├─────────────┼────┼────┼────┼──────┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ ar#1        │ 1  │ 1  │ 0  │ 0    │ 0    │ -1   │ 0    │ 1    │ 8     │   │        │
├─────────────┼────┼────┼────┼──────┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ ObjFunc#    │ -3 │ 2  │ -1 │ 0    │ 0    │ 0    │ 0    │ 0    │ 0     │   │        │
├─────────────┼────┼────┼────┼──────┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ TwoStageAr# │ -3 │ 2  │ -1 │ 0    │ 1    │ 1    │ 0    │ 0    │ -13   │   │        │
└─────────────┴────┴────┴────┴──────┴──────┴──────┴──────┴──────┴───────┴───┴────────┘"#,
        "Ch 7 Example 15 initial"
    );

    tableau.do_iteration()?;
    assert_eq!(
        tableau.to_string(),
        r#"
┌─────────────┬───┬──────┬──────┬──────┬──────┬──────┬──────┬──────┬───────┬───┬────────┐
│ Basic var   │ x │ y    │ z    │ sl#0 │ su#0 │ su#1 │ ar#0 │ ar#1 │ Value │ θ │ Row op │
├─────────────┼───┼──────┼──────┼──────┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ sl#0        │ 0 │ 5/2  │ 3/2  │ 1    │ 1/2  │ 0    │ -1/2 │ 0    │ 15/2  │   │        │
├─────────────┼───┼──────┼──────┼──────┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ x           │ 1 │ -3/2 │ 1/2  │ 0    │ -1/2 │ 0    │ 1/2  │ 0    │ 5/2   │   │        │
├─────────────┼───┼──────┼──────┼──────┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ ar#1        │ 0 │ 5/2  │ -1/2 │ 0    │ 1/2  │ -1   │ -1/2 │ 1    │ 11/2  │   │        │
├─────────────┼───┼──────┼──────┼──────┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ ObjFunc#    │ 0 │ -5/2 │ 1/2  │ 0    │ -3/2 │ 0    │ 3/2  │ 0    │ 15/2  │   │        │
├─────────────┼───┼──────┼──────┼──────┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ TwoStageAr# │ 0 │ -5/2 │ 1/2  │ 0    │ -1/2 │ 1    │ 3/2  │ 0    │ -11/2 │   │        │
└─────────────┴───┴──────┴──────┴──────┴──────┴──────┴──────┴──────┴───────┴───┴────────┘"#,
        "Ch 7 Example 15 after 1 complete iteration"
    );

    tableau.do_iteration()?;
    assert_eq!(
        tableau.to_string(),
        r#"
┌───────────┬───┬───┬──────┬──────┬──────┬──────┬───────┬───┬────────┐
│ Basic var │ x │ y │ z    │ sl#0 │ su#0 │ su#1 │ Value │ θ │ Row op │
├───────────┼───┼───┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ sl#0      │ 0 │ 0 │ 2    │ 1    │ 0    │ 1    │ 2     │   │        │
├───────────┼───┼───┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ x         │ 1 │ 0 │ 1/5  │ 0    │ -1/5 │ -3/5 │ 29/5  │   │        │
├───────────┼───┼───┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ y         │ 0 │ 1 │ -1/5 │ 0    │ 1/5  │ -2/5 │ 11/5  │   │        │
├───────────┼───┼───┼──────┼──────┼──────┼──────┼───────┼───┼────────┤
│ ObjFunc#  │ 0 │ 0 │ 0    │ 0    │ -1   │ -1   │ 13    │   │        │
└───────────┴───┴───┴──────┴──────┴──────┴──────┴───────┴───┴────────┘"#,
        "Ch 7 Example 15 after 2 complete iterations (TwoStageAr# should be gone)"
    );

    tableau.do_iteration()?;
    assert_eq!(
        tableau.to_string(),
        r#"
┌───────────┬───┬───┬────┬──────┬──────┬──────┬───────┬───┬────────┐
│ Basic var │ x │ y │ z  │ sl#0 │ su#0 │ su#1 │ Value │ θ │ Row op │
├───────────┼───┼───┼────┼──────┼──────┼──────┼───────┼───┼────────┤
│ sl#0      │ 0 │ 0 │ 2  │ 1    │ 0    │ 1    │ 2     │   │        │
├───────────┼───┼───┼────┼──────┼──────┼──────┼───────┼───┼────────┤
│ x         │ 1 │ 1 │ 0  │ 0    │ 0    │ -1   │ 8     │   │        │
├───────────┼───┼───┼────┼──────┼──────┼──────┼───────┼───┼────────┤
│ su#0      │ 0 │ 5 │ -1 │ 0    │ 1    │ -2   │ 11    │   │        │
├───────────┼───┼───┼────┼──────┼──────┼──────┼───────┼───┼────────┤
│ ObjFunc#  │ 0 │ 5 │ -1 │ 0    │ 0    │ -3   │ 24    │   │        │
└───────────┴───┴───┴────┴──────┴──────┴──────┴───────┴───┴────────┘"#,
        "Ch 7 Example 15 after 3 complete iterations"
    );

    tableau.do_iteration()?;
    assert_eq!(
        tableau.to_string(),
        r#"
┌───────────┬───┬───┬───┬──────┬──────┬──────┬───────┬───┬────────┐
│ Basic var │ x │ y │ z │ sl#0 │ su#0 │ su#1 │ Value │ θ │ Row op │
├───────────┼───┼───┼───┼──────┼──────┼──────┼───────┼───┼────────┤
│ su#1      │ 0 │ 0 │ 2 │ 1    │ 0    │ 1    │ 2     │   │        │
├───────────┼───┼───┼───┼──────┼──────┼──────┼───────┼───┼────────┤
│ x         │ 1 │ 1 │ 2 │ 1    │ 0    │ 0    │ 10    │   │        │
├───────────┼───┼───┼───┼──────┼──────┼──────┼───────┼───┼────────┤
│ su#0      │ 0 │ 5 │ 3 │ 2    │ 1    │ 0    │ 15    │   │        │
├───────────┼───┼───┼───┼──────┼──────┼──────┼───────┼───┼────────┤
│ ObjFunc#  │ 0 │ 5 │ 5 │ 3    │ 0    │ 0    │ 30    │   │        │
└───────────┴───┴───┴───┴──────┴──────┴──────┴───────┴───┴────────┘"#,
        "Ch 7 Example 15 after 4 complete iterations"
    );

    Ok(())
}
