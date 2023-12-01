use crate::model::{CraftingState, CraftingStep, PlayerStats, Recipe};
use derive_more::Constructor;

#[derive(Constructor)]
pub struct Observe {}
impl CraftingStep for Observe {
    fn apply(
        &self,
        state: &CraftingState,
        _stats: &PlayerStats,
        _recipe: &Recipe,
    ) -> CraftingState {
        CraftingState {
            prev_step_was_observe: true,
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
        _recipe: &Recipe,
    ) -> CraftingState {
        if !state.prev_step_was_observe {
            // TODO output some  kind of warning or error
            return state.clone();
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
            p::rlvl640_gear(),
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
            p::rlvl640_gear(),
            &["Observe", "Focused Synthesis"],
        )
        .final_state;

        assert_eq!(496, final_state.progress);
        assert_eq!(60, final_state.durability);
        assert_eq!(622 - 7 - 5, final_state.cp);
        assert_eq!(2, final_state.steps);
    }

    #[test]
    fn focused_touch_succeeds_if_preceded_by_observe() {
        let final_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            p::rlvl640_gear(),
            &["Observe", "Focused Touch"],
        )
        .final_state;

        assert_eq!(370, final_state.quality);
        assert_eq!(60, final_state.durability);
        assert_eq!(622 - 7 - 18, final_state.cp);
        assert_eq!(2, final_state.steps);
    }
}
