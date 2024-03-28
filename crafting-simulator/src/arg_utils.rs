use crate::{model::SimulatorRecipe, presets::Presets as preset};
use color_eyre::eyre::{eyre, Result};
use ff14_data::{
    lookup::{ItemLookup, RecipeLookup},
    model::Food,
};

// TODO: might be nice to dedupe more arg handling knowledge here
// (eg help text with list of values, name of arg, etc)
// but it's not super necessary.

pub async fn recipe_from_arg_value(value: &str) -> Result<SimulatorRecipe> {
    let item_lookup = &ItemLookup::from_datamining_csv().await?;
    let item = item_lookup
        .item_by_name_opt(value)
        .ok_or_else(|| eyre!("No item found with name {}", value))?;

    let recipe_lookup = &RecipeLookup::from_datamining_csv().await?;
    let recipe = recipe_lookup
        .recipe_for_item(item.id)
        .ok_or_else(|| eyre!("No recipe found for item {}", item.name))?;

    Ok(SimulatorRecipe::from_recipe(&recipe))
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
