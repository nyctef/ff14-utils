use crate::model::*;
use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::path::Path;
use tokio::fs::File;
use tokio_stream::StreamExt;

async fn read_csv(csv_path: &Path) -> Result<Vec<FxHashMap<String, String>>> {
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
        .collect::<FxHashMap<_, _>>();
    // the third row contains a type for each field eg int32,int32,CraftType,byte,Item,...
    let types_row = records.next().await.expect("types row")?;
    // map from a field index to the type of that field
    // TODO: is this actually going to get used?
    let _types_lookup = types_row.iter().enumerate().collect::<FxHashMap<_, _>>();

    let result: Result<Vec<FxHashMap<String, String>>> = records
        .map(|record| {
            let record = record?;

            field_name_lookup
                .iter()
                .map(|(field_name, field_offset)| {
                    let value = record.get(*field_offset).ok_or_else(|| {
                        eyre!(
                            "Failed to get field {field_name} (offset {field_offset}) from csv row"
                        )
                    })?;
                    Ok((field_name.to_string(), value.to_string()))
                })
                .collect()
        })
        .collect()
        .await;

    result
}

#[allow(dead_code)]
pub async fn read_recipes(csv_base_path: &Path) -> Result<Vec<Recipe>> {
    read_csv(&csv_base_path.join("Recipe.csv"))
        .await?
        .iter()
        .map(|record| {
            let recipe_id = RecipeId::try_from(record.get("#").unwrap())?;

            let result_id: ItemId = record.get("Item{Result}").unwrap().try_into().unwrap();
            let result_amount: u32 = record.get("Amount{Result}").unwrap().parse().unwrap();
            let result = RecipeItem::new(result_id, result_amount);

            let mut ingredients = vec![];
            for i in 0..10 {
                let ingredient_field_name = &format!("Item{{Ingredient}}[{i}]");
                let ingredient_id = record.get(ingredient_field_name).unwrap();
                let amount_field_name = &format!("Amount{{Ingredient}}[{i}]");
                let amount: u32 = record.get(amount_field_name).unwrap().parse()?;
                if amount > 0 {
                    ingredients.push(RecipeItem::new(ItemId::try_from(ingredient_id)?, amount))
                }
            }

            Ok(Recipe::new(recipe_id, ingredients, result))
        })
        .collect()
}

pub async fn read_items(csv_base_path: &Path) -> Result<Vec<Item>> {
    read_csv(&csv_base_path.join("Item.csv"))
        .await?
        .iter()
        .map(|record| {
            let item_id = record.get("#").unwrap();
            let item_name = record.get("Name").unwrap();
            let name_singular = record.get("Singular").unwrap();
            let name_plural = record.get("Plural").unwrap();
            let ilvl: u32 = record.get("Level{Item}").unwrap().parse().unwrap();

            Ok(Item::new(
                ItemId::try_from(item_id)?,
                item_name.to_owned(),
                name_singular.to_owned(),
                name_plural.to_owned(),
                ilvl.to_owned(),
            ))
        })
        .collect()
}

pub async fn read_materia(csv_base_path: &Path) -> Result<Vec<Materia>> {
    read_csv(&csv_base_path.join("Materia.csv"))
        .await?
        .iter()
        .map(|record| {
            let item_id = record.get("#").unwrap();
            let levels = (0..10)
                .map(|i| {
                    let item_id =
                        ItemId::try_from(record.get(&format!("Item[{i}]")).unwrap()).unwrap();
                    let value = record
                        .get(&format!("Value[{i}]"))
                        .unwrap()
                        .parse::<i16>()
                        .unwrap();

                    MateriaLevel::new(item_id, i + 1, value)
                })
                .filter(|ml| ml.item_id != ItemId::ZERO)
                .collect_vec();

            Ok(Materia::new(MateriaId::try_from(item_id)?, levels))
        })
        .collect()
}
