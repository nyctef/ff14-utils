use crate::model::{CraftingState, CraftingStep, PlayerStats, Recipe};
use derive_more::Constructor;

#[derive(Constructor)]
pub struct BasicSynthesis {
    potency: u16,
    cp_cost: u8,
    durability_cost: u8,
}

impl CraftingStep for BasicSynthesis {
    fn apply(&self, state: &CraftingState, stats: &PlayerStats, recipe: &Recipe) -> CraftingState {
        // based on some of the calculations in https://github.com/ffxiv-teamcraft/simulator/tree/dec02537f2ac0ec8c1bd61d85bc45f7b4b34e301/src/model/actions
        // ignoring recipe level difference for now
        let base_progression = (stats.craftsmanship * 10) / recipe.rlvl.progress_divider as u16 + 2;

        let progression_modified_by_level =
            (base_progression as f32 * recipe.rlvl.progress_modifier as f32 * 0.01f32) as u16;

        // todo: muscle memory, veneration

        let mut potency = self.potency;
        if state.veneration_stacks > 0 {
            potency += potency / 2;
        }

        let total_increase = (progression_modified_by_level * potency) / 100;

        CraftingState {
            progress: state.progress + total_increase,
            touch_combo_stage: 0,
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
}
