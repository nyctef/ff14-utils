use crate::model::*;
use ff14_data::{food::FoodLookup, model::Food};
use lazy_static::lazy_static;

lazy_static! {
    static ref FOODS: FoodLookup = FoodLookup::get_food_lookup().unwrap();
}

pub struct Presets;

impl Presets {
    /// ie diadochos gear
    pub fn l90_4star_gear() -> SimulatorRecipe {
        SimulatorRecipe {
            rlvl: 640,
            progress_divider: 130,
            quality_divider: 115,
            progress_modifier: 80,
            quality_modifier: 70,
            difficulty: 6600,
            durability: 70,
            quality_target: 14040,
            required_craftsmanship: 3950,
            required_control: 3660,
        }
    }

    /// eg garnet cotton
    pub fn l90_4star_intermediate() -> SimulatorRecipe {
        SimulatorRecipe {
            rlvl: 640,
            progress_divider: 130,
            quality_divider: 115,
            progress_modifier: 80,
            quality_modifier: 70,
            difficulty: 4488,
            durability: 35,
            quality_target: 9090,
            required_craftsmanship: 3950,
            required_control: 3660,
        }
    }

    pub fn l90_player() -> PlayerStats {
        PlayerStats::level_90(4014, 3574, 500)
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

    pub fn baseline_recipe(
        difficulty: u16,
        durability: u16,
        quality_target: u16,
    ) -> SimulatorRecipe {
        Self::baseline_recipe_with_required_stats(difficulty, durability, quality_target, 0, 0)
    }

    pub fn baseline_recipe_with_required_stats(
        difficulty: u16,
        durability: u16,
        quality_target: u16,
        required_craftsmanship: u16,
        required_control: u16,
    ) -> SimulatorRecipe {
        SimulatorRecipe {
            // we assume the rlvl is always higher than the player's crafter level
            rlvl: 999,
            // these values just cancel out in the progress/quality calculations
            progress_divider: 100,
            progress_modifier: 100,
            quality_divider: 100,
            quality_modifier: 100,
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
    pub fn without_required_stats(recipe: SimulatorRecipe) -> SimulatorRecipe {
        SimulatorRecipe {
            required_craftsmanship: 0,
            required_control: 0,
            ..recipe
        }
    }
}
