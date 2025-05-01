use color_eyre::eyre::{eyre, Context, Result};
use ff14_data::{
    lookup::{ItemLookup, RecipeLookup},
    model::*,
};
use ff14_utils::{
    recipe_calculation::{
        match_recipe_to_output_count, print_recipe_calculation, process_recipe_item,
    },
    universalis::get_market_data_lookup,
};
use itertools::Itertools;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let items = ItemLookup::from_datamining_csv().await?;
    let recipes = RecipeLookup::from_datamining_csv().await?;

    let recipe = choose_recipe_from_args(&items, &recipes)?;

    let all_ids = recipe.relevant_item_ids(&recipes).collect_vec();
    let market_data = get_market_data_lookup(&all_ids).await?;

    let (_, results) = process_recipe_item(0, &recipe.result, &items, &market_data, &recipes, true);
    print_recipe_calculation(results);

    Ok(())
}

fn choose_recipe_from_args(items: &ItemLookup, recipes: &RecipeLookup) -> Result<Recipe> {
    let args = env::args().collect_vec();
    let result_recipe;
    let result_count;

    let look_up_recipe = |name| {
        items
            .item_by_name_opt(name)
            .and_then(|i| recipes.recipe_for_item(i.id))
            .ok_or_else(|| eyre!("Could not find recipe matching item '{}'", name))
    };

    match &args[1..] {
        [name] => {
            result_recipe = look_up_recipe(name)?;
            result_count = 1;
        }
        [name, count] => {
            result_recipe = look_up_recipe(name)?;
            result_count = count.parse::<u32>().wrap_err("Failed to parse count")?;
        }
        _ => return Err(eyre!("Usage: specific-recipe [name] [amount]")),
    }

    let recipe = match_recipe_to_output_count(result_count, result_recipe);
    Ok(recipe)
}
