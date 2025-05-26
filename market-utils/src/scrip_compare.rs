use crate::recipe_calculation::process_recipe_item;
use crate::universalis::get_market_data_lookup;
use ff14_data::lookup::{ItemLookup, RecipeLookup};
use ff14_data::model::Recipe;
use itertools::Itertools;
use thousands::Separable;

fn scrip_per_item(ilvl: u32) -> u32 {
    match ilvl {
        // orange
        690 => 144,
        // purple
        685 => 198,
        675 => 171,
        665 => 157,
        656 => 142,
        650 => 128,
        560 => 114,
        548 => 108,
        _ => todo!("scrip per item for ilvl {}", ilvl),
    }
}

pub async fn print_scrip_source_compare(
    items: &ItemLookup,
    recipes_lookup: &RecipeLookup,
    recipes: Vec<&Recipe>,
    target_scrip_count: u32,
) -> color_eyre::Result<()> {
    let all_ids = recipes
        .iter()
        .flat_map(|r| r.relevant_item_ids(&recipes_lookup))
        .collect_vec();
    let market_data = get_market_data_lookup(&all_ids).await?;

    let result_lines = recipes
        .iter()
        .map(|&r| {
            // make enough copies of the recipe to get the target scrip count
            let scrip_per_item = scrip_per_item(items.item_by_id(r.result.item_id).ilvl);
            // https://stackoverflow.com/a/2745086
            let recipe_multiplier = (target_scrip_count + scrip_per_item - 1) / scrip_per_item;
            assert!(recipe_multiplier > 0, "recipe multiplier must be > 0");
            r * recipe_multiplier
        })
        .map(|r| process_recipe_item(0, &r.result, &items, &market_data, &recipes_lookup, false).1)
        .map(|r| r.into_iter().last().unwrap())
        .map(|l| {
            let scrip_value = scrip_per_item(items.item_by_id(l.item_id).ilvl);
            let crafting_price = l.crafting_price.expect("crafting price");
            let cost = crafting_price / l.amount / scrip_value;
            let text = format!(
                "{:<50}: {} or ~{} per scrip",
                l.name_and_amount,
                crafting_price.separate_with_commas(),
                cost.separate_with_commas()
            );
            (cost, text)
        })
        .sorted_by_key(|l| l.0);

    for (_cost, line) in result_lines {
        println!("{}", line);
    }
    // TODO: maybe a --detailed option to print all results?
    Ok(())
}
