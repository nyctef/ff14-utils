use color_eyre::eyre::{eyre, Result};
use ff14_utils::{
    csv,
    lookup::{ItemLookup, RecipeLookup},
    model::*,
    universalis::{get_market_data_lookup, price_up_to},
};
use itertools::Itertools;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let csv_base = PathBuf::from("../ffxiv-datamining/csv");
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
        .flat_map(|r| r.relevant_item_ids(&recipes_lookup).collect_vec())
        .collect_vec();
    let market_data = get_market_data_lookup(&*all_ids).await?;

    for recipe in &recipes {
        let resulting_item = items.item_by_id(recipe.result.item_id);
        let results: Result<Vec<_>> = recipe
            .ingredients
            .iter()
            .map(|ri| {
                let i = items.item_by_id(ri.item_id);
                let md = market_data.get(&i.id).unwrap();
                let price =
                    price_up_to(&md.listings, ri.amount.into(), false).map_err(|e| eyre!(e))?;

                Ok((ri, i, price))
            })
            .collect();
        let results = results?;

        let total_price: u32 = results.iter().map(|r| r.2).sum();

        println!(
            "{}:\t{}",
            format_recipe_item(&recipe.result, resulting_item),
            total_price
        );
        for (ri, i, price) in results {
            println!("\t{:>8} {}", price, format_recipe_item(ri, i));
        }
    }

    Ok(())
}

fn format_recipe_item(ri: &RecipeItem, i: &Item) -> String {
    format!(
        "{} {}",
        ri.amount,
        if ri.amount > 1 {
            &i.name_plural
        } else {
            &i.name_singular
        }
    )
}
