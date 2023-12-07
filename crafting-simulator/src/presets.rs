use crate::model::*;
use ff14_data::{
    model::{RecipeLevel, RecipeLevelId},
    rlvl::RlvlLookup,
};
use lazy_static::lazy_static;

lazy_static! {
    static ref RECIPE_LEVELS: RlvlLookup = RlvlLookup::get_rlvl_lookup().unwrap();
}

pub struct Presets;

impl Presets {
    pub fn rlvl640_gear() -> Recipe {
        // ie diadochos gear
        Recipe {
            rlvl: RECIPE_LEVELS.rlvl(640).clone(),
            difficulty: 6600,
            durability: 70,
            quality_target: 14040,
        }
    }

    pub fn rlvl640_intermediate() -> Recipe {
        // ie diadochos gear
        Recipe {
            rlvl: RECIPE_LEVELS.rlvl(640).clone(),
            difficulty: 4488,
            durability: 35,
            quality_target: 9090,
        }
    }

    /// the max white scrip collectible
    pub fn rlvl555_collectible() -> Recipe {
        Recipe {
            rlvl: RECIPE_LEVELS.rlvl(555).clone(),
            difficulty: 3400,
            durability: 80,
            quality_target: 7100,
        }
    }

    pub fn l90_player() -> PlayerStats {
        PlayerStats::level_90(4014, 3574, 500)
    }

    pub fn l90_player_2() -> PlayerStats {
        PlayerStats::level_90(3862, 3529, 576)
    }

    pub fn l90_player_with_jhinga_biryani_hq() -> PlayerStats {
        PlayerStats::level_90(4014, 3574 + 90, 536 + 86)
    }

    pub fn l90_player_with_jhinga_biryani_hq_and_draught() -> PlayerStats {
        PlayerStats::level_90(4014, 3574 + 90, 536 + 86 + 21)
    }

    pub fn baseline_player() -> PlayerStats {
        // craftsmanship and control here are chosen to cancel out the +2 and +35 terms
        // in the progress/quality calculations
        PlayerStats::level_90(980, 650, 1000)
    }

    pub fn baseline_recipe(difficulty: u16, durability: u16, quality_target: u16) -> Recipe {
        Recipe {
            rlvl: RecipeLevel {
                // we assume the rlvl is always higher than the player's crafter level
                rlvl: RecipeLevelId::new(999),
                // these values just cancel out in the progress/quality calculations
                progress_divider: 100,
                progress_modifier: 100,
                quality_divider: 100,
                quality_modifier: 100,
                stars: 4,
            },
            difficulty,
            durability,
            quality_target,
        }
    }
}
