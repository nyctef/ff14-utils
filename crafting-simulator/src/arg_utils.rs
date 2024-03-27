use crate::{model::Recipe, presets::Presets as preset};
use color_eyre::eyre::{eyre, Result};
use ff14_data::model::Food;

// TODO: might be nice to dedupe more arg handling knowledge here
// (eg help text with list of values, name of arg, etc)
// but it's not super necessary.

pub fn recipe_from_arg_value(value: &str) -> Result<Recipe> {
    match value {
        "l90_4s_mat" => Ok(preset::l90_4star_intermediate()),
        "l90_4s_gear" => Ok(preset::l90_4star_gear()),
        "l90_3s_mat" => Ok(preset::l90_3star_intermediate()),
        "l90_3s_gear" => Ok(preset::l90_3star_gear()),
        "l90_relic_tier3" => Ok(preset::l90_relic_tier3()),
        "l90_relic_tier4" => Ok(preset::l90_relic_tier4()),
        "l90_chocobo_glam" => Ok(preset::l90_chocobo_glam()),
        other => Err(eyre!("Unrecognised recipe type {}", other)),
    }
}

pub fn food_from_arg_value(value: Option<&str>) -> Result<Option<&'static Food>> {
    let food = value.map(|f| match f {
        "tsai_tou" => Ok(preset::tsai_tou_vounou()),
        "jhinga_biryani" => Ok(preset::jhinga_biryani()),
        other => Err(eyre!("Unrecognised food type {}", other)),
    });

    food.transpose()
}

pub fn potion_from_arg_value(value: Option<&str>) -> Result<Option<&'static Food>> {
    let potion = value.map(|f| match f {
        "cunning_draught" => Ok(preset::cunning_draught()),
        other => Err(eyre!("Unrecognised potion type {}", other)),
    });

    potion.transpose()
}
