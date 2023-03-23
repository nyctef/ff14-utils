use crate::{
    lookup::{ItemLookup, RecipeLookup},
    model::*,
    time_utils::hm_ago_from_now,
    universalis::{price_up_to, ItemMarketData},
};
use colored::Colorize;
use std::{cmp::min, collections::HashMap, fmt::Display};
use thousands::Separable;

pub fn process_recipe_item(
    indent: usize,
    ri: &RecipeItem,
    items: &ItemLookup,
    market_data: &HashMap<ItemId, ItemMarketData>,
    recipes: &RecipeLookup,
) -> u32 {
    // TODO: try to reverse order to be natural?
    let md = market_data.get(&ri.item_id);
    let i = items.item_by_id(ri.item_id);
    let market_price = md.and_then(|md| price_up_to(&md.listings, ri.amount, i.can_be_hq).ok());
    let crafting_price = recipes.recipe_for_item(i.id).map(|sub_recipe| {
        sub_recipe
            .ingredients
            .iter()
            .map(|sub_ri| process_recipe_item(indent + 2, sub_ri, items, market_data, recipes))
            .sum()
    });

    let lower_price = min(
        market_price.unwrap_or(u32::MAX),
        crafting_price.unwrap_or(u32::MAX),
    );

    let market_price_str = market_price
        .map(|p| {
            format!(
                "M:{} {}",
                p.separate_with_commas(),
                md.map(|md| { hm_ago_from_now(md.last_upload_time).dimmed() })
                    .unwrap_or_default()
            )
        })
        .unwrap_or_default();
    let crafting_price_str = crafting_price
        .map(|p| format!("C:{}", p.separate_with_commas(),))
        .unwrap_or_default();
    let diff_str = if let (Some(mp), Some(cp)) = (market_price, crafting_price) {
        format_num_diff(mp, cp).to_string()
    } else {
        String::new()
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
