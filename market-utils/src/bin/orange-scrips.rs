use color_eyre::eyre::Result;
use ff14_data::lookup::{ItemLookup, RecipeLookup};
use ff14_utils::scrip_compare::print_scrip_compare;
use itertools::Itertools;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let items = ItemLookup::from_datamining_csv().await?;
    let recipes_lookup = RecipeLookup::from_datamining_csv().await?;

    let l100_collectables = items
        .matching(|i| i.ilvl == 690 && i.name.starts_with("Rarefied"))
        .collect_vec();

    let recipes = l100_collectables
        .iter()
        // only include items that have a recipe (ie skip gathering collectables)
        .filter_map(|i| recipes_lookup.recipe_for_item(i.id))
        .map(|r| r * 10)
        .collect_vec();

    print_scrip_compare(&items, &recipes_lookup, recipes, 144).await?;

    Ok(())
}
