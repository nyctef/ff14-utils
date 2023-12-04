use color_eyre::eyre::Result;
use ff14_data::lookup::{ItemLookup, RecipeLookup};
use ff14_utils::{
    csv,
    recipe_calculation::{print_line_item, process_recipe_item},
    universalis::get_market_data_lookup,
};
use itertools::Itertools;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let csv_base = PathBuf::from("../../ffxiv-datamining/csv");
    let items = ItemLookup::new(csv::read_items(&csv_base).await?);
    let recipes_lookup = RecipeLookup::new(csv::read_recipes(&csv_base).await?);

    let l90_collectables = items
        .matching(|i| i.ilvl == 560 && i.name.starts_with("Rarefied"))
        .collect_vec();

    let recipes = l90_collectables
        .iter()
        // only include items that have a recipe (ie skip gathering collectables)
        .filter_map(|i| recipes_lookup.recipe_for_item(i.id))
        .map(|r| r * 10)
        .collect_vec();

    let all_ids = recipes
        .iter()
        .flat_map(|r| r.relevant_item_ids(&recipes_lookup))
        .collect_vec();
    let market_data = get_market_data_lookup(&all_ids).await?;

    for recipe in &recipes {
        let (_, result) =
            process_recipe_item(0, &recipe.result, &items, &market_data, &recipes_lookup);
        print_line_item(result.iter().last().unwrap());
    }

    Ok(())
}
