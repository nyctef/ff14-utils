use derive_more::Constructor;
use ff14_data::model::RecipeLevel;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PlayerStats {
    /* Not the player's visible level, but the internal level that gets checked against the recipe's rlvl */
    pub player_lvl: u16,
    pub craftsmanship: u16,
    pub control: u16,
    pub cp: u16,
}

impl PlayerStats {
    pub fn level_90(craftsmanship: u16, control: u16, cp: u16) -> PlayerStats {
        PlayerStats {
            player_lvl: 560,
            craftsmanship,
            control,
            cp,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CraftingState {
    pub durability: i16,
    pub progress: u16,
    pub quality: u16,
    pub cp: i16,
    pub steps: u8,
    pub inner_quiet_stacks: u8,
    pub veneration_stacks: u8,
    pub innovation_stacks: u8,
    pub great_strides_stacks: u8,
    pub muscle_memory_stacks: u8,
    pub manipulation_stacks: u8,
    pub manipulation_delay: u8,
    pub waste_not_stacks: u8,
    pub final_appraisal_stacks: u8,
    // of course observe doesn't actually have stacks, but this
    // is the easiest way we have to track "last step was observe"
    // at the moment
    pub observe_stacks: u8,
    pub touch_combo_stage: u8,
}

impl CraftingState {
    pub fn new(durability: u16, cp: u16) -> CraftingState {
        CraftingState {
            durability: durability as i16,
            progress: 0,
            quality: 0,
            cp: cp as i16,
            steps: 0,
            inner_quiet_stacks: 0,
            veneration_stacks: 0,
            innovation_stacks: 0,
            great_strides_stacks: 0,
            muscle_memory_stacks: 0,
            manipulation_stacks: 0,
            manipulation_delay: 0,
            waste_not_stacks: 0,
            final_appraisal_stacks: 0,
            observe_stacks: 0,
            touch_combo_stage: 0,
        }
    }

    pub fn initial(stats: &PlayerStats, recipe: &Recipe) -> CraftingState {
        CraftingState::new(recipe.durability, stats.cp)
    }
}

// TODO: merge with the recipe struct from the data crate
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Recipe {
    pub rlvl: RecipeLevel,
    pub difficulty: u16,
    pub durability: u16,
    pub quality_target: u16,
    // TODO: can these be determined from the rlvl, or are they always recipe-specific?
    pub required_craftsmanship: u16,
    pub required_control: u16,
}

pub type StepResult = Result<CraftingState, CraftingIssueType>;

pub trait CraftingStep {
    fn apply(&self, state: &CraftingState, stats: &PlayerStats, recipe: &Recipe) -> StepResult;

    fn cp_cost(&self, state: &CraftingState) -> u8;

    fn durability_cost(&self) -> u8;

    /**
     * most of the time, a crafting step will increment the step count by one as expected. Some exceptions:
     * - if we do combo steps, then those count as more than one
     * - Final Appraisal doesn't consume a step, so it counts as zero
     */
    fn num_steps(&self) -> u8 {
        1
    }
}

/** InfallibleStep is very similar to CraftingStep, except it always returns a CraftingState instead of a result type. This just makes the common case require a bit less typing. */
pub trait InfallibleStep {
    fn apply(&self, state: &CraftingState, stats: &PlayerStats, recipe: &Recipe) -> CraftingState;

    fn cp_cost(&self, state: &CraftingState) -> u8;

    fn durability_cost(&self) -> u8;

    fn num_steps(&self) -> u8 {
        1
    }
}

impl<T: InfallibleStep> CraftingStep for T {
    fn apply(&self, state: &CraftingState, stats: &PlayerStats, recipe: &Recipe) -> StepResult {
        Ok(T::apply(self, state, stats, recipe))
    }

    fn cp_cost(&self, state: &CraftingState) -> u8 {
        T::cp_cost(self, state)
    }

    fn durability_cost(&self) -> u8 {
        T::durability_cost(self)
    }

    fn num_steps(&self) -> u8 {
        T::num_steps(self)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CraftStatus {
    Success,
    Failure,
    Incomplete,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CraftingIssueType {
    InsufficientStats,
    OutOfCP,
    DurabilityFailed,
    LackingInnerQuiet,
    PreventedByWasteNot,
    NotOnFirstStep,
    ChanceBasedAction,
}

impl CraftingIssueType {
    /// whether a particular issue ends the craft, or just causes one action to fail
    pub fn is_fatal(&self) -> bool {
        match self {
            CraftingIssueType::InsufficientStats => true,
            CraftingIssueType::DurabilityFailed => true,
            CraftingIssueType::OutOfCP => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Constructor)]
pub struct CraftingIssue {
    pub issue_type: CraftingIssueType,
    pub step_index: u8,
}

#[derive(Debug, Clone)]
pub struct CraftingReport {
    /// The list of steps actually run. Differs from the input list of steps
    /// since some steps might fail to trigger or cause the craft to end
    pub step_log: Vec<&'static str>,
    pub final_state: CraftingState,
    pub issues: Vec<CraftingIssue>,
    pub status: CraftStatus,
}
