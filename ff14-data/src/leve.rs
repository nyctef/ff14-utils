use crate::item::ItemId;
use color_eyre::eyre::Result;
use itertools::Itertools;
use serde_json::Value;

#[derive(Debug)]
pub struct Leve {
    pub quest_level: u8,
    pub item_id: ItemId,
    pub item_name: String,
    pub item_count: u8,
    pub gil_reward: u32,
    pub leve_name: String,
}

pub fn get_endwalker_leves() -> Result<Vec<Leve>> {
    let leve_data: Value = serde_json::from_str(include_str!("../data/Leve.json"))?;
    let leve_data = leve_data.as_object().unwrap();
    let leves = leve_data.get("Results").unwrap().as_array().unwrap();
    let leves = leves
        .iter()
        .map(|l| Leve {
            quest_level: l.get("ClassJobLevel").unwrap().as_u64().unwrap() as u8,
            item_id: ItemId::new(
                l.get("CraftLeve")
                    .unwrap()
                    .get("Item0")
                    .unwrap()
                    .get("ID")
                    .unwrap()
                    .as_u64()
                    .unwrap_or(0) as i32,
            ),
            item_name: l
                .get("CraftLeve")
                .unwrap()
                .get("Item0")
                .unwrap()
                .get("Name")
                .unwrap()
                .as_str()
                .unwrap_or("")
                .to_string(),
            item_count: l
                .get("CraftLeve")
                .unwrap()
                .get("ItemCount0")
                .unwrap()
                .as_u64()
                .unwrap_or(0) as u8,
            gil_reward: l.get("GilReward").unwrap().as_u64().unwrap() as u32,
            leve_name: l.get("Name").unwrap().as_str().unwrap().to_string(),
        })
        .filter(|l| {
            l.gil_reward > 0
                && l.quest_level >= 80
                && l.item_count > 0
                && l.item_name != ""
                && l.item_id != ItemId::ZERO
        })
        .collect_vec();

    Ok(leves)
}
