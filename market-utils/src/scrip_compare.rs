use crate::recipe_calculation::process_recipe_item;
use crate::universalis::get_market_data_lookup;
use ff14_data::lookup::{ItemLookup, RecipeLookup};
use ff14_data::model::Recipe;
use itertools::Itertools;
use thousands::Separable;

pub async fn print_scrip_compare(
    items: &ItemLookup,
    recipes_lookup: &RecipeLookup,
    recipes: Vec<Recipe>,
    scrip_per_item: u32,
) -> color_eyre::Result<()> {
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
            (line.crafting_price.expect("crafting price") / line.amount / scrip_per_item)
                .separate_with_commas()
        );
        // TODO: maybe a --detailed option to print all results?
    }
    Ok(())
}
