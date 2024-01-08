use crate::{actions::Actions, model::*};
use color_eyre::eyre::{eyre, Report, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use std::{collections::HashMap, ops::ControlFlow};

pub struct Simulator;

impl Simulator {
    pub fn run_steps(
        player: PlayerStats,
        recipe: &Recipe,
        steps: &[&'static str],
    ) -> CraftingReport {
        let initial_state = CraftingState::initial(&player, recipe);
        let actions = Actions::make_action_lookup();
        let steps = parse_steps(steps, &actions).expect("steps should be valid");
        let fold_result = steps.into_iter().try_fold(
            (
                Vec::<&'static str>::new(),
                Vec::<CraftingIssue>::new(),
                initial_state,
            ),
            |(step_log, prev_issues, prev_state), (name, step)| {
                let mut next = prev_state;
                let mut next_issues = prev_issues;
                let mut step_log = step_log;

                let cp_cost = step.cp_cost(&next) as i16;
                let durability_cost_divider = if next.waste_not_stacks > 0 { 2 } else { 1 };
                let durability_cost = (step.durability_cost() / durability_cost_divider) as i16;

                match step.apply(&next, &player, recipe) {
                    Ok(step_result) => {
                        // step applied correctly, so we take its updated state and pay its cp/durability cost
                        next = step_result;
                        next.cp -= cp_cost;
                        next.durability -= durability_cost;
                        step_log.push(name);
                    }
                    Err(issue) if issue == CraftingIssueType::ChanceBasedAction => {
                        // we assume chance based actions fail, but we still pay the durability/cp cost
                        next.cp -= cp_cost;
                        next.durability -= durability_cost;
                        next_issues.push(CraftingIssue::new(issue, next.steps))
                    }
                    Err(other_issue) => {
                        // step errored, but didn't break the whole craft
                        // (are there any cases here where we need to check issue.is_fatal()?)
                        next_issues.push(CraftingIssue::new(other_issue, next.steps))
                    }
                }

                if next.durability <= 0 && next.progress < recipe.difficulty {
                    // craft failed
                    next_issues.push(CraftingIssue::new(
                        CraftingIssueType::DurabilityFailed,
                        next.steps,
                    ));
                    return ControlFlow::Break((step_log, next_issues, next));
                }

                if next.cp < 0 {
                    // this isn't technically an outright error, but the sequence is unlikely to work any more.
                    // TODO: prevent the previous step which didn't have enough CP from running
                    // TODO: try continuing the craft and just prevent any future steps with insufficient CP
                    next_issues.push(CraftingIssue::new(CraftingIssueType::OutOfCP, next.steps));
                    return ControlFlow::Break((step_log, next_issues, next));
                }

                if next.progress >= recipe.difficulty {
                    // craft succeeded
                    return ControlFlow::Break((step_log, next_issues, next));
                }

                if next.manipulation_stacks > 0 && next.manipulation_delay == 0 {
                    next.durability = i16::min(next.durability + 5, recipe.durability as i16);
                }

                next.veneration_stacks = next.veneration_stacks.saturating_sub(step.num_steps());
                next.innovation_stacks = next.innovation_stacks.saturating_sub(step.num_steps());
                next.muscle_memory_stacks =
                    next.muscle_memory_stacks.saturating_sub(step.num_steps());
                next.great_strides_stacks =
                    next.great_strides_stacks.saturating_sub(step.num_steps());
                next.manipulation_stacks =
                    next.manipulation_stacks.saturating_sub(step.num_steps());
                next.manipulation_delay = next.manipulation_delay.saturating_sub(step.num_steps());
                next.waste_not_stacks = next.waste_not_stacks.saturating_sub(step.num_steps());
                next.observe_stacks = next.observe_stacks.saturating_sub(step.num_steps());
                next.steps += step.num_steps();

                ControlFlow::Continue((step_log, next_issues, next))
            },
        );

        let (step_log, issues, final_state) = either_controlflow(fold_result);

        let status = match (
            final_state.progress >= recipe.difficulty,
            issues.iter().any(|i| i.issue_type.is_fatal()),
        ) {
            (true, false) => CraftStatus::Success,
            (_, true) => CraftStatus::Failure,
            (_, _) => CraftStatus::Incomplete,
        };

        CraftingReport {
            step_log,
            final_state,
            issues,
            status,
        }
    }
}

lazy_static! {
    static ref MACRO_AC_LINE: Regex =
        RegexBuilder::new(r#"

                          # option /act
                          (/ac)?
                          # any leading whitespace
                          \s*
                          # optional starting quote
                          "?
                          # word boundary
                          \b
                          # the name itself
                          (?P<name>[a-z'\ ]+)
                          # word boundary
                          \b
                          # optional closing quote
                          "?
                          # any number of <wait.2> or <se.3> etc
                          (\s*<.*>)*

                          "#)
            .case_insensitive(true)
            .ignore_whitespace(true)
            .build()
            .unwrap();
}

fn parse_steps<'a>(
    steps: &[&'static str],
    actions: &'a HashMap<&str, Box<dyn CraftingStep>>,
) -> Result<Vec<(&'static str, &'a Box<dyn CraftingStep>)>> {
    let mut result = vec![];
    for &step in steps {
        if step.starts_with("/echo") {
            // /echo lines don't cause a step to happen
            continue;
        }
        if step.trim().is_empty() {
            // empty lines don't cause a step to happen
            continue;
        }

        if let Some(captures) = MACRO_AC_LINE.captures(step) {
            let step_name = captures.name("name").unwrap().as_str();
            let action = actions
                .get(step_name)
                .ok_or_else(|| eyre!("Couldn't find action named: <{}>", step_name))?;
            result.push((step_name, action));
            continue;
        } else {
            return Err(eyre!("Failed to parse step: {}", step));
        }
    }
    Ok(result)
}

fn either_controlflow<T>(input: ControlFlow<T, T>) -> T {
    // TODO: is there a builtin method for this?
    match input {
        ControlFlow::Continue(c) => c,
        ControlFlow::Break(b) => b,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{CraftStatus, CraftingIssueType, PlayerStats};
    use crate::presets::Presets as p;
    use crate::simulator::Simulator as s;
    use itertools::Itertools;

    // for simplicity we'll create a few baseline setups which should mean
    // that the actual progress/quality numbers are just equal to the potency
    // values of the actions themselves

    #[test]
    fn baseline_synthesis_is_potency() {
        let final_state = s::run_steps(
            p::baseline_player(),
            &p::baseline_recipe(1000, 70, 1000),
            &["Basic Synthesis"],
        )
        .final_state;

        assert_eq!(120, final_state.progress);
    }

    #[test]
    fn baseline_touch_is_potency() {
        let final_state = s::run_steps(
            p::baseline_player(),
            &p::baseline_recipe(1000, 70, 1000),
            &["Basic Touch"],
        )
        .final_state;

        assert_eq!(100, final_state.quality);
    }

    #[test]
    fn craft_is_successful_if_progress_is_reached_before_durability_fails() {
        let report = s::run_steps(
            p::baseline_player(),
            // difficulty of 300 is easy to hit here
            &p::baseline_recipe(300, 70, 1000),
            &["Basic Synthesis", "Basic Synthesis", "Basic Synthesis"],
        );

        assert_eq!(360, report.final_state.progress);
        assert_eq!(40, report.final_state.durability);
        assert_eq!(CraftStatus::Success, report.status);
    }

    #[test]
    fn craft_is_not_successful_or_failed_if_neither_progress_or_durability_condition_is_triggered()
    {
        let report = s::run_steps(
            p::baseline_player(),
            &p::baseline_recipe(300, 70, 1000),
            // just a single synthesis is insufficient to succeed or fail the craft
            &["Basic Synthesis"],
        );

        assert_eq!(CraftStatus::Incomplete, report.status);
    }

    #[test]
    fn craft_is_failed_if_durability_hit_zero_before_progress_is_reached() {
        let report = s::run_steps(
            p::baseline_player(),
            // difficulty of 1000 is hard to hit here
            &p::baseline_recipe(1000, 40, 1000),
            &[
                "Basic Synthesis",
                "Basic Synthesis",
                "Basic Synthesis",
                "Basic Synthesis",
            ],
        );

        assert_eq!(CraftStatus::Failure, report.status);
        let single_issue = report.issues.into_iter().exactly_one().unwrap();
        assert_eq!(CraftingIssueType::DurabilityFailed, single_issue.issue_type);
        // step indexes here are 0-indexed
        assert_eq!(3, single_issue.step_index);
    }

    #[test]
    fn craft_is_succeeded_if_progress_is_met_and_durability_runs_out_in_same_step() {
        let report = s::run_steps(
            p::baseline_player(),
            // 10 durability and 120 progress means one basic synth triggers both conditions
            &p::baseline_recipe(120, 10, 1000),
            &["Basic Synthesis"],
        );

        assert_eq!(CraftStatus::Success, report.status);
    }

    #[test]
    fn craft_fails_if_cp_runs_out() {
        let report = s::run_steps(
            // very little CP available
            PlayerStats::level_90(4000, 4000, 10),
            &p::baseline_recipe(1000, 40, 1000),
            &["Groundwork"],
        );

        // technically not having enough CP just results in one action failing
        // and the rest of the craft macro continuing, but it's almost always
        // fatal in practice.

        assert_eq!(CraftStatus::Failure, report.status);
        let single_issue = report.issues.into_iter().exactly_one().unwrap();
        assert_eq!(CraftingIssueType::OutOfCP, single_issue.issue_type);
        assert_eq!(0, single_issue.step_index);
    }

    #[test]
    fn reaching_zero_cp_does_not_fail_the_craft() {
        // it just prevents you from using any cp-consuming actions
        // (at least according to teamcraft - haven't tested this one personally)
        let report = s::run_steps(
            // very little CP available
            PlayerStats::level_90(4000, 4000, 18),
            &p::baseline_recipe(480, 40, 1000),
            // this is just enough to exactly hit 480 potency
            &["Groundwork", "Basic Synthesis"],
        );

        assert_eq!(CraftStatus::Success, report.status);
        assert!(report.issues.is_empty());
    }

    #[test]
    fn other_issues_reported_by_actions_are_listed_but_are_not_fatal() {
        let report = s::run_steps(
            p::baseline_player(),
            &p::baseline_recipe(480, 70, 1000),
            &["Groundwork", "Byregot's Blessing", "Basic Synthesis"],
        );

        assert_eq!(CraftStatus::Success, report.status);
        let single_issue = report.issues.into_iter().exactly_one().unwrap();
        assert_eq!(
            CraftingIssueType::LackingInnerQuiet,
            single_issue.issue_type
        );
    }

    #[test]
    fn cp_and_durability_can_end_up_negative() {
        // since we potentially want to give some preference to sequences that almost don't run out of cp
        let report = s::run_steps(
            // very little CP available
            PlayerStats::level_90(4000, 4000, 10),
            &p::baseline_recipe(480, 10, 1000),
            &["Groundwork"],
        );

        assert_eq!(-10, report.final_state.durability);
        assert_eq!(-8, report.final_state.cp);
    }

    #[test]
    fn quality_increases_after_craft_complete_do_not_count() {
        let report = s::run_steps(
            p::baseline_player(),
            &p::baseline_recipe(360, 100, 1000),
            // the first Groundwork completes the crafts, so the extra Basic Touchs don't apply
            &["Groundwork", "Basic Touch", "Basic Touch"],
        );

        assert_eq!(CraftStatus::Success, report.status);
        assert_eq!(360, report.final_state.progress);
        assert_eq!(0, report.final_state.quality);
    }

    #[test]
    fn progress_increases_after_craft_failure_do_not_count() {
        let report = s::run_steps(
            p::baseline_player(),
            &p::baseline_recipe(500, 10, 1000),
            // We run out of durability after the first Groundwork - the second would complete the craft, but it doesn't get to run
            &["Groundwork", "Groundwork"],
        );

        assert_eq!(CraftStatus::Failure, report.status);
        assert_eq!(180, report.final_state.progress);
    }

    #[test]
    fn test_parse_steps() {
        let actions = crate::actions::Actions::make_action_lookup();
        let steps = parse_steps(
            &[
                "Basic Synthesis",
                r#"/ac "Byregot's Blessing""#,
                r#"/ac "Basic Synthesis" <wait.3>"#,
                r#"/ac Manipulation <wait.2>"#,
            ],
            &actions,
        )
        .unwrap();

        assert_eq!(4, steps.len());
        assert_eq!("Basic Synthesis", steps[0].0);
        assert_eq!("Byregot's Blessing", steps[1].0);
        assert_eq!("Basic Synthesis", steps[2].0);
        assert_eq!("Manipulation", steps[3].0);
    }
}
