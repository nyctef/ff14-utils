use color_eyre::eyre::{eyre, Context, Result};
use ff14_data::lookup::{ItemLookup, RecipeLookup};
use ff14_utils::scrip_compare::print_scrip_source_compare;
use itertools::Itertools;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let mut target_scrip_count = 4_000;

    let args = env::args().collect_vec();
    match &args[1..] {
        [] => {}
        [count] => {
            target_scrip_count = count.parse::<u32>().wrap_err("Failed to parse count")?;
        }
        _ => return Err(eyre!("Usage: orange-scrips [script amount]")),
    }

    let items = ItemLookup::from_datamining_csv().await?;
    let recipes_lookup = RecipeLookup::from_datamining_csv().await?;

    let l100_collectables = items
        .matching(|i| i.ilvl == 690 && i.name.starts_with("Rarefied"))
        .collect_vec();

    let recipes = l100_collectables
        .iter()
        // only include items that have a recipe (ie skip gathering collectables)
        .filter_map(|i| recipes_lookup.recipe_for_item(i.id))
        .collect_vec();

    print_scrip_source_compare(&items, &recipes_lookup, recipes, target_scrip_count).await?;

    Ok(())
}
