use crate::model::*;
use color_eyre::eyre::{eyre, Context, Result};
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::path::Path;
use tokio::fs::File;
use tokio_stream::StreamExt;

async fn read_csv(csv_path: &Path) -> Result<Vec<FxHashMap<String, String>>> {
    let mut reader = csv_async::AsyncReader::from_reader(
        File::open(csv_path)
            .await
            .wrap_err_with(|| eyre!("Couldn't open file {csv_path:?}"))?,
    );
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
                    Ok(((*field_name).to_string(), value.to_string()))
                })
                .collect()
        })
        .collect()
        .await;

    result
}

pub async fn read_recipes(csv_base_path: &Path) -> Result<Vec<Recipe>> {
    let rlvls = read_rlvls(csv_base_path).await?;

    read_csv(&csv_base_path.join("Recipe.csv"))
        .await?
        .iter()
        .map(|record| {
            let recipe_id = RecipeId::try_from(record.get("#").unwrap())?;

            let result_id: ItemId = record.get("Item{Result}").unwrap().try_into().unwrap();
            let result_amount: u32 = record.get("Amount{Result}").unwrap().parse().unwrap();
            let result = RecipeItem::new(result_id, result_amount);

            let mut ingredients = vec![];
            for i in 0..8 {
                let ingredient_field_name = &format!("Item{{Ingredient}}[{i}]");
                let ingredient_id = record.get(ingredient_field_name).unwrap();
                let amount_field_name = &format!("Amount{{Ingredient}}[{i}]");
                let amount: u32 = record.get(amount_field_name).unwrap().parse()?;
                if amount > 0 {
                    ingredients.push(RecipeItem::new(ItemId::try_from(ingredient_id)?, amount));
                }
            }

            let rlvl_id: RecipeLevelId =
                record.get("RecipeLevelTable").unwrap().try_into().unwrap();
            let rlvl = rlvls.iter().find(|rl| rl.rlvl == rlvl_id).unwrap().clone();

            let difficulty_factor: u16 = record.get("DifficultyFactor").unwrap().parse().unwrap();
            let quality_factor: u16 = record.get("QualityFactor").unwrap().parse().unwrap();
            let durability_factor: u16 = record.get("DurabilityFactor").unwrap().parse().unwrap();

            let difficulty = modify_by_factor(rlvl.base_difficulty, difficulty_factor);
            let durability = modify_by_factor(rlvl.base_durability, durability_factor);
            let quality_target = modify_by_factor(rlvl.base_quality_target, quality_factor);

            let required_craftsmanship: u16 =
                record.get("RequiredCraftsmanship").unwrap().parse()?;
            let required_control: u16 = record.get("RequiredControl").unwrap().parse()?;

            Ok(Recipe::new(
                recipe_id,
                ingredients,
                result,
                rlvl,
                difficulty,
                durability,
                quality_target,
                required_craftsmanship,
                required_control,
            ))
        })
        .collect()
}

/// where `factor` is an integer percentage (eg 100 = 100% = 1.0)
fn modify_by_factor(base: u16, factor: u16) -> u16 {
    // cast to u32 to avoid overflow
    ((base as u32 * factor as u32) / 100) as u16
}

pub async fn read_rlvls(csv_base_path: &Path) -> Result<Vec<RecipeLevel>, color_eyre::eyre::Error> {
    let rlvls: Vec<RecipeLevel> = read_csv(&csv_base_path.join("RecipeLevelTable.csv"))
        .await?
        .iter()
        .map(|record| {
            let rlvl = RecipeLevelId::try_from(record.get("#").unwrap()).unwrap();

            let progress_divider: u8 = record.get("ProgressDivider").unwrap().parse().unwrap();
            let progress_modifier: u8 = record.get("ProgressModifier").unwrap().parse().unwrap();
            let quality_divider: u8 = record.get("QualityDivider").unwrap().parse().unwrap();
            let quality_modifier: u8 = record.get("QualityModifier").unwrap().parse().unwrap();

            let base_difficulty: u16 = record.get("Difficulty").unwrap().parse().unwrap();
            let base_durability: u16 = record.get("Durability").unwrap().parse().unwrap();
            let base_quality_target: u16 = record.get("Quality").unwrap().parse().unwrap();

            let stars: u8 = record.get("Stars").unwrap().parse().unwrap();

            Ok(RecipeLevel::new(
                rlvl,
                progress_divider,
                progress_modifier,
                quality_divider,
                quality_modifier,
                base_difficulty,
                base_durability,
                base_quality_target,
                stars,
            ))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(rlvls)
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
            let can_be_hq = record.get("CanBeHq").unwrap() == "True";
            let equip_slot = record
                .get("EquipSlotCategory")
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let equip_slot = EquipSlotCategory::from(equip_slot).unwrap();

            Ok(Item::new(
                ItemId::try_from(item_id)?,
                item_name.clone(),
                name_singular.clone(),
                name_plural.clone(),
                ilvl.to_owned(),
                can_be_hq,
                equip_slot,
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
            let levels = (0..12)
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
