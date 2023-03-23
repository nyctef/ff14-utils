use color_eyre::eyre::{eyre, Context, Result};
use ff14_utils::{
    csv,
    lookup::{ItemLookup, RecipeLookup},
    model::*,
    universalis::{get_market_data_lookup, price_up_to},
};
use itertools::Itertools;
use std::{env, path::PathBuf};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let csv_base = PathBuf::from("../ffxiv-datamining/csv");
    let items = ItemLookup::new(csv::read_items(&csv_base).await?);
    let recipes = RecipeLookup::new(csv::read_recipes(&csv_base).await?);

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

    let recipe_count = div_ceil(result_count, result_recipe.result.amount);
    let recipe = result_recipe * recipe_count;

    let all_ids = recipe.relevant_item_ids().collect_vec();
    let market_data = get_market_data_lookup(&*all_ids).await?;

    let resulting_item = items.item_by_id(recipe.result.item_id);
    let results: Result<Vec<_>> = recipe
        .ingredients
        .iter()
        .map(|ri| {
            let i = items.item_by_id(ri.item_id);
            let md = market_data.get(&i.id).unwrap();
            let price = price_up_to(&md.listings, ri.amount.into(), false).map_err(|e| eyre!(e))?;

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

    Ok(())
}

fn div_ceil(a: u32, b: u32) -> u32 {
    // https://stackoverflow.com/a/72442854
    (a + b - 1) / b
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
