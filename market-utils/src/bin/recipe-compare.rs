use color_eyre::eyre::{eyre, Result};
use ff14_data::{
    lookup::{ItemLookup, RecipeLookup},
    model::*,
};
use ff14_utils::{
    recipe_calculation::{print_line_item, process_recipe_item},
    universalis::get_market_data_lookup,
};
use itertools::Itertools;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let items = ItemLookup::from_datamining_csv().await?;
    let recipes_lookup = RecipeLookup::from_datamining_csv().await?;

    let recipes = choose_recipes_from_args(&items, &recipes_lookup)?;

    let all_ids = recipes
        .iter()
        .flat_map(|r| r.relevant_item_ids(&recipes_lookup))
        .collect_vec();
    let market_data = get_market_data_lookup(&all_ids).await?;

    let mut bottom_lines = recipes
        .iter()
        .map(|r| {
            let (_, results) =
                process_recipe_item(0, &r.result, &items, &market_data, &recipes_lookup);
            results.into_iter().last().unwrap()
        })
        .collect_vec();

    bottom_lines.sort_by_key(|l| l.crafting_profit);

    for line in bottom_lines {
        print_line_item(&line);
    }

    Ok(())
}

fn choose_recipes_from_args<'a>(
    items: &ItemLookup,
    recipes: &'a RecipeLookup,
) -> Result<Vec<&'a Recipe>> {
    let args = env::args().collect_vec();
    let results;

    let look_up_recipe = |name| {
        items
            .name_containing(name)
            .filter_map(|i| recipes.recipe_for_item(i.id))
            .collect_vec()
    };

    match &args[1..] {
        [name] => {
            results = look_up_recipe(name);
        }
        _ => return Err(eyre!("Usage: recipe-compare [substr]")),
    }

    Ok(results)
}
