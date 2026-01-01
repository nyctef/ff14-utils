use color_eyre::eyre::Result;
use ff14_data::lookup::{ItemLookup, RecipeLookup};

fn main() -> Result<()> {
    color_eyre::install()?;

    let items = ItemLookup::from_embedded()?;
    let recipes = RecipeLookup::from_embedded()?;

    for item in items.all() {
        if recipes.recipe_for_item(item.id).is_some() {
            println!("{}", item.name);
        }
    }

    Ok(())
}
