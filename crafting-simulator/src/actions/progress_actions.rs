use crate::model::{
    CraftingIssueType, CraftingState, CraftingStep, InfallibleStep, PlayerStats, SimulatorRecipe,
    StepResult,
};
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
    recipe: &SimulatorRecipe,
    potency: u16,
) -> u16 {
    // based on some of the calculations in https://github.com/ffxiv-teamcraft/simulator/tree/dec02537f2ac0ec8c1bd61d85bc45f7b4b34e301/src/model/actions
    let base_progression = (stats.craftsmanship * 10) / recipe.progress_divider as u16 + 2;

    let progression_modified_by_level =
        (base_progression as f32 * recipe.progress_modifier as f32 * 0.01f32) as u32;

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
    fn apply(
        &self,
        state: &CraftingState,
        stats: &PlayerStats,
        recipe: &SimulatorRecipe,
    ) -> StepResult {
        if self.prevented_by_waste_not && state.waste_not_stacks > 0 {
            return Err(CraftingIssueType::PreventedByWasteNot);
        }

        let total_increase = calculate_progress_increase(state, stats, recipe, self.potency);

        Ok(CraftingState {
            progress: state.progress + total_increase,
            touch_combo_stage: 0,
            muscle_memory_stacks: 0,
            ..*state
        })
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

impl InfallibleStep for Veneration {
    fn apply(
        &self,
        state: &CraftingState,
        _stats: &PlayerStats,
        _recipe: &SimulatorRecipe,
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
    fn apply(
        &self,
        state: &CraftingState,
        stats: &PlayerStats,
        recipe: &SimulatorRecipe,
    ) -> StepResult {
        if state.steps > 0 {
            return Err(CraftingIssueType::NotOnFirstStep);
        }

        Ok(CraftingState {
            progress: state.progress + calculate_progress_increase(state, stats, recipe, 300),
            muscle_memory_stacks: 6,
            ..*state
        })
    }

    fn cp_cost(&self, _state: &CraftingState) -> u8 {
        6
    }

    fn durability_cost(&self) -> u8 {
        10
    }
}

pub struct Groundwork {}
impl InfallibleStep for Groundwork {
    fn apply(
        &self,
        state: &CraftingState,
        stats: &PlayerStats,
        recipe: &SimulatorRecipe,
    ) -> CraftingState {
        // todo: some duplication with the regular simulator logic here - not sure if there's a good way around this
        let durability_cost = if state.waste_not_stacks > 0 { 10 } else { 20 };
        let potency = if state.durability >= durability_cost {
            360
        } else {
            180
        };

        let total_increase = calculate_progress_increase(state, stats, recipe, potency);

        CraftingState {
            progress: state.progress + total_increase,
            touch_combo_stage: 0,
            muscle_memory_stacks: 0,
            ..*state
        }
    }

    fn cp_cost(&self, _state: &CraftingState) -> u8 {
        18
    }

    fn durability_cost(&self) -> u8 {
        20
    }
}

pub struct FinalAppraisal {}
impl InfallibleStep for FinalAppraisal {
    fn apply(
        &self,
        state: &CraftingState,
        _stats: &PlayerStats,
        _recipe: &SimulatorRecipe,
    ) -> CraftingState {
        CraftingState {
            final_appraisal_stacks: 6,
            ..*state
        }
    }

    fn cp_cost(&self, _state: &CraftingState) -> u8 {
        1
    }

    fn durability_cost(&self) -> u8 {
        0
    }

    fn num_steps(&self) -> u8 {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::presets::Presets as p;
    use crate::simulator::Simulator as s;

    #[test]
    fn basic_synthesis_1() {
        let final_state = s::run_steps(
            p::l90_player(),
            &p::without_required_stats(p::l90_4star_gear()),
            &["Basic Synthesis"],
        )
        .final_state;

        assert_eq!(297, final_state.progress);
        assert_eq!(60, final_state.durability);
    }

    #[test]
    fn careful_synthesis_1() {
        let final_state = s::run_steps(
            p::l90_player(),
            &p::without_required_stats(p::l90_4star_gear()),
            &["Careful Synthesis"],
        )
        .final_state;

        assert_eq!(446, final_state.progress);
        assert_eq!(60, final_state.durability);
        assert_eq!(500 - 7, final_state.cp);
    }

    #[test]
    fn veneration_increases_next_synthesis_step_by_50_percent() {
        let final_state = s::run_steps(
            p::l90_player(),
            &p::without_required_stats(p::l90_4star_gear()),
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
            &p::without_required_stats(p::l90_4star_gear()),
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
            &p::without_required_stats(p::l90_4star_gear()),
            &["Veneration", "Muscle Memory"],
        )
        .final_state;

        assert_eq!(0, final_state.progress);
        assert_eq!(70, final_state.durability);
        assert_eq!(500 - 18, final_state.cp);
    }

    #[test]
    fn muscle_memory_buffs_next_progress_action_within_5_steps() {
        let final_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            &p::l90_4star_gear(),
            &["Muscle Memory", "Basic Synthesis"],
        )
        .final_state;

        assert_eq!(744 + 595, final_state.progress);
    }

    #[test]
    fn muscle_memory_buff_lasts_up_to_5_steps() {
        let final_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            &p::l90_4star_gear(),
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
            &p::l90_4star_gear(),
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
            &p::l90_4star_gear(),
            &["Muscle Memory", "Basic Synthesis", "Basic Synthesis"],
        )
        .final_state;

        assert_eq!(744 + 595 + 297, final_state.progress);
    }

    #[test]
    fn prudent_synthesis_not_allowed_during_waste_not() {
        let final_state = s::run_steps(
            p::l90_player(),
            &p::without_required_stats(p::l90_4star_gear()),
            &["Waste Not", "Prudent Synthesis"],
        )
        .final_state;

        assert_eq!(0, final_state.progress);
        assert_eq!(70, final_state.durability);
        assert_eq!(500 - 56, final_state.cp);
    }

    #[test]
    fn groundwork_only_has_half_effectiveness_if_durability_about_to_run_out() {
        let final_state = s::run_steps(
            p::baseline_player(),
            &p::baseline_recipe(360, 10, 1000),
            &["Groundwork"],
        )
        .final_state;

        // since groundwork costs 20, but we only have 10 durability remaining,
        // it gets 180 potency here instead of 360
        assert_eq!(180, final_state.progress);
    }

    #[test]
    fn waste_not_can_prevent_groundwork_from_being_half_effective() {
        let final_state = s::run_steps(
            p::baseline_player(),
            &p::baseline_recipe(360, 10, 1000),
            &["Waste Not", "Groundwork"],
        )
        .final_state;

        // waste not effectively changes groundwork's durability cost for the
        // purpose of this effect
        assert_eq!(360, final_state.progress);
    }

    #[test]
    fn final_appraisal_prevents_one_progress_step_from_completing_the_craft() {
        let final_state = s::run_steps(
            p::baseline_player(),
            &p::baseline_recipe(360, 80, 1000),
            &["Final Appraisal", "Groundwork"],
        )
        .final_state;

        assert_eq!(
            359, final_state.progress,
            "craft should be 1 progress point away from completion"
        );
        assert_eq!(final_state.cp, 1000 - 18 - 1, "final appraisal costs 1 cp")
    }

    #[test]
    fn final_appraisal_lasts_five_steps() {
        let final_state = s::run_steps(
            p::baseline_player(),
            &p::baseline_recipe(360, 80, 1000),
            &[
                "Final Appraisal",
                "Observe",
                "Observe",
                "Observe",
                "Observe",
                "Observe",
                "Groundwork",
            ],
        )
        .final_state;

        assert_eq!(
            360, final_state.progress,
            "craft should have completed since final appraisal buff expired"
        );
    }

    #[test]
    fn final_appraisal_doesnt_cost_a_step() {
        let final_state = s::run_steps(
            p::baseline_player(),
            &p::baseline_recipe(360, 80, 1000),
            &[
                "Innovation",
                "Final Appraisal",
                "Final Appraisal",
                "Final Appraisal",
            ],
        )
        .final_state;

        assert_eq!(
            4, final_state.innovation_stacks,
            "final appraisal doesn't consume real steps"
        );
    }
}
