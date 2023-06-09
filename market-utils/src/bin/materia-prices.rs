use color_eyre::eyre::Result;
use ff14_utils::{csv, time_utils::hm_ago_from_now, universalis};
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let csv_base = PathBuf::from("../../ffxiv-datamining/csv");
    let items = csv::read_items(&csv_base).await?;
    let materia = csv::read_materia(&csv_base).await?;

    let items_by_id = items.iter().map(|i| (i.id, i)).collect::<FxHashMap<_, _>>();

    let all_materia = materia
        .iter()
        .flat_map(|m| m.materia_levels.iter())
        .filter(|ml| ml.level >= 9)
        .map(|ml| (ml.item_id, &items_by_id.get(&ml.item_id).unwrap().name))
        .collect_vec();

    let response =
        universalis::get_market_data(&all_materia.iter().map(|m| m.0).collect_vec()).await?;
    let data = response
        .iter()
        .map(|d| (&items_by_id.get(&d.item_id).unwrap().name, d))
        .sorted_by_key(|d| d.0)
        .collect_vec();

    for (name, market_data) in data {
        println!(
            "{:<40} cheapest {:>7}, last updated {}",
            format!("{name}:"),
            market_data.listings.first().unwrap().price_per_item,
            hm_ago_from_now(market_data.last_upload_time)
        );
    }

    Ok(())
}
