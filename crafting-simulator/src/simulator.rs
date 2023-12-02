use crate::{actions::Actions, model::*};
use itertools::Itertools;

pub struct Simulator;

impl Simulator {
    pub fn run_steps(player: PlayerStats, recipe: Recipe, steps: &[&str]) -> CraftingReport {
        let initial_state = CraftingState::initial(&player, &recipe);
        let actions = Actions::make_action_lookup();
        let steps: Vec<_> = steps
            .iter()
            .map(|name| {
                actions
                    .get(name)
                    .ok_or_else(|| format!("Unknown action: {}", name))
            })
            .try_collect()
            .unwrap();
        let (issues, final_state) = steps.iter().fold(
            (Vec::<CraftingIssue>::new(), initial_state),
            |(prev_issues, prev_state), step| {
                let mut next = prev_state;
                let mut next_issues = prev_issues;

                next.cp = next.cp.saturating_sub(step.cp_cost(&next) as i16);
                let durability_cost_divider = if next.waste_not_stacks > 0 { 2 } else { 1 };
                next.durability = next
                    .durability
                    .saturating_sub((step.durability_cost() / durability_cost_divider) as i16);

                match step.apply(&next, &player, &recipe) {
                    Ok(step_result) => next = step_result,
                    Err(issue) => next_issues.push(CraftingIssue::new(issue, next.steps)),
                }

                if next.durability <= 0 && next.progress < recipe.difficulty {
                    next_issues.push(CraftingIssue::new(
                        CraftingIssueType::DurabilityFailed,
                        next.steps,
                    ));
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
                next.steps += step.num_steps();
                (next_issues, next)
            },
        );

        let state = match (
            final_state.progress >= recipe.difficulty,
            !issues.is_empty(),
        ) {
            (true, false) => CraftStatus::Success,
            (_, true) => CraftStatus::Failure,
            (_, _) => CraftStatus::Incomplete,
        };

        CraftingReport {
            final_state,
            issues,
            state,
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::model::{CraftStatus, CraftingIssueType};
    use crate::presets::Presets as p;
    use crate::simulator::Simulator as s;

    // for simplicity we'll create a few baseline setups which should mean
    // that the actual progress/quality numbers are just equal to the potency
    // values of the actions themselves

    #[test]
    fn baseline_synthesis_is_potency() {
        let final_state = s::run_steps(
            p::baseline_player(),
            p::baseline_recipe(1000, 70, 1000),
            &["Basic Synthesis"],
        )
        .final_state;

        assert_eq!(120, final_state.progress);
    }

    #[test]
    fn baseline_touch_is_potency() {
        let final_state = s::run_steps(
            p::baseline_player(),
            p::baseline_recipe(1000, 70, 1000),
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
            p::baseline_recipe(300, 70, 1000),
            &["Basic Synthesis", "Basic Synthesis", "Basic Synthesis"],
        );

        assert_eq!(360, report.final_state.progress);
        assert_eq!(40, report.final_state.durability);
        assert_eq!(CraftStatus::Success, report.state);
    }

    #[test]
    fn craft_is_not_successful_or_failed_if_neither_progress_or_durability_condition_is_triggered()
    {
        let report = s::run_steps(
            p::baseline_player(),
            p::baseline_recipe(300, 70, 1000),
            // just a single synthesis is insufficient to succeed or fail the craft
            &["Basic Synthesis"],
        );

        assert_eq!(CraftStatus::Incomplete, report.state);
    }

    #[test]
    fn craft_is_failed_if_durability_hit_zero_before_progress_is_reached() {
        let report = s::run_steps(
            p::baseline_player(),
            // difficulty of 1000 is hard to hit here
            p::baseline_recipe(1000, 40, 1000),
            &[
                "Basic Synthesis",
                "Basic Synthesis",
                "Basic Synthesis",
                "Basic Synthesis",
            ],
        );

        assert_eq!(CraftStatus::Failure, report.state);
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
            p::baseline_recipe(120, 10, 1000),
            &["Basic Synthesis"],
        );

        assert_eq!(CraftStatus::Success, report.state);
    }

    #[test]
    fn craft_fails_if_cp_runs_out() {
        todo!()
    }

    #[test]
    fn other_issues_reported_by_actions_are_listed_but_are_not_fatal() {
        todo!()
    }
}
