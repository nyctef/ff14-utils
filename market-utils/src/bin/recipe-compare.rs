use color_eyre::eyre::{eyre, Context, Result};
use ff14_utils::{
    csv,
    lookup::{ItemLookup, RecipeLookup},
    model::*,
    recipe_calculation::{match_recipe_to_output_count, print_line_item, process_recipe_item},
    universalis::get_market_data_lookup,
};
use itertools::Itertools;
use std::{env, path::PathBuf};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let csv_base = PathBuf::from("../../ffxiv-datamining/csv");
    let items = ItemLookup::new(csv::read_items(&csv_base).await?);
    let recipes_lookup = RecipeLookup::new(csv::read_recipes(&csv_base).await?);

    let recipes = choose_recipes_from_args(&items, &recipes_lookup)?;

    let all_ids = recipes
        .iter()
        .flat_map(|r| r.relevant_item_ids(&recipes_lookup))
        .collect_vec();
    let market_data = get_market_data_lookup(&all_ids).await?;

    for recipe in recipes {
        let (_, results) =
            process_recipe_item(0, &recipe.result, &items, &market_data, &recipes_lookup);
        print_line_item(results.last().unwrap());
    }

    Ok(())
}

fn choose_recipes_from_args<'a>(
    items: &ItemLookup,
    recipes: &'a RecipeLookup,
) -> Result<Vec<&'a Recipe>> {
    let args = env::args().collect_vec();
    let results;

    let look_up_recipe = |name| {
        items
            .name_containing(name)
            .filter_map(|i| recipes.recipe_for_item(i.id))
            .collect_vec()
    };

    match &args[1..] {
        [name] => {
            results = look_up_recipe(name);
        }
        _ => return Err(eyre!("Usage: specific-recipe [substr]")),
    }

    Ok(results)
}
