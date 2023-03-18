use crate::model::*;
use chrono::{DateTime, TimeZone, Utc};
use color_eyre::eyre::Result;
use derive_more::Constructor;
use itertools::Itertools;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Constructor)]
pub struct ItemMarketData {
    pub item_id: ItemId,
    pub last_upload_time: DateTime<Utc>,
    pub listings: Vec<ItemMarketListing>,
    pub history: Vec<ItemMarketHistory>,
}

#[derive(Debug, Constructor)]
pub struct ItemMarketListing {
    pub price_per_item: u32,
    pub amount: u32,
    pub hq: bool,
}

#[derive(Debug, Constructor)]
pub struct ItemMarketHistory {
    pub price_per_item: u32,
    pub amount: u32,
    pub hq: bool,
}

#[derive(Debug, Deserialize)]
struct UniversalisMarketDataJson {
    pub items: HashMap<i32, UniversalisMarketDataItemJson>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UniversalisMarketDataItemJson {
    last_upload_time: i64,
    listings: Vec<UniversalisMarketListingJson>,
    recent_history: Vec<UniversalisMarketHistoryJson>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UniversalisMarketListingJson {
    price_per_unit: u32,
    quantity: u32,
    hq: bool,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UniversalisMarketHistoryJson {
    hq: bool,
    price_per_unit: u32,
    quantity: u32,
    on_mannequin: bool,
}

pub async fn get_market_data(ids: impl Into<&[ItemId]>) -> Result<Vec<ItemMarketData>> {
    let ids = ids.into();
    assert!(ids.len() > 1, "Universalis gives us results in a different format if we only query a single item, and we don't currently cope with that");
    let base = "https://universalis.app/api/v2";
    let world = "Moogle";
    let ids = ids.iter().map(|x| format!("{}", x)).collect_vec().join(",");
    let client = Client::new();
    let response = client
        .get(format!("{base}/{world}/{ids}"))
        .query(&[("entries", 10)])
        .query(&[(
            "fields",
            vec![
                "items.lastUploadTime",
                "items.listings.quantity",
                "items.listings.pricePerUnit",
                "items.listings.hq",
                "items.recentHistory.quantity",
                "items.recentHistory.pricePerUnit",
                "items.recentHistory.hq",
                "items.recentHistory.onMannequin",
            ]
            .join(","),
        )])
        .send()
        .await?
        .json::<UniversalisMarketDataJson>()
        .await?;

    Ok(response
        .items
        .iter()
        .map(|(&k, v)| {
            ItemMarketData::new(
                ItemId::new(k),
                Utc.timestamp_millis_opt(v.last_upload_time).unwrap(),
                v.listings
                    .iter()
                    .map(|l| ItemMarketListing::new(l.price_per_unit, l.quantity, l.hq))
                    .collect_vec(),
                v.recent_history
                    .iter()
                    .filter(|h| !h.on_mannequin)
                    .map(|h| ItemMarketHistory::new(h.price_per_unit, h.quantity, h.hq))
                    .collect_vec(),
            )
        })
        .collect_vec())
}
