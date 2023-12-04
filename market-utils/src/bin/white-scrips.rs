use color_eyre::eyre::Result;
use ff14_data::lookup::{ItemLookup, RecipeLookup};
use ff14_utils::{
    csv, recipe_calculation::process_recipe_item, universalis::get_market_data_lookup,
};
use itertools::Itertools;
use std::path::PathBuf;
use thousands::Separable;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let csv_base = PathBuf::from("../../ffxiv-datamining/csv");
    let items = ItemLookup::new(csv::read_items(&csv_base).await?);
    let recipes_lookup = RecipeLookup::new(csv::read_recipes(&csv_base).await?);

    let l89_collectables = items
        .matching(|i| i.ilvl == 548 && i.name.starts_with("Rarefied"))
        .collect_vec();

    let recipes = l89_collectables
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

    let result_lines = recipes
        .iter()
        .map(|r| process_recipe_item(0, &r.result, &items, &market_data, &recipes_lookup).1)
        .map(|r| r.into_iter().last().unwrap())
        .sorted_by_key(|line| line.crafting_price);

    for line in result_lines {
        println!(
            "{:<50}: {} or ~{} per scrip",
            line.name_and_amount,
            line.crafting_price
                .expect("crafting price")
                .separate_with_commas(),
            (line.crafting_price.expect("crafting price") / line.amount / 198)
                .separate_with_commas()
        );
        // TODO: maybe a --detailed option to print all results?
    }

    Ok(())
}
