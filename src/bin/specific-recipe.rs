use color_eyre::eyre::{eyre, Context, Result};
use colored::Colorize;
use ff14_utils::{
    csv,
    lookup::{ItemLookup, RecipeLookup},
    model::*,
    time_utils::hm_ago_from_now,
    universalis::{get_market_data_lookup, price_up_to, ItemMarketData},
};
use itertools::Itertools;
use std::{cmp::min, collections::HashMap, env, fmt::Display, path::PathBuf};
use thousands::Separable;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let csv_base = PathBuf::from("../ffxiv-datamining/csv");
    let items = ItemLookup::new(csv::read_items(&csv_base).await?);
    let recipes = RecipeLookup::new(csv::read_recipes(&csv_base).await?);

    let recipe = choose_recipe_from_args(&items, &recipes)?;

    let all_ids = recipe.relevant_item_ids(&recipes).collect_vec();
    let market_data = get_market_data_lookup(&*all_ids).await?;

    process_recipe_item(0, &recipe.result, &items, &market_data, &recipes);

    Ok(())
}

fn process_recipe_item(
    indent: usize,
    ri: &RecipeItem,
    items: &ItemLookup,
    market_data: &HashMap<ItemId, ItemMarketData>,
    recipes: &RecipeLookup,
) -> u32 {
    // TODO: try to reverse order to be natural?
    let md = market_data.get(&ri.item_id).unwrap();
    let i = items.item_by_id(ri.item_id);
    let market_price = price_up_to(&md.listings, ri.amount, i.can_be_hq);
    let crafting_price = recipes.recipe_for_item(i.id).map(|sub_recipe| {
        sub_recipe
            .ingredients
            .iter()
            .map(|sub_ri| process_recipe_item(indent + 2, sub_ri, items, market_data, recipes))
            .sum()
    });

    let lower_price = min(
        market_price.clone().unwrap_or(u32::MAX),
        crafting_price.clone().unwrap_or(u32::MAX),
    );

    let market_price_str = market_price
        .as_ref()
        .map(|p| {
            format!(
                "M:{} {}",
                p.separate_with_commas(),
                hm_ago_from_now(md.last_upload_time).dimmed()
            )
        })
        .unwrap_or_default();
    let crafting_price_str = crafting_price
        .map(|p| format!("C:{}", p.separate_with_commas(),))
        .unwrap_or_default();
    let diff_str = if let (Ok(mp), Some(cp)) = (market_price, crafting_price) {
        format_num_diff(mp, cp).to_string()
    } else {
        "".to_string()
    };

    let price_display = vec![market_price_str, crafting_price_str, diff_str].join(" ");

    println!(
        "{}{}{} {}",
        " ".repeat(indent),
        format_recipe_item(ri, i),
        if i.can_be_hq { " (HQ)" } else { "" },
        price_display
    );
    lower_price
}

fn format_num_diff(baseline: u32, value: u32) -> impl Display {
    if value < baseline {
        format!("+{}", baseline - value)
            .separate_with_commas()
            .green()
    } else {
        format!("-{}", value - baseline)
            .separate_with_commas()
            .red()
    }
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

    let recipe_count = div_ceil(result_count, result_recipe.result.amount);
    let recipe = result_recipe * recipe_count;
    Ok(recipe)
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
