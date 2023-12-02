use crate::model::{
    CraftingIssueType, CraftingState, CraftingStep, InfallibleStep, PlayerStats, Recipe, StepResult,
};
use derive_more::Constructor;

#[derive(Constructor)]
pub struct BasicTouch {
    potency: u16,
    cp_cost: u8,
    durability_cost: u8,
    inner_quiet_stacks: u8,
    prevented_by_waste_not: bool,
}

fn calc_quality_increase(
    stats: &PlayerStats,
    recipe: &Recipe,
    state: &CraftingState,
    potency: u16,
) -> u16 {
    let base_quality = (stats.control * 10) / recipe.rlvl.quality_divider as u16 + 35;
    let base_quality_modified_by_level =
        (base_quality as f32 * recipe.rlvl.quality_modifier as f32 * 0.01f32) as u32;

    let mut buff_modifier: f32 = 1.0;
    buff_modifier += 0.1 * state.inner_quiet_stacks as f32;

    let mut buff_modifier_multiplier = 1.0;
    if state.innovation_stacks > 0 {
        buff_modifier_multiplier += 0.5;
    }
    if state.great_strides_stacks > 0 {
        buff_modifier_multiplier += 1.0;
    }
    buff_modifier *= buff_modifier_multiplier;

    let total_quality_increase =
        buff_modifier * (base_quality_modified_by_level * potency as u32) as f32 / 100.;
    total_quality_increase as u16
}

impl CraftingStep for BasicTouch {
    fn apply(&self, state: &CraftingState, stats: &PlayerStats, recipe: &Recipe) -> StepResult {
        if self.prevented_by_waste_not && state.waste_not_stacks > 0 {
            return Err(CraftingIssueType::PreventedByWasteNot);
        }

        Ok(CraftingState {
            quality: state.quality + calc_quality_increase(stats, recipe, state, self.potency),
            inner_quiet_stacks: u8::min(10, state.inner_quiet_stacks + self.inner_quiet_stacks),
            great_strides_stacks: 0,
            touch_combo_stage: 0,
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

pub struct Innovation {}

impl InfallibleStep for Innovation {
    fn apply(
        &self,
        state: &CraftingState,
        _stats: &PlayerStats,
        _recipe: &Recipe,
    ) -> CraftingState {
        CraftingState {
            // see above comment about veneration stacks being 5 instead of 4
            innovation_stacks: 5,
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

pub struct GreatStrides {}
impl InfallibleStep for GreatStrides {
    fn apply(
        &self,
        state: &CraftingState,
        _stats: &PlayerStats,
        _recipe: &Recipe,
    ) -> CraftingState {
        CraftingState {
            great_strides_stacks: 4,
            touch_combo_stage: 0,
            ..*state
        }
    }

    fn cp_cost(&self, _state: &CraftingState) -> u8 {
        32
    }

    fn durability_cost(&self) -> u8 {
        0
    }
}

pub struct ByregotsBlessing {}
impl CraftingStep for ByregotsBlessing {
    fn apply(&self, state: &CraftingState, stats: &PlayerStats, recipe: &Recipe) -> StepResult {
        if state.inner_quiet_stacks == 0 {
            return Err(CraftingIssueType::LackingInnerQuiet);
        }
        let potency = 100 + (state.inner_quiet_stacks as u16 * 20);
        Ok(CraftingState {
            inner_quiet_stacks: 0,
            quality: state.quality + calc_quality_increase(stats, recipe, state, potency),
            touch_combo_stage: 0,
            ..*state
        })
    }

    fn cp_cost(&self, _state: &CraftingState) -> u8 {
        24
    }

    fn durability_cost(&self) -> u8 {
        10
    }
}

#[derive(Constructor)]
pub struct ComboTouch {
    potency: u16,
    regular_cp_cost: u8,
    combo_cp_cost: u8,
    touch_combo_stage_required: Option<u8>,
    touch_combo_stage_applied: u8,
    durability_cost: u8,
}
impl InfallibleStep for ComboTouch {
    fn apply(&self, state: &CraftingState, stats: &PlayerStats, recipe: &Recipe) -> CraftingState {
        let new_touch_combo_stage = if self.touch_combo_stage_required.is_none()
            || self.touch_combo_stage_required == Some(state.touch_combo_stage)
        {
            self.touch_combo_stage_applied
        } else {
            0
        };

        CraftingState {
            inner_quiet_stacks: u8::min(10, state.inner_quiet_stacks + 1),
            quality: state.quality + calc_quality_increase(stats, recipe, state, self.potency),
            touch_combo_stage: new_touch_combo_stage,
            ..*state
        }
    }

    fn cp_cost(&self, state: &CraftingState) -> u8 {
        if self.touch_combo_stage_required == Some(state.touch_combo_stage) {
            self.combo_cp_cost
        } else {
            self.regular_cp_cost
        }
    }

    fn durability_cost(&self) -> u8 {
        self.durability_cost
    }
}

#[cfg(test)]
mod tests {
    use crate::presets::Presets as p;
    use crate::simulator::Simulator as s;

    #[test]
    fn basic_touch_1() {
        let final_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            p::rlvl640_gear(),
            &["Basic Touch"],
        )
        .final_state;

        assert_eq!(0, final_state.progress);
        assert_eq!(60, final_state.durability);
        assert_eq!(247, final_state.quality);
        assert_eq!(622 - 18, final_state.cp);
    }

    #[test]
    fn basic_touch_2() {
        let final_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            p::rlvl640_gear(),
            &["Basic Touch", "Basic Touch"],
        )
        .final_state;

        // each Basic Touch adds an inner quiet stack, so the next touch action will be stronger
        assert_eq!(247 + 271, final_state.quality);
    }

    #[test]
    fn basic_touch_caps_at_10_inner_quiet_stacks() {
        let final_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            p::rlvl640_gear(),
            &["Basic Touch"; 12],
        )
        .final_state;

        // each Basic Touch adds an inner quiet stack, so the next touch action will be stronger
        assert_eq!(
            247 + 271 + 296 + 321 + 345 + 370 + 395 + 419 + 444 + 469 + (2 * 494),
            final_state.quality
        );
    }

    #[test]
    fn innovation_buffs_basic_touch() {
        let final_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            p::rlvl640_gear(),
            &[
                "Innovation",
                "Basic Touch",
                "Basic Touch",
                "Basic Touch",
                "Basic Touch",
                "Basic Touch",
            ],
        )
        .final_state;

        assert_eq!(
            // first four touches get buffed by innovation and inner quiet
            370 + 407 + 444 + 481
            // fifth touch only gets the inner quiet stacks            
             + 345,
            final_state.quality
        );
    }

    #[test]
    fn great_strides_buffs_basic_touch() {
        let final_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            p::rlvl640_gear(),
            &["Great Strides", "Basic Touch"],
        )
        .final_state;

        assert_eq!(247 * 2, final_state.quality);
    }

    #[test]
    fn great_strides_buff_expires_after_3_turns() {
        let final_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            p::rlvl640_gear(),
            &[
                "Great Strides",
                "Observe",
                "Observe",
                "Observe",
                "Basic Touch",
            ],
        )
        .final_state;

        assert_eq!(247 + 0, final_state.quality);
    }

    #[test]
    fn byregots_blessing() {
        let final_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            p::rlvl640_gear(),
            &["Basic Touch", "Great Strides", "Byregot's Blessing"],
        )
        .final_state;

        assert_eq!(0, final_state.inner_quiet_stacks);
        assert_eq!(247 + 652, final_state.quality);
        assert_eq!(50, final_state.durability);
    }

    #[test]
    fn advanced_touch_costs_46_cp_if_not_comboed() {
        let final_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            p::rlvl640_gear(),
            &["Advanced Touch"],
        )
        .final_state;

        assert_eq!(622 - 46, final_state.cp);
    }

    #[test]
    fn standard_touch_costs_32_cp_if_not_comboed() {
        let final_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            p::rlvl640_gear(),
            &["Standard Touch"],
        )
        .final_state;

        assert_eq!(622 - 32, final_state.cp);
    }

    #[test]
    fn standard_touch_by_itself_isnt_enough_to_combo_advanced_touch() {
        let final_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            p::rlvl640_gear(),
            &["Standard Touch", "Advanced Touch"],
        )
        .final_state;

        // the tooltip just says "combo action: standard touch" but it
        // seems to require that standard touch was also combo'd from basic touch
        assert_eq!(622 - 32 - 46, final_state.cp);
    }

    #[test]
    fn basic_standard_advanced_touch_combo_works() {
        let final_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            p::rlvl640_gear(),
            &["Basic Touch", "Standard Touch", "Advanced Touch"],
        )
        .final_state;

        assert_eq!(622 - 18 - 18 - 18, final_state.cp);
    }

    #[test]
    fn multiple_standard_touches_break_the_combo() {
        let final_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            p::rlvl640_gear(),
            &[
                "Basic Touch",
                "Standard Touch",
                "Standard Touch",
                "Advanced Touch",
            ],
        )
        .final_state;

        assert_eq!(622 - 18 - 18 - 32 - 46, final_state.cp);
    }

    #[test]
    fn further_advanced_touches_dont_combo() {
        let final_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            p::rlvl640_gear(),
            &[
                "Basic Touch",
                "Standard Touch",
                "Advanced Touch",
                "Advanced Touch",
            ],
        )
        .final_state;

        assert_eq!(622 - 18 - 18 - 18 - 46, final_state.cp);
    }

    #[test]
    fn other_touches_break_the_combo() {
        let final_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            p::rlvl640_gear(),
            &[
                "Basic Touch",
                "Standard Touch",
                "Prudent Touch",
                "Advanced Touch",
            ],
        )
        .final_state;

        assert_eq!(622 - 18 - 18 - 25 - 46, final_state.cp);
    }

    #[test]
    fn buff_actions_break_the_combo() {
        let final_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            p::rlvl640_gear(),
            &[
                "Basic Touch",
                "Standard Touch",
                "Innovation",
                "Advanced Touch",
            ],
        )
        .final_state;

        assert_eq!(622 - 18 - 18 - 18 - 46, final_state.cp);
    }

    // TODO: pretty much every action except basic/standard/advanced touch should break the combo -
    // is there a way we can reliably test them all, or move the logic somehow so that resetting
    // the combo is the default behavior?

    #[test]
    fn prudent_touch_not_allowed_during_waste_not() {
        let final_state = s::run_steps(
            p::l90_player(),
            p::rlvl640_gear(),
            &["Waste Not", "Prudent Touch"],
        )
        .final_state;

        assert_eq!(0, final_state.quality);
        assert_eq!(70, final_state.durability);
        assert_eq!(500 - 56, final_state.cp);
    }

    #[test]
    fn preparatory_touch_increases_inner_quiet_by_2() {
        let final_state = s::run_steps(
            p::l90_player(),
            p::rlvl640_gear(),
            &["Preparatory Touch", "Preparatory Touch"],
        )
        .final_state;

        assert_eq!(4, final_state.inner_quiet_stacks);
    }
}
