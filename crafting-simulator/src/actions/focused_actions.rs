use crate::model::{
    CraftingIssueType, CraftingState, CraftingStep, InfallibleStep, PlayerStats, SimulatorRecipe,
    StepResult,
};
use derive_more::Constructor;

#[derive(Constructor)]
pub struct Observe {}
impl InfallibleStep for Observe {
    fn apply(
        &self,
        state: &CraftingState,
        _stats: &PlayerStats,
        _recipe: &SimulatorRecipe,
    ) -> CraftingState {
        CraftingState {
            observe_stacks: 2,
            touch_combo_stage: 0,
            ..*state
        }
    }

    fn cp_cost(&self, _state: &CraftingState) -> u8 {
        7
    }

    fn durability_cost(&self) -> u8 {
        0
    }
}

#[derive(Constructor)]
pub struct FocusedStep {
    underlying: Box<dyn CraftingStep>,
}
impl CraftingStep for FocusedStep {
    fn apply(
        &self,
        state: &CraftingState,
        _stats: &PlayerStats,
        _recipe: &SimulatorRecipe,
    ) -> StepResult {
        if state.observe_stacks <= 0 {
            return Err(CraftingIssueType::ChanceBasedAction);
        }

        self.underlying.apply(state, _stats, _recipe)
    }

    fn cp_cost(&self, state: &CraftingState) -> u8 {
        self.underlying.cp_cost(state)
    }

    fn durability_cost(&self) -> u8 {
        self.underlying.durability_cost()
    }
}

#[cfg(test)]
mod tests {
    use crate::presets::Presets as p;
    use crate::simulator::Simulator as s;

    #[test]
    fn focused_synthesis_fails_if_not_preceded_by_observe() {
        // technically it has a 50% success rate, but we don't want to rely on that in a simulator
        let final_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            &p::l90_4star_gear(),
            &["Focused Synthesis"],
        )
        .final_state;

        assert_eq!(0, final_state.progress);
        assert_eq!(60, final_state.durability);
    }

    #[test]
    fn focused_synthesis_succeeds_if_preceded_by_observe() {
        let final_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            &p::l90_4star_gear(),
            &["Observe", "Focused Synthesis"],
        )
        .final_state;

        assert_eq!(496, final_state.progress);
        assert_eq!(60, final_state.durability);
        assert_eq!(622 - 7 - 5, final_state.cp);
        assert_eq!(2, final_state.steps);
    }

    #[test]
    fn focused_synthesis_fails_if_another_step_comes_between_observe_and_it() {
        // technically it has a 50% success rate, but we don't want to rely on that in a simulator
        let final_state = s::run_steps(
            p::baseline_player(),
            &p::baseline_recipe(1000, 70, 1000),
            &["Observe", "Veneration", "Focused Synthesis"],
        )
        .final_state;

        assert_eq!(0, final_state.progress);
        assert_eq!(60, final_state.durability);
    }

    #[test]
    fn focused_touch_succeeds_if_preceded_by_observe() {
        let final_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            &p::l90_4star_gear(),
            &["Observe", "Focused Touch"],
        )
        .final_state;

        assert_eq!(370, final_state.quality);
        assert_eq!(60, final_state.durability);
        assert_eq!(622 - 7 - 18, final_state.cp);
        assert_eq!(2, final_state.steps);
    }
}
