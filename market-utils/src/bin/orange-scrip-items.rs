use color_eyre::eyre::{eyre, Context, Result};
use ff14_data::lookup::ItemLookup;
use ff14_utils::universalis::get_market_data;
use itertools::Itertools;
use std::env;
use thousands::Separable;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let items_lookup = ItemLookup::from_datamining_csv()
        .await
        .expect("Failed to load item data");

    let mut target_scrip_count = 4_000;

    let args = env::args().collect_vec();
    match &args[1..] {
        [] => {}
        [count] => {
            target_scrip_count = count.parse::<u32>().wrap_err("Failed to parse count")?;
        }
        _ => return Err(eyre!("Usage: orange-scrip-items [script amount]")),
    }

    let items = [
        (15, "Queso Fresco"),
        (15, "Woolback Loin"),
        (15, "Cassava"),
        (15, "Splendid Mate Leaves"),
        (15, "Aji Amarillo"),
        (125, "Condensed Solution"),
        (10, "Rumpless Chicken"),
        (10, "Navel Orange"),
        (10, "Wild Coffee Beans"),
        (10, "Brown Cardamom"),
        (10, "Royal Lobster"),
        (500, "Craftsman's Command Materia XII"),
        (500, "Craftsman's Competence Materia XII"),
        (500, "Craftsman's Cunning Materia XII"),
    ];

    let items = items
        .iter()
        .filter_map(|(scrip, name)| Some(*scrip).zip(items_lookup.item_by_name_opt(name)))
        .collect_vec();
    let item_ids = items.iter().map(|(_, item)| item.id).collect_vec();

    let market_data = get_market_data(&item_ids)
        .await
        .expect("Failed to fetch market data");

    let mut results = items
        .into_iter()
        .map(|(scrip_cost, item)| {
            let item_count = target_scrip_count / scrip_cost;
            let prices = market_data.iter().find(|data| data.item_id == item.id);
            // let buy_price = prices.and_then(|data| price_up_to(&data.listings, item_count, false).ok());
            // for the purposes of selling, though, we're going to undercut
            // whatever the cheapest price currently is
            let sell_price = prices
                .and_then(|data| data.listings.first().map(|listing| listing.price_per_item))
                .map(|p| p * item_count);
            match sell_price {
                Some(p) => (
                    p / target_scrip_count,
                    format!(
                        "{}x {}: {} gil (~{} per scrip)",
                        item_count,
                        item.name,
                        p.separate_with_commas(),
                        (p / target_scrip_count).separate_with_commas()
                    ),
                ),
                None => (0, format!("{}: Price not available", item.name)),
            }
        })
        .collect_vec();

    results.sort_by_key(|(per_scrip, _)| -(*per_scrip as i64));

    for (_, line) in results {
        println!("{}", line);
    }

    Ok(())
}
