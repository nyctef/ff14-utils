use crate::model::{CraftingState, CraftingStep, PlayerStats, Recipe};

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
            durability: state.durability - self.durability_cost as i16,
            cp: state.cp - self.cp_cost as i16,
            ..*state
        }
    }
}

pub struct Veneration {}

impl CraftingStep for Veneration {
    fn apply(
        &self,
        state: &CraftingState,
        _stats: &PlayerStats,
        _recipe: &Recipe,
    ) -> CraftingState {
        CraftingState {
            cp: state.cp - 18,
            // note that we set the stack count here to 5 instead of 4,
            // since the generic logic will decrement it by 1 every time a step happens.
            // Is there a nicer way to make this read how it should?
            veneration_stacks: 5,
            ..*state
        }
    }
}

pub struct BasicTouch {
    potency: u16,
    cp_cost: u8,
    durability_cost: u8,
}

impl CraftingStep for BasicTouch {
    fn apply(&self, state: &CraftingState, stats: &PlayerStats, recipe: &Recipe) -> CraftingState {
        let base_quality = (stats.control * 10) / recipe.rlvl.quality_divider as u16 + 35;
        let base_quality_modified_by_level =
            (base_quality as f32 * recipe.rlvl.quality_modifier as f32 * 0.01f32) as u16;

        let mut buff_modifier: f32 = 1.0;
        buff_modifier += 0.1 * state.inner_quiet_stacks as f32;

        let total_quality_increase =
            buff_modifier * (base_quality_modified_by_level * self.potency) as f32 / 100.;
        CraftingState {
            quality: state.quality + total_quality_increase as u16,
            durability: state.durability - self.durability_cost as i16,
            cp: state.cp - self.cp_cost as i16,
            inner_quiet_stacks: u8::min(10, state.inner_quiet_stacks + 1),
            ..*state
        }
    }
}

pub struct Actions {}
impl Actions {
    pub fn basic_synthesis() -> impl CraftingStep {
        BasicSynthesis {
            potency: 120,
            cp_cost: 0,
            durability_cost: 10,
        }
    }

    pub fn careful_synthesis() -> impl CraftingStep {
        BasicSynthesis {
            potency: 180,
            cp_cost: 7,
            durability_cost: 10,
        }
    }

    pub fn prudent_synthesis() -> impl CraftingStep {
        BasicSynthesis {
            potency: 180,
            cp_cost: 18,
            durability_cost: 5,
        }
    }

    pub fn groundwork() -> impl CraftingStep {
        BasicSynthesis {
            potency: 360,
            cp_cost: 18,
            durability_cost: 20,
        }
    }

    pub fn veneration() -> impl CraftingStep {
        Veneration {}
    }

    pub fn basic_touch() -> impl CraftingStep {
        BasicTouch {
            potency: 100,
            cp_cost: 18,
            durability_cost: 10,
        }
    }

    // TODO: standard and advanced touch (probably easiest to implement these as combo steps?)

    pub fn prudent_touch() -> impl CraftingStep {
        BasicTouch {
            potency: 100,
            cp_cost: 25,
            durability_cost: 5,
        }
    }

    pub fn preparatory_touch() -> impl CraftingStep {
        BasicTouch {
            potency: 200,
            cp_cost: 40,
            durability_cost: 20,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::presets::Presets as p;
    use crate::simulator::Simulator as s;

    // basically just setting up scenarios on teamcraft and checking that these numbers match theirs

    #[test]
    fn basic_synthesis_1() {
        let new_state = s::run_steps(p::l90_player(), p::rlvl640_gear(), &["Basic Synthesis"]);

        assert_eq!(297, new_state.progress);
        assert_eq!(60, new_state.durability);
    }

    #[test]
    fn careful_synthesis_1() {
        let new_state = s::run_steps(p::l90_player(), p::rlvl640_gear(), &["Careful Synthesis"]);

        assert_eq!(446, new_state.progress);
        assert_eq!(60, new_state.durability);
        assert_eq!(500 - 7, new_state.cp);
    }

    #[test]
    fn veneration_increases_next_synthesis_step_by_50_percent() {
        let new_state = s::run_steps(
            p::l90_player(),
            p::rlvl640_gear(),
            &["Veneration", "Basic Synthesis"],
        );

        assert_eq!(446, new_state.progress);
        assert_eq!(500 - 18, new_state.cp);
    }

    #[test]
    fn veneration_runs_out_after_four_steps() {
        let new_state = s::run_steps(
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
        );

        assert_eq!((446 * 4) + 297, new_state.progress);
        assert_eq!(500 - 18, new_state.cp);
    }

    #[test]
    fn basic_touch_1() {
        let new_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            p::rlvl640_gear(),
            &["Basic Touch"],
        );

        assert_eq!(0, new_state.progress);
        assert_eq!(60, new_state.durability);
        assert_eq!(247, new_state.quality);
        assert_eq!(622 - 18, new_state.cp);
    }

    #[test]
    fn basic_touch_2() {
        let new_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            p::rlvl640_gear(),
            &["Basic Touch", "Basic Touch"],
        );

        // each Basic Touch adds an inner quiet stack, so the next touch action will be stronger
        assert_eq!(247 + 271, new_state.quality);
    }

    #[test]
    fn basic_touch_caps_at_10_inner_quiet_stacks() {
        let new_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            p::rlvl640_gear(),
            &["Basic Touch"; 12],
        );

        // each Basic Touch adds an inner quiet stack, so the next touch action will be stronger
        assert_eq!(
            247 + 271 + 296 + 321 + 345 + 370 + 395 + 419 + 444 + 469 + (2 * 494),
            new_state.quality
        );
    }

    #[test]
    fn innovation_buffs_basic_touch() {
        todo!();
    }
}
