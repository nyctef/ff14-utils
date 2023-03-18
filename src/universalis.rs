use crate::model::*;
use color_eyre::eyre::{eyre, Result};
use reqwest::Client;

pub struct ItemMarketData {
    pub item_id: ItemId,
    pub item_name: String,
}

pub async fn get_market_data(ids: impl Into<&[ItemId]>) -> Result<Vec<ItemMarketData>> {
    let ids = ids.into();
    assert!(ids.len() > 1, "Universalis gives us results in a different format if we only query a single item, and we don't currently cope with that");
    let base = "https://universalis.app/api/v2";
    let world = "Moogle";
    let ids = ids
        .iter()
        .map(|x| format!("{}", x))
        .collect::<Vec<_>>()
        .join(",");
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
        .json::<serde_json::Value>()
        .await?;

    dbg!(response);

    Err(eyre!("TODO"))
}
