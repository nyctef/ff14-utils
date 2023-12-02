use crate::model::{CraftingState, InfallibleStep, PlayerStats, Recipe};
use derive_more::Constructor;

pub struct Manipulation {}
impl InfallibleStep for Manipulation {
    fn apply(
        &self,
        state: &CraftingState,
        _stats: &PlayerStats,
        _recipe: &Recipe,
    ) -> CraftingState {
        CraftingState {
            manipulation_stacks: 9,
            manipulation_delay: 1,
            ..*state
        }
    }

    fn cp_cost(&self, _state: &CraftingState) -> u8 {
        96
    }

    fn durability_cost(&self) -> u8 {
        0
    }
}

#[derive(Constructor)]
pub struct WasteNot {
    length: u8,
    cp_cost: u8,
}
impl InfallibleStep for WasteNot {
    fn apply(
        &self,
        state: &CraftingState,
        _stats: &PlayerStats,
        _recipe: &Recipe,
    ) -> CraftingState {
        CraftingState {
            waste_not_stacks: self.length + 1,
            ..*state
        }
    }

    fn cp_cost(&self, _state: &CraftingState) -> u8 {
        self.cp_cost
    }

    fn durability_cost(&self) -> u8 {
        0
    }
}

pub struct MastersMend {}
impl InfallibleStep for MastersMend {
    fn apply(&self, state: &CraftingState, _stats: &PlayerStats, recipe: &Recipe) -> CraftingState {
        CraftingState {
            durability: i16::min(recipe.durability as i16, state.durability + 30),
            ..*state
        }
    }

    fn cp_cost(&self, _state: &CraftingState) -> u8 {
        88
    }

    fn durability_cost(&self) -> u8 {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::model::CraftStatus;
    use crate::presets::Presets as p;
    use crate::simulator::Simulator as s;

    #[test]
    fn manipulation_restores_5_durability_after_some_step() {
        let after_one_step = s::run_steps(
            p::l90_player(),
            p::rlvl640_gear(),
            &["Manipulation", "Basic Synthesis"],
        )
        .final_state;

        assert_eq!(65, after_one_step.durability);

        let after_two_steps = s::run_steps(
            p::l90_player(),
            p::rlvl640_gear(),
            &["Manipulation", "Basic Synthesis", "Observe"],
        )
        .final_state;

        assert_eq!(70, after_two_steps.durability);
    }

    #[test]
    fn manipulation_doesnt_start_working_until_after_the_next_step() {
        let after_one_step = s::run_steps(
            p::l90_player(),
            p::rlvl640_gear(),
            &["Basic Synthesis", "Manipulation"],
        )
        .final_state;
        assert_eq!(60, after_one_step.durability);

        let after_two_steps = s::run_steps(
            p::l90_player(),
            p::rlvl640_gear(),
            &["Basic Synthesis", "Manipulation", "Observe"],
        )
        .final_state;
        assert_eq!(65, after_two_steps.durability);
    }

    #[test]
    fn manipulation_cant_increase_durability_above_max() {
        let after_waiting_a_while = s::run_steps(
            p::l90_player(),
            p::rlvl640_gear(),
            &[
                "Basic Synthesis",
                "Manipulation",
                "Observe",
                "Observe",
                "Observe",
            ],
        )
        .final_state;
        assert_eq!(70, after_waiting_a_while.durability);
    }

    #[test]
    fn manipulation_cant_recover_a_craft_if_it_hit_zero_in_the_previous_step() {
        let just_too_late = s::run_steps(
            p::l90_player(),
            p::rlvl640_gear(),
            &[
                "Groundwork",
                "Groundwork",
                "Groundwork",
                // 10 durability left here
                "Manipulation",
                "Basic Synthesis",
                // Basic Synthesis takes 10 durability away before Manipulation can restore it
            ],
        );

        assert_eq!(CraftStatus::Failure, just_too_late.state);
    }

    #[test]
    fn waste_not_reduces_durability_cost_of_next_four_steps_by_50_percent() {
        let final_state = s::run_steps(
            p::l90_player(),
            p::rlvl640_gear(),
            &[
                "Waste Not",
                "Basic Synthesis",
                "Basic Synthesis",
                "Basic Synthesis",
                "Basic Synthesis",
                "Basic Synthesis",
            ],
        )
        .final_state;

        assert_eq!(70 - (4 * 5) - 10, final_state.durability);
    }

    #[test]
    fn masters_mend_restores_a_chunk_of_durability() {
        let final_state = s::run_steps(
            p::l90_player(),
            p::rlvl640_gear(),
            &["Groundwork", "Groundwork", "Master's Mend"],
        )
        .final_state;

        assert_eq!(70 - 20 - 20 + 30, final_state.durability);
    }

    #[test]
    fn masters_mend_cant_increase_durability_above_max() {
        let final_state =
            s::run_steps(p::l90_player(), p::rlvl640_gear(), &["Master's Mend"]).final_state;

        assert_eq!(70, final_state.durability);
    }
}
