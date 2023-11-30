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
    pub durability: u16,
    pub progress: u16,
    pub quality: u16,
    pub cp: u16,
    pub steps: u8,
    pub inner_quiet_stacks: u8,
}

impl CraftingState {
    pub fn new(durability: u16, cp: u16) -> CraftingState {
        CraftingState {
            durability,
            progress: 0,
            quality: 0,
            cp,
            steps: 0,
            inner_quiet_stacks: 0,
        }
    }

    pub fn initial(stats: &PlayerStats, recipe: &Recipe) -> CraftingState {
        CraftingState::new(recipe.durability, stats.cp)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Recipe {
    pub rlvl: RecipeLevel,
    pub difficulty: u16,
    pub durability: u16,
    pub quality_target: u16,
    // TODO: is it worth tracking if the recipe accepts HQ mats?
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RecipeLevel {
    pub rlvl: u16,
    pub progress_divider: u8,
    pub progress_modifier: u8,
    pub quality_divider: u8,
    pub quality_modifier: u8,
}

pub static RLVL640: RecipeLevel = RecipeLevel {
    rlvl: 640,
    progress_divider: 130,
    progress_modifier: 80,
    quality_divider: 180,
    quality_modifier: 100,
};

pub trait CraftingStep {
    fn apply(&self, state: &CraftingState, stats: &PlayerStats, recipe: &Recipe) -> CraftingState;

    /**
     * most of the time, a crafting step will increment the step count by one as expected. Some exceptions:
     * - if we do combo steps, then those count as more than one
     * - Final Appraisal doesn't consume a step, so it counts as zero
     */
    fn num_steps(&self) -> u8 {
        1
    }
}
