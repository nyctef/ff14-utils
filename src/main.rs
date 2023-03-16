use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{eyre, Result};
use model::{ItemId, Recipe, RecipeId, RecipeItem};
use tokio::fs::File;
use tokio_stream::StreamExt;

mod model;

async fn read_csv(csv_path: &Path) -> Result<Vec<HashMap<String, String>>> {
    // Function reads CSV file that has column named "region" at second position (index = 1).
    // It writes to new file only rows with region equal to passed argument
    // and removes region column.
    let mut reader = csv_async::AsyncReader::from_reader(File::open(csv_path).await?);
    let mut records = reader.records();
    // the first row is always of the form key,0,1,2,...
    // The csv parser assumes that row is a header row and discards it.

    // the actual header row is next: this contains the names of the fields
    let header_row = records.next().await.expect("header row")?;
    // map from a field name to the index of that field in the results
    let field_name_lookup = header_row
        .iter()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect::<HashMap<_, _>>();
    // the third row contains a type for each field eg int32,int32,CraftType,byte,Item,...
    let types_row = records.next().await.expect("types row")?;
    // map from a field index to the type of that field
    // TODO: is this actually going to get used?
    let _types_lookup = types_row.iter().enumerate().collect::<HashMap<_, _>>();

    let result: Result<Vec<HashMap<String, String>>> = records
        .map(|record| {
            let record = record?;

            field_name_lookup
                .iter()
                .map(|(field_name, field_offset)| {
                    let value = record.get(*field_offset).ok_or(eyre!(
                        "Failed to get field {field_name} (offset {field_offset}) from csv row"
                    ))?;
                    Ok((field_name.to_string(), value.to_string()))
                })
                .collect()
        })
        .collect()
        .await;

    result
}

async fn read_recipes(csv_base_path: &Path) -> Result<Vec<Recipe>> {
    read_csv(&csv_base_path.join("Recipe.csv"))
        .await?
        .iter()
        .map(|record| {
            let recipe_id = record.get("#").unwrap();
            let mut items = vec![];
            for i in 0..10 {
                let ingredient_field_name = &format!("Item{{Ingredient}}[{i}]");
                let ingredient_id = record.get(ingredient_field_name).unwrap();
                let amount_field_name = &format!("Amount{{Ingredient}}[{i}]");
                let amount: u8 = record.get(amount_field_name).unwrap().parse()?;
                if amount > 0 {
                    items.push(RecipeItem::new(ItemId::try_from(ingredient_id)?, amount))
                }
            }
            Ok(Recipe::new(RecipeId::try_from(recipe_id)?, items))
        })
        .collect()
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let csv_base = PathBuf::from("../ffxiv-datamining/csv");
    let recipes = read_recipes(&csv_base).await?;

    dbg!(recipes.iter().take(10).collect::<Vec<_>>());

    Ok(())
}
