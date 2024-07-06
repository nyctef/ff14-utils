use color_eyre::eyre::{eyre, Result};
use ff14_data::leve::{get_dawntrail_leves, Leve};
use ff14_utils::universalis::{get_market_data_lookup, price_up_to};
use itertools::Itertools;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    run().await
}

async fn run() -> Result<()> {
    let leves = get_dawntrail_leves()?;

    // dbg!(&leves);
    // dbg!(leves.len());
    //
    let ids = leves.iter().map(|l| l.item_id).collect_vec();

    let market_data = get_market_data_lookup(&ids).await?;

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
