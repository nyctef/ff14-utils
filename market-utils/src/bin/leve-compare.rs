use color_eyre::eyre::{eyre, Result};
use ff14_utils::{
    model::ItemId,
    universalis::{get_market_data_lookup, price_up_to},
};
use itertools::Itertools;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    run().await
}

#[derive(Debug)]
struct Leve {
    quest_level: u8,
    item_id: ItemId,
    item_name: String,
    item_count: u8,
    gil_reward: u32,
    leve_name: String,
}

async fn run() -> Result<()> {
    let leve_data: Value = serde_json::from_str(include_str!("../../data/Leve.json"))?;
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

    // dbg!(&leves);
    // dbg!(leves.len());
    //
    let ids = leves.iter().map(|l| l.item_id).collect_vec();
    let market_data = get_market_data_lookup(&ids[..]).await?;

    let mut bottom_lines = leves
        .iter()
        .map(|leve| -> Result<(u32, u32, &Leve)> {
            let hq_reward = leve.gil_reward * 2;
            let md = market_data.get(&leve.item_id).unwrap();
            let market_price = price_up_to(&md.listings, leve.item_count.into(), true)
                .map_err(|e| eyre!("{}", e))?;
            Ok((hq_reward, market_price, leve))
        })
        .filter_map(|r| r.ok())
        .collect_vec();

    bottom_lines.sort_by_key(|l| l.0 as i32 - l.1 as i32);

    for line in bottom_lines {
        println!(
            "{} - {} - {} - {} ({} x{})",
            line.0 as i32 - line.1 as i32,
            line.0,
            line.1,
            line.2.leve_name,
            line.2.item_name,
            line.2.item_count
        );
    }

    Ok(())
}
