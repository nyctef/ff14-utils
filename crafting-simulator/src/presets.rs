use crate::model::*;
use ff14_data::{
    food::FoodLookup,
    model::{Food, RecipeLevel, RecipeLevelId},
    rlvl::RlvlLookup,
};
use lazy_static::lazy_static;

lazy_static! {
    static ref RECIPE_LEVELS: RlvlLookup = RlvlLookup::get_rlvl_lookup().unwrap();
    static ref FOODS: FoodLookup = FoodLookup::get_food_lookup().unwrap();
}

pub struct Presets;

impl Presets {
    /// pajamas!
    pub fn l90_chocobo_glam() -> Recipe {
        Recipe {
            rlvl: RECIPE_LEVELS.rlvl(610).clone(),
            difficulty: 4400,
            durability: 70,
            quality_target: 8200,
            required_craftsmanship: 0,
            required_control: 0,
        }
    }

    /// ie diadochos gear
    pub fn l90_4star_gear() -> Recipe {
        Recipe {
            rlvl: RECIPE_LEVELS.rlvl(640).clone(),
            difficulty: 6600,
            durability: 70,
            quality_target: 14040,
            required_craftsmanship: 3950,
            required_control: 3660,
        }
    }

    /// eg garnet cotton
    pub fn l90_4star_intermediate() -> Recipe {
        Recipe {
            rlvl: RECIPE_LEVELS.rlvl(640).clone(),
            difficulty: 4488,
            durability: 35,
            quality_target: 9090,
            required_craftsmanship: 3950,
            required_control: 3660,
        }
    }

    /// eg Indagator's gear
    pub fn l90_3star_gear() -> Recipe {
        Recipe {
            rlvl: RECIPE_LEVELS.rlvl(620).clone(),
            difficulty: 5720,
            durability: 70,
            quality_target: 12900,
            required_craftsmanship: 3700,
            required_control: 3280,
        }
    }

    /// eg ilmenite ingot
    pub fn l90_3star_intermediate() -> Recipe {
        Recipe {
            rlvl: RECIPE_LEVELS.rlvl(610).clone(),
            difficulty: 3696,
            durability: 35,
            quality_target: 8200,
            required_craftsmanship: 3700,
            required_control: 3280,
        }
    }

    /// for customized components
    pub fn l90_relic_tier3() -> Recipe {
        Recipe {
            rlvl: RECIPE_LEVELS.rlvl(620).clone(),
            difficulty: 4620,
            durability: 60,
            quality_target: 12040,
            required_craftsmanship: 0,
            required_control: 0,
        }
    }

    /// for brilliant components
    pub fn l90_relic_tier4() -> Recipe {
        Recipe {
            rlvl: RECIPE_LEVELS.rlvl(625).clone(),
            difficulty: 5280,
            durability: 60,
            quality_target: 13050,
            required_craftsmanship: 0,
            required_control: 0,
        }
    }

    /// the max white scrip collectible
    pub fn l89_collectible() -> Recipe {
        Recipe {
            rlvl: RECIPE_LEVELS.rlvl(555).clone(),
            difficulty: 3400,
            durability: 80,
            quality_target: 7100,
            required_craftsmanship: 0,
            required_control: 0,
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
        Self::baseline_recipe_with_required_stats(difficulty, durability, quality_target, 0, 0)
    }

    pub fn baseline_recipe_with_required_stats(
        difficulty: u16,
        durability: u16,
        quality_target: u16,
        required_craftsmanship: u16,
        required_control: u16,
    ) -> Recipe {
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
            required_craftsmanship,
            required_control,
        }
    }

    pub fn tsai_tou_vounou() -> &'static Food {
        FOODS.by_name("Tsai tou Vounou").unwrap()
    }

    pub fn jhinga_biryani() -> &'static Food {
        FOODS.by_name("Jhinga Biryani").unwrap()
    }

    pub fn cunning_draught() -> &'static Food {
        FOODS.by_name("Cunning Craftsman's Draught").unwrap()
    }

    /// quick workaround for tests using realistic data
    /// that were passing before stat requirements were implemented
    #[cfg(test)]
    pub fn without_required_stats(recipe: Recipe) -> Recipe {
        Recipe {
            required_craftsmanship: 0,
            required_control: 0,
            ..recipe
        }
    }
}
