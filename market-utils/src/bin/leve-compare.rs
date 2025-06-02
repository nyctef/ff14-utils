use color_eyre::eyre::{eyre, Result};
use ff14_data::leve::{get_relevant_leves, Leve};
use ff14_utils::{
    format_table::Table,
    universalis::{get_market_data_lookup, price_up_to},
};
use itertools::Itertools;
use thousands::Separable;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    run().await
}

async fn run() -> Result<()> {
    let leves = get_relevant_leves()?;

    // dbg!(&leves);
    // dbg!(leves.len());
    //
    let ids = leves.iter().map(|l| l.item_id).collect_vec();

    let market_data = get_market_data_lookup(&ids).await?;

    let mut bottom_lines = leves
        .iter()
        .map(|leve| -> Result<(u32, u32, &Leve)> {
            let hq_reward = leve.gil_reward * 2;
            let md = market_data.get(&leve.item_id);
            if md.is_none() {
                eprintln!("No market data for {}, skipping...", leve.item_name);
                return Err(eyre!("No market data for {}", leve.item_name));
            }
            let md = md.unwrap();
            let market_price = price_up_to(&md.listings, leve.item_count.into(), true)
                .map_err(|e| eyre!("{}", e))?;
            Ok((hq_reward, market_price, leve))
        })
        .filter_map(|r| r.ok())
        .collect_vec();

    bottom_lines.sort_by_key(|l| l.0 as i32 - l.1 as i32);

    let mut table = Table::<String, 3>::new();
    table.add_row([
        "Profit (gil)".to_string(),
        "".to_string(),
        "Leve".to_string(),
    ]);
    table.add_separator();

    for line in bottom_lines {
        let profit = line.0 as i32 - line.1 as i32;
        if profit < 0 {
            continue; // Skip negative profit lines
        }
        table.add_row([
            format!("{}", profit.separate_with_commas()),
            format!(
                "{}-{}",
                line.0.separate_with_commas(),
                line.1.separate_with_commas()
            ),
            format!(
                "{} ({} x{})",
                line.2.leve_name, line.2.item_name, line.2.item_count
            ),
        ]);
    }

    table.print();

    Ok(())
}
