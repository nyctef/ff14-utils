use crate::model::{CraftingState, CraftingStep, PlayerStats, Recipe};
use derive_more::Constructor;

#[derive(Constructor)]
pub struct BasicSynthesis {
    potency: u16,
    cp_cost: u8,
    durability_cost: u8,
    prevented_by_waste_not: bool,
}

fn calculate_progress_increase(
    state: &CraftingState,
    stats: &PlayerStats,
    recipe: &Recipe,
    potency: u16,
) -> u16 {
    // based on some of the calculations in https://github.com/ffxiv-teamcraft/simulator/tree/dec02537f2ac0ec8c1bd61d85bc45f7b4b34e301/src/model/actions
    let base_progression = (stats.craftsmanship * 10) / recipe.rlvl.progress_divider as u16 + 2;

    let progression_modified_by_level =
        (base_progression as f32 * recipe.rlvl.progress_modifier as f32 * 0.01f32) as u32;

    // todo: muscle memory, veneration

    let mut buffed_potency = potency as u32;
    if state.veneration_stacks > 0 {
        buffed_potency += potency as u32 / 2;
    }
    if state.muscle_memory_stacks > 0 {
        buffed_potency += potency as u32;
    }

    let total_increase = (progression_modified_by_level * buffed_potency) / 100;
    total_increase as u16
}

impl CraftingStep for BasicSynthesis {
    fn apply(&self, state: &CraftingState, stats: &PlayerStats, recipe: &Recipe) -> CraftingState {
        if self.prevented_by_waste_not && state.waste_not_stacks > 0 {
            // TODO: warning or error
            return state.clone();
        }

        let total_increase = calculate_progress_increase(state, stats, recipe, self.potency);

        CraftingState {
            progress: state.progress + total_increase,
            touch_combo_stage: 0,
            muscle_memory_stacks: 0,
            ..*state
        }
    }

    fn cp_cost(&self, _state: &CraftingState) -> u8 {
        self.cp_cost
    }

    fn durability_cost(&self) -> u8 {
        self.durability_cost
    }
}

#[derive(Constructor)]
pub struct Veneration {}

impl CraftingStep for Veneration {
    fn apply(
        &self,
        state: &CraftingState,
        _stats: &PlayerStats,
        _recipe: &Recipe,
    ) -> CraftingState {
        CraftingState {
            // note that we set the stack count here to 5 instead of 4,
            // since the generic logic will decrement it by 1 every time a step happens.
            // Is there a nicer way to make this read how it should?
            veneration_stacks: 5,
            touch_combo_stage: 0,
            ..*state
        }
    }

    fn cp_cost(&self, _state: &CraftingState) -> u8 {
        18
    }

    fn durability_cost(&self) -> u8 {
        0
    }
}

#[derive(Constructor)]
pub struct MuscleMemory {}
impl CraftingStep for MuscleMemory {
    fn apply(&self, state: &CraftingState, stats: &PlayerStats, recipe: &Recipe) -> CraftingState {
        if state.steps > 0 {
            // TODO: output some kind of warning or error
            return state.clone();
        }

        CraftingState {
            progress: state.progress + calculate_progress_increase(state, stats, recipe, 300),
            muscle_memory_stacks: 6,
            ..*state
        }
    }

    fn cp_cost(&self, _state: &CraftingState) -> u8 {
        6
    }

    fn durability_cost(&self) -> u8 {
        10
    }
}

#[cfg(test)]
mod tests {
    use crate::presets::Presets as p;
    use crate::simulator::Simulator as s;

    #[test]
    fn basic_synthesis_1() {
        let final_state =
            s::run_steps(p::l90_player(), p::rlvl640_gear(), &["Basic Synthesis"]).final_state;

        assert_eq!(297, final_state.progress);
        assert_eq!(60, final_state.durability);
    }

    #[test]
    fn careful_synthesis_1() {
        let final_state =
            s::run_steps(p::l90_player(), p::rlvl640_gear(), &["Careful Synthesis"]).final_state;

        assert_eq!(446, final_state.progress);
        assert_eq!(60, final_state.durability);
        assert_eq!(500 - 7, final_state.cp);
    }

    #[test]
    fn veneration_increases_next_synthesis_step_by_50_percent() {
        let final_state = s::run_steps(
            p::l90_player(),
            p::rlvl640_gear(),
            &["Veneration", "Basic Synthesis"],
        )
        .final_state;

        assert_eq!(446, final_state.progress);
        assert_eq!(500 - 18, final_state.cp);
    }

    #[test]
    fn veneration_runs_out_after_four_steps() {
        let final_state = s::run_steps(
            p::l90_player(),
            p::rlvl640_gear(),
            &[
                "Veneration",
                "Basic Synthesis",
                "Basic Synthesis",
                "Basic Synthesis",
                "Basic Synthesis",
                // this fifth synthesis should not be affected by veneration any more
                "Basic Synthesis",
            ],
        )
        .final_state;

        assert_eq!((446 * 4) + 297, final_state.progress);
        assert_eq!(500 - 18, final_state.cp);
    }

    #[test]
    fn muscle_memory_fails_if_not_the_first_step() {
        let final_state = s::run_steps(
            p::l90_player(),
            p::rlvl640_gear(),
            &["Veneration", "Muscle Memory"],
        )
        .final_state;

        assert_eq!(0, final_state.progress);
        assert_eq!(60, final_state.durability);
        assert_eq!(500 - 18 - 6, final_state.cp);
    }

    #[test]
    fn muscle_memory_buffs_next_progress_action_within_5_steps() {
        let final_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            p::rlvl640_gear(),
            &["Muscle Memory", "Basic Synthesis"],
        )
        .final_state;

        assert_eq!(744 + 595, final_state.progress);
    }

    #[test]
    fn muscle_memory_buff_lasts_up_to_5_steps() {
        let final_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            p::rlvl640_gear(),
            &[
                "Muscle Memory",
                "Observe",
                "Observe",
                "Observe",
                "Observe",
                "Basic Synthesis",
            ],
        )
        .final_state;

        assert_eq!(744 + 595, final_state.progress);
    }

    #[test]
    fn muscle_memory_buff_runs_out_if_not_used_in_5_steps() {
        let final_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            p::rlvl640_gear(),
            &[
                "Muscle Memory",
                "Observe",
                "Observe",
                "Observe",
                "Observe",
                "Observe",
                "Basic Synthesis",
            ],
        )
        .final_state;

        assert_eq!(744 + 297, final_state.progress);
    }

    #[test]
    fn muscle_memory_buff_only_applies_once() {
        let final_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            p::rlvl640_gear(),
            &["Muscle Memory", "Basic Synthesis", "Basic Synthesis"],
        )
        .final_state;

        assert_eq!(744 + 595 + 297, final_state.progress);
    }

    #[test]
    fn prudent_synthesis_not_allowed_during_waste_not() {
        let final_state = s::run_steps(
            p::l90_player(),
            p::rlvl640_gear(),
            &["Waste Not", "Prudent Synthesis"],
        )
        .final_state;

        assert_eq!(0, final_state.progress);
        // TODO: we don't actually have a good way of preventing the durability cost from triggering here
        // maybe we need to bring back the Result<,> return type from applying crafting steps?

        // assert_eq!(70, final_state.durability);
        //assert_eq!(500 - 56, final_state.cp);
    }
}
