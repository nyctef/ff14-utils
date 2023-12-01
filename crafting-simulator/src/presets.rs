use crate::model::*;

pub struct Presets;

impl Presets {
    pub fn rlvl_640() -> RecipeLevel {
        RecipeLevel {
            rlvl: 640,
            progress_divider: 130,
            progress_modifier: 80,
            quality_divider: 180,
            quality_modifier: 100,
        }
    }

    pub fn rlvl640_gear() -> Recipe {
        Recipe {
            rlvl: Self::rlvl_640(),
            difficulty: 6600,
            durability: 70,
            quality_target: 14040,
        }
    }

    pub fn l90_player() -> PlayerStats {
        PlayerStats::level_90(4014, 3574, 500)
    }
}
