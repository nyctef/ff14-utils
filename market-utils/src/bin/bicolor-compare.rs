use color_eyre::eyre::{eyre, Result};
use ff14_data::lookup::ItemLookup;
use ff14_utils::universalis::{get_market_data_lookup, price_up_to};
use itertools::Itertools;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    run().await
}

async fn run() -> Result<()> {
    let items = [
        "Silver Lobo Hide",
        "Alpaca Fillet",
        "Megamaguey Pineapple",
        "Hammerhead Crocodile Skin",
        "Swampmonk Thigh",
        "Poison Frog Secretions",
        "Lesser Apollyon Shell",
        "Br'aax Hide",
        "Branchbearer Fruit",
        "Ty'aitya Wingblade",
        "Rroneek Fleece",
        "Rroneek Chuck",
        "Nopalitender Tuna",
        "Tumbleclaw Weeds",
        "Gomphotherium Skin",
        "Axe Beak Wing",
        "Gargantua Hide",
    ];

    let item_lookup = ItemLookup::from_datamining_csv().await?;

    let items = items
        .iter()
        .map(|item_name| item_lookup.item_by_name(item_name))
        .collect_vec();

    // dbg!(&items);

    let ids = items.iter().map(|i| i.id).collect_vec();
    let market_data = get_market_data_lookup(&ids).await?;

    for item in items {
        let md = market_data.get(&item.id);
        if md.is_none() {
            eprintln!("No market data for {}, skipping...", item.name);
            continue;
        }
        let md = md.unwrap();
        let market_price = price_up_to(&md.listings, 1, false).map_err(|e| eyre!("{}", e))?;
        println!("{} - {}", item.name, market_price);
    }

    Ok(())
}
