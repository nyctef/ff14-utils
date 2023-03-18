use color_eyre::eyre::{eyre, Result};
use ff14_utils::{
    csv,
    model::*,
    universalis::{get_market_data, price_up_to},
};
use itertools::Itertools;
use std::{collections::HashMap, iter, path::PathBuf};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let csv_base = PathBuf::from("../ffxiv-datamining/csv");
    let items = csv::read_items(&csv_base).await?;
    let recipes = csv::read_recipes(&csv_base).await?;

    let pancakes = items
        .iter()
        .find(|i| i.name == "Rarefied Giant Popoto Pancakes")
        .unwrap();
    let recipe = recipes
        .iter()
        .find(|r| r.result.item_id == pancakes.id)
        .unwrap();

    let all_ids = iter::once(recipe.result.item_id)
        .chain(recipe.ingredients.iter().map(|i| i.item_id))
        .collect_vec();
    let get_market_data = get_market_data(&*all_ids).await?;
    let market_data = get_market_data
        .iter()
        .map(|x| (x.item_id, x))
        .collect::<HashMap<_, _>>();

    let count = 10;
    let recipe = recipe * count;

    let results: Result<Vec<_>> = recipe
        .ingredients
        .iter()
        .map(|ri| {
            let i = items.iter().find(|i| i.id == ri.item_id).unwrap();
            let md = market_data.get(&i.id).unwrap();
            let price = price_up_to(&md.listings, ri.amount.into(), false).map_err(|e| eyre!(e))?;

            Ok((ri, i, price))
        })
        .collect();
    let results = results?;

    let total_price: u32 = results.iter().map(|r| r.2).sum();

    println!(
        "{}:\t{}",
        format_recipe_item(&recipe.result, pancakes),
        total_price
    );
    for (ri, i, price) in results {
        println!("\t{:>8} {}", price, format_recipe_item(ri, i));
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
