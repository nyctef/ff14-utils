use color_eyre::eyre::Result;
use ff14_data::lookup::{ItemLookup, MateriaLookup};
use ff14_utils::{time_utils::hm_ago_from_now, universalis};
use itertools::Itertools;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let items = ItemLookup::from_embedded().await?;
    let materia = MateriaLookup::from_embedded().await?;

    let all_materia = materia
        .iter()
        .flat_map(|m| m.materia_levels.iter())
        .filter(|ml| ml.level >= 11)
        .map(|ml| (ml.item_id, &items.item_by_id(ml.item_id).name))
        .collect_vec();

    let response =
        universalis::get_market_data(&all_materia.iter().map(|m| m.0).collect_vec()).await?;
    let data = response
        .iter()
        .map(|d| (&items.item_by_id(d.item_id).name, d))
        .sorted_by_key(|d| d.0)
        .collect_vec();

    for (name, market_data) in data {
        println!(
            "{:<40} cheapest {:>7}, last updated {}",
            format!("{name}:"),
            market_data.listings.first().expect(&format!("price for {name}")).price_per_item,
            hm_ago_from_now(market_data.last_upload_time)
        );
    }

    Ok(())
}
