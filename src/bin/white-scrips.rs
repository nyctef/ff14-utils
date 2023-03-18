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

    let l89_collectables = items
        .iter()
        .filter(|i| i.ilvl == 548 && i.name.starts_with("Rarefied"))
        .collect_vec();

    let recipes = l89_collectables
        .iter()
        .map(|i| recipes.iter().find(|r| r.result.item_id == i.id).unwrap())
        .map(|r| r * 10)
        .collect_vec();

    let all_ids = recipes
        .iter()
        .flat_map(|r| {
            iter::once(r.result.item_id)
                .chain(r.ingredients.iter().map(|i| i.item_id))
                .collect_vec()
        })
        .collect_vec();
    let get_market_data = get_market_data(&*all_ids).await?;
    let market_data = get_market_data
        .iter()
        .map(|x| (x.item_id, x))
        .collect::<HashMap<_, _>>();

    for recipe in &recipes {
        let resulting_item = items
            .iter()
            .find(|i| i.id == recipe.result.item_id)
            .unwrap();
        let results: Result<Vec<_>> = recipe
            .ingredients
            .iter()
            .map(|ri| {
                let i = items.iter().find(|i| i.id == ri.item_id).unwrap();
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
