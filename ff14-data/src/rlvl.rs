use crate::{jsonext::JsonValueExt, model::*};
use color_eyre::eyre::Result;
use itertools::Itertools;
use serde_json::Value;

fn get_rlvls() -> Result<Vec<RecipeLevel>> {
    let leve_data: Value = serde_json::from_str(include_str!("../data/RecipeLevelTable.json"))?;
    let leve_data = leve_data.as_object().unwrap();
    let rlvls = leve_data.get("Results").unwrap().as_array().unwrap();

    let rlvls = rlvls
        .iter()
        .map(|r| RecipeLevel {
            rlvl: r.unwrap_i32("ID").into(),
            progress_divider: r.unwrap_u8("ProgressDivider"),
            progress_modifier: r.unwrap_u8("ProgressModifier"),
            quality_divider: r.unwrap_u8("QualityDivider"),
            quality_modifier: r.unwrap_u8("QualityModifier"),
            stars: r.unwrap_u8("Stars"),
        })
        .collect_vec();

    Ok(rlvls)
}

pub struct RlvlLookup {
    rlvls: Vec<RecipeLevel>,
}

impl RlvlLookup {
    fn new(rlvls: Vec<RecipeLevel>) -> RlvlLookup {
        assert!(rlvls
            .iter()
            .enumerate()
            // TODO: can we get these .into()s to not require the fully qualified syntax?
            .all(|(i, r)| i + 1 == <RecipeLevelId as Into<usize>>::into(r.rlvl)));
        RlvlLookup { rlvls }
    }

    pub fn by_id(&self, id: RecipeLevelId) -> &RecipeLevel {
        &self.rlvls[<RecipeLevelId as Into<usize>>::into(id) - 1]
    }

    pub fn rlvl(&self, id: usize) -> &RecipeLevel {
        &self.rlvls[id - 1]
    }

    pub fn get_rlvl_lookup() -> Result<RlvlLookup> {
        Ok(RlvlLookup::new(get_rlvls()?))
    }
}
