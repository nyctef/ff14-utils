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
            ..*state
        }
    }

    fn cp_cost(&self) -> u8 {
        self.cp_cost
    }

    fn durability_cost(&self) -> u8 {
        self.durability_cost
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
            // note that we set the stack count here to 5 instead of 4,
            // since the generic logic will decrement it by 1 every time a step happens.
            // Is there a nicer way to make this read how it should?
            veneration_stacks: 5,
            ..*state
        }
    }

    fn cp_cost(&self) -> u8 {
        18
    }

    fn durability_cost(&self) -> u8 {
        0
    }
}

pub struct BasicTouch {
    potency: u16,
    cp_cost: u8,
    durability_cost: u8,
}

fn calc_quality_increase(
    stats: &PlayerStats,
    recipe: &Recipe,
    state: &CraftingState,
    potency: u16,
) -> u16 {
    let base_quality = (stats.control * 10) / recipe.rlvl.quality_divider as u16 + 35;
    let base_quality_modified_by_level =
        (base_quality as f32 * recipe.rlvl.quality_modifier as f32 * 0.01f32) as u16;

    let mut buff_modifier: f32 = 1.0;
    buff_modifier += 0.1 * state.inner_quiet_stacks as f32;

    let mut buff_modifier_multiplier = 1.0;
    if state.innovation_stacks > 0 {
        buff_modifier_multiplier += 0.5;
    }
    if state.great_strides {
        buff_modifier_multiplier += 1.0;
    }
    buff_modifier *= buff_modifier_multiplier;

    let total_quality_increase =
        buff_modifier * (base_quality_modified_by_level * potency) as f32 / 100.;
    total_quality_increase as u16
}

impl CraftingStep for BasicTouch {
    fn apply(&self, state: &CraftingState, stats: &PlayerStats, recipe: &Recipe) -> CraftingState {
        CraftingState {
            quality: state.quality + calc_quality_increase(stats, recipe, state, self.potency),
            inner_quiet_stacks: u8::min(10, state.inner_quiet_stacks + 1),
            great_strides: false,
            ..*state
        }
    }

    fn cp_cost(&self) -> u8 {
        self.cp_cost
    }

    fn durability_cost(&self) -> u8 {
        self.durability_cost
    }
}

pub struct Innovation {}

impl CraftingStep for Innovation {
    fn apply(
        &self,
        state: &CraftingState,
        _stats: &PlayerStats,
        _recipe: &Recipe,
    ) -> CraftingState {
        CraftingState {
            // see above comment about veneration stacks being 5 instead of 4
            innovation_stacks: 5,
            ..*state
        }
    }

    fn cp_cost(&self) -> u8 {
        18
    }

    fn durability_cost(&self) -> u8 {
        0
    }
}

pub struct GreatStrides {}
impl CraftingStep for GreatStrides {
    fn apply(
        &self,
        state: &CraftingState,
        _stats: &PlayerStats,
        _recipe: &Recipe,
    ) -> CraftingState {
        CraftingState {
            great_strides: true,
            ..*state
        }
    }

    fn cp_cost(&self) -> u8 {
        32
    }

    fn durability_cost(&self) -> u8 {
        0
    }
}

pub struct ByregotsBlessing {}
impl CraftingStep for ByregotsBlessing {
    fn apply(&self, state: &CraftingState, stats: &PlayerStats, recipe: &Recipe) -> CraftingState {
        if state.inner_quiet_stacks == 0 {
            // TODO: emit a warning or an error somehow?
            return state.clone();
        }
        let potency = 100 + (state.inner_quiet_stacks as u16 * 20);
        CraftingState {
            inner_quiet_stacks: 0,
            quality: state.quality + calc_quality_increase(stats, recipe, state, potency),
            ..*state
        }
    }

    fn cp_cost(&self) -> u8 {
        24
    }

    fn durability_cost(&self) -> u8 {
        10
    }
}

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
            ..*state
        }
    }

    fn cp_cost(&self) -> u8 {
        7
    }

    fn durability_cost(&self) -> u8 {
        0
    }
}

pub struct FocusedSynthesis {
    underlying: Box<dyn CraftingStep>,
}
impl CraftingStep for FocusedSynthesis {
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

    fn cp_cost(&self) -> u8 {
        self.underlying.cp_cost()
    }

    fn durability_cost(&self) -> u8 {
        self.underlying.durability_cost()
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

    pub fn innovation() -> impl CraftingStep {
        Innovation {}
    }

    pub fn great_strides() -> impl CraftingStep {
        GreatStrides {}
    }

    pub fn byregots_blessing() -> impl CraftingStep {
        ByregotsBlessing {}
    }

    pub fn observe() -> impl CraftingStep {
        Observe {}
    }

    pub fn focused_synthesis() -> impl CraftingStep {
        FocusedSynthesis {
            underlying: Box::new(BasicSynthesis {
                potency: 200,
                cp_cost: 5,
                durability_cost: 10,
            }),
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
        let new_state = s::run_steps(
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
        );

        assert_eq!(
            // first four touches get buffed by innovation and inner quiet
            370 + 407 + 444 + 481
            // fifth touch only gets the inner quiet stacks            
             + 345,
            new_state.quality
        );
    }

    #[test]
    fn great_strides_buffs_basic_touch() {
        let new_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            p::rlvl640_gear(),
            &["Great Strides", "Basic Touch"],
        );

        assert_eq!(247 * 2, new_state.quality);
    }

    #[test]
    fn byregots_blessing() {
        let new_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            p::rlvl640_gear(),
            &["Basic Touch", "Great Strides", "Byregot's Blessing"],
        );

        assert_eq!(0, new_state.inner_quiet_stacks);
        assert_eq!(247 + 652, new_state.quality);
        assert_eq!(50, new_state.durability);
    }

    #[test]
    fn focused_synthesis_fails_if_not_preceded_by_observe() {
        // technically it has a 50% success rate, but we don't want to rely on that in a simulator
        let new_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            p::rlvl640_gear(),
            &["Focused Synthesis"],
        );

        assert_eq!(0, new_state.progress);
        assert_eq!(60, new_state.durability);
    }

    #[test]
    fn focused_synthesis_succeeds_if_preceded_by_observe() {
        let new_state = s::run_steps(
            p::l90_player_with_jhinga_biryani_hq(),
            p::rlvl640_gear(),
            &["Observe", "Focused Synthesis"],
        );

        assert_eq!(496, new_state.progress);
        assert_eq!(60, new_state.durability);
        assert_eq!(622 - 7 - 5, new_state.cp);
        assert_eq!(2, new_state.steps);
    }
}
