use color_eyre::eyre::Result;
use ff14_data::lookup::{ItemLookup, RecipeLookup};
use ff14_utils::csv;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let csv_base = PathBuf::from("../../ffxiv-datamining/csv");
    let items = ItemLookup::new(csv::read_items(&csv_base).await?);
    let recipes = RecipeLookup::new(csv::read_recipes(&csv_base).await?);

    for item in items.all() {
        if let Some(_) = recipes.recipe_for_item(item.id) {
            println!("{}", item.name);
        }
    }

    Ok(())
}
