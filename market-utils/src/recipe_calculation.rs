use crate::{
    lookup::{ItemLookup, RecipeLookup},
    model::*,
    time_utils::hm_ago_from_now,
    universalis::{price_up_to, ItemMarketData},
};
use chrono::{DateTime, Utc};
use colored::Colorize;
use std::{cmp::min, collections::HashMap, fmt::Display};
use thousands::Separable;

pub struct LineItem {
    indent: usize,
    pub name_and_amount: String,
    pub amount: u32,
    pub market_price: Option<u32>,
    pub market_price_age: Option<DateTime<Utc>>,
    pub crafting_price: Option<u32>,
    pub crafting_profit: Option<i64>,
}

pub fn process_recipe_item(
    indent: usize,
    ri: &RecipeItem,
    items: &ItemLookup,
    market_data: &HashMap<ItemId, ItemMarketData>,
    recipes: &RecipeLookup,
) -> (u32, Vec<LineItem>) {
    let md = market_data.get(&ri.item_id);
    let i = items.item_by_id(ri.item_id);
    let market_price = md.and_then(|md| price_up_to(&md.listings, ri.amount, i.can_be_hq).ok());
    let crafting_results = recipes.recipe_for_item(ri.item_id).map(|sub_recipe| {
        (match_recipe_to_output_count(ri.amount, sub_recipe))
            .ingredients
            .iter()
            .map(|sub_ri| process_recipe_item(indent + 2, sub_ri, items, market_data, recipes))
            .fold(
                (0, vec![]),
                |(prev_price, mut prev_lines), (sub_price, lines)| {
                    prev_lines.extend(lines);
                    (prev_price + sub_price, prev_lines)
                },
            )
    });
    let (crafting_price, crafting_lines) = crafting_results.unzip();

    let mut crafting_lines = crafting_lines.unwrap_or(vec![]);
    let crafting_profit = market_price
        .zip(crafting_price)
        .map(|(mp, cp)| mp as i64 - cp as i64);
    crafting_lines.push(LineItem {
        indent,
        amount: ri.amount,
        name_and_amount: format_recipe_item(ri, i),
        market_price,
        market_price_age: md.map(|md| md.last_upload_time),
        crafting_price,
        crafting_profit,
    });

    let lower_price = min(
        market_price.unwrap_or(u32::MAX),
        crafting_price.unwrap_or(u32::MAX),
    );

    (lower_price, crafting_lines)
}

pub fn print_recipe_calculation(mut lines: Vec<LineItem>) {
    // Reverse the crafting lines so that the final result is at the top
    // and sub-recipes are nested below
    //
    // we'd like to do this in process_recipe_item to avoid the coupling,
    // but we can't because we have to only reverse once at the end of the
    // process rather than once at each recursion step.
    lines.reverse();

    for line in lines {
        print_line_item(&line);
    }
}

pub fn print_line_item(line: &LineItem) {
    let market_price_str = line
        .market_price
        .map(|p| {
            format!(
                "M:{} {}",
                p.separate_with_commas(),
                line.market_price_age
                    .map(|md| { hm_ago_from_now(md).dimmed() })
                    .unwrap_or_default()
            )
        })
        .unwrap_or_default();
    let crafting_price_str = line
        .crafting_price
        .map(|p| format!("C:{}", p.separate_with_commas(),))
        .unwrap_or_default();
    let diff_str = line
        .crafting_profit
        .map(|p| format_num_diff(p).to_string())
        .unwrap_or(String::new());

    let price_display = vec![market_price_str, crafting_price_str, diff_str].join(" ");

    println!(
        "{}{} {}",
        " ".repeat(line.indent),
        line.name_and_amount,
        price_display
    );
}

pub fn match_recipe_to_output_count(output_count: u32, original_recipe: &Recipe) -> Recipe {
    let recipe_count = div_ceil(output_count, original_recipe.result.amount);
    original_recipe * recipe_count
}

fn div_ceil(a: u32, b: u32) -> u32 {
    // https://stackoverflow.com/a/72442854
    (a + b - 1) / b
}

fn format_num_diff(value: i64) -> impl Display {
    if value > 0 {
        format!("+{}", value).separate_with_commas().green()
    } else {
        format!("-{}", value).separate_with_commas().red()
    }
}

fn format_recipe_item(ri: &RecipeItem, i: &Item) -> String {
    format!(
        "{} {}{}",
        ri.amount,
        if ri.amount > 1 {
            &i.name_plural
        } else {
            &i.name_singular
        },
        if i.can_be_hq { " (HQ)" } else { "" }
    )
}
