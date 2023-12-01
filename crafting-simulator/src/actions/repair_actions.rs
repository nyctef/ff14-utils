use crate::model::{CraftingState, CraftingStep, PlayerStats, Recipe};

pub struct Manipulation {}
impl CraftingStep for Manipulation {
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

#[cfg(test)]
mod tests {
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
        todo!();
    }
}
