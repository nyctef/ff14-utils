use crate::{jsonext::JsonValueExt, model::*};
use color_eyre::eyre::Result;
use itertools::Itertools;
use serde_json::Value;

fn get_foods() -> Result<Vec<Food>> {
    let food_data: Value = serde_json::from_str(include_str!("../data/CraftingFoodItems.json"))?;

    let foods = food_data.unwrap_array("Results");
    let foods = foods
        .iter()
        .map(|f| Food {
            food_id: f.unwrap_i32("ID").into(),
            item_ui_category_name: f.unwrap_value("ItemUICategory").unwrap_string("Name"),
            name: f.unwrap_string("Name"),
            bonuses: f
                .unwrap_object("Bonuses")
                .iter()
                .map(|(name, b)| FoodBonus {
                    bonus_id: b.unwrap_i32("ID").into(),
                    name: name.to_owned(),
                    max: b.unwrap_u8("Max"),
                    max_hq: b.unwrap_u8("MaxHQ"),
                    value: b.unwrap_u8("Value"),
                    value_hq: b.unwrap_u8("ValueHQ"),
                })
                .collect_vec(),
        })
        .collect_vec();

    Ok(foods)
}

pub struct FoodLookup {
    foods: Vec<Food>,
}

impl FoodLookup {
    // TODO: reconsider if these kinds of methods should really be returning Results,
    // since we're unwrapping everything anyway (we have the json files checked in so
    // something would have to go pretty wrong for it to fail unexpectedly)
    pub fn get_food_lookup() -> Result<FoodLookup> {
        Ok(FoodLookup {
            foods: get_foods()?,
        })
    }

    pub fn by_name(&self, name: &str) -> Option<&Food> {
        // there aren't many foods, so this should be fast enough
        self.foods.iter().find(|f| f.name == name)
    }
}
