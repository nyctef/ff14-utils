use color_eyre::eyre::Result;
use ff14_data::{lookup::ItemLookup, model::Item};
use ff14_utils::universalis::get_market_data_lookup;
use itertools::Itertools;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    run().await
}

async fn run() -> Result<()> {
    let lookup = ItemLookup::from_embedded().await?;
    let maps = lookup
        .matching(|i| i.name.starts_with("Timeworn") && i.name.ends_with("Map"))
        .collect_vec();

    // dbg!(&maps);
    //
    let ids = maps.iter().map(|l| l.id).collect_vec();

    let market_data = get_market_data_lookup(&ids).await?;

    let mut bottom_lines = maps
        .iter()
        .map(|map| -> Result<(u32, &Item)> {
            let market_price = market_data
                .get(&map.id)
                .and_then(|md| md.listings.first().map(|l| l.price_per_item))
                .unwrap_or(0);
            Ok((market_price, map))
        })
        .filter_map(|r| r.ok())
        .collect_vec();

    bottom_lines.sort_by_key(|l| l.0);

    for line in bottom_lines {
        println!("{} - {}", line.0, get_link(&line.1.name));
    }

    Ok(())
}

fn get_link(name: &str) -> String {
    let target = format!("https://ffxiv.consolegameswiki.com/wiki/{}", name);
    // OSC 8 escape code
    // https://github.com/Alhadis/OSC8-Adoption/
    // https://iterm2.com/3.2/documentation-escape-codes.html
    // format:
    // OSC 8 ; <params> ; <url> ; ST
    //    <text>
    // OSC 8 ; <params> ; ST
    return format!("\x1B]8;;{}\x07{}\x1B]8;;\x07", target, name);
}
