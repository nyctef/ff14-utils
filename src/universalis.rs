use crate::model::*;
use chrono::{DateTime, TimeZone, Utc};
use color_eyre::eyre::{eyre, Result};
use derive_more::Constructor;
use itertools::Itertools;
use reqwest::Client;
use serde::Deserialize;
use std::{cmp, collections::HashMap};

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

/// Note this is relatively naive: it assumes the latest universalis data is still valid +
/// it assumes that we'll happily buy stacks of up to 99 items even if we wanted less and
/// can easily resell the remainder. It also assumes that the incoming listings are sorted
/// by price (which seems to always be the case from what the universalis api gives us?)
fn price_up_to(
    listings: &[ItemMarketListing],
    amount_wanted: u32,
    hq_only: bool,
) -> Result<u32, String> {
    let mut amount_remaining = amount_wanted;
    let mut cumulative_price = 0;
    let mut listings = listings
        .iter()
        // https://stackoverflow.com/a/68522183
        .filter(|l| if hq_only { l.hq } else { true });

    while amount_remaining > 0 {
        match listings.next() {
            None => break,
            Some(next_listing) => {
                cumulative_price +=
                    next_listing.price_per_item * (cmp::min(amount_remaining, next_listing.amount));
                amount_remaining = amount_remaining.saturating_sub(next_listing.amount);
            }
        }
    }
    match amount_remaining {
        x if x > 0 => Err(format!("Couldn't find enough listings to satisfy demand")),
        _ => Ok(cumulative_price),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_price_up_to() {
        let listings = vec![
            ItemMarketListing::new(10, 1, false),
            ItemMarketListing::new(20, 1, true),
            ItemMarketListing::new(100, 10, false),
        ];

        // just buying the cheapest
        assert_eq!(Ok(10), price_up_to(&listings, 1, false));

        // buying the two cheapest
        assert_eq!(Ok(30), price_up_to(&listings, 2, false));

        // buying three cuts into the big stack
        // but we don't report the price of having to buy the whole thing
        assert_eq!(Ok(130), price_up_to(&listings, 3, false));

        // buying only HQ means we can't buy the cheapest
        assert_eq!(Ok(20), price_up_to(&listings, 1, true));

        // trying to buy more than is available
        assert_eq!(
            Err("Couldn't find enough listings to satisfy demand".to_owned()),
            price_up_to(&listings, 20, false)
        );
    }
}
