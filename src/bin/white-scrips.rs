use color_eyre::eyre::Result;
use ff14_utils::{csv, model::*};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let csv_base = PathBuf::from("../ffxiv-datamining/csv");
    let items = csv::read_items(&csv_base).await?;
    let recipes = csv::read_recipes(&csv_base).await?;

    let pancakes = items
        .iter()
        .find(|i| i.name == "Rarefied Giant Popoto Pancakes")
        .unwrap();
    let recipe = recipes
        .iter()
        .find(|r| r.result.item_id == pancakes.id)
        .unwrap();

    println!("{}:", format_recipe_item(&recipe.result, pancakes));
    for ri in &recipe.ingredients {
        let i = items.iter().find(|i| i.id == ri.item_id).unwrap();
        println!("\t{}", format_recipe_item(ri, i))
    }

    Ok(())
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
