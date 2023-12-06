use crate::model::*;
use color_eyre::eyre::Result;
use itertools::Itertools;
use serde_json::Value;

fn unwrap_i32(property_name: &'static str, value: &serde_json::value::Value) -> i32 {
    value.get(property_name).unwrap().as_i64().unwrap() as i32
}

fn unwrap_u8(property_name: &'static str, value: &serde_json::value::Value) -> u8 {
    value.get(property_name).unwrap().as_u64().unwrap() as u8
}

pub fn get_rlvl_lookup() -> Result<Vec<RecipeLevel>> {
    let leve_data: Value = serde_json::from_str(include_str!("../data/RecipeLevelTable.json"))?;
    let leve_data = leve_data.as_object().unwrap();
    let rlvls = leve_data.get("Results").unwrap().as_array().unwrap();

    let rlvls = rlvls
        .iter()
        .map(|r| RecipeLevel {
            rlvl: unwrap_i32("ID", r).into(),
            progress_divider: unwrap_u8("ProgressDivider", r),
            progress_modifier: unwrap_u8("ProgressModifier", r),
            quality_divider: unwrap_u8("QualityDivider", r),
            quality_modifier: unwrap_u8("QualityModifier", r),
            stars: unwrap_u8("Stars", r),
        })
        .collect_vec();

    Ok(rlvls)
}
