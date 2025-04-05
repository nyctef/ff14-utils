use ff14_utils::universalis::get_market_data;
use ff14_data::lookup::ItemLookup;

#[tokio::main]
async fn main() {
    let items_lookup = ItemLookup::from_datamining_csv().await.expect("Failed to load item data");

    let items = [
        "Hydrophobic Preservative",
        "Shaaloani Coke",
        "Neo Abrasive",
        "Cronopio Skin",
        "Diatryma Pelt",
        "Dichromatic Compound",
    ];

    let item_ids: Vec<_> = items
        .iter()
        .filter_map(|&name| items_lookup.item_by_name_opt(name).map(|item| item.id))
        .collect();

    match get_market_data(&item_ids).await {
        Ok(market_data) => {
            println!("Prices for items bought with tomes of heliometry:");
            for &name in &items {
                let price = items_lookup
                    .item_by_name_opt(name)
                    .and_then(|item| {
                        market_data
                            .iter()
                            .find(|data| data.item_id == item.id)
                            .and_then(|data| data.listings.first().map(|listing| listing.price_per_item))
                    });
                match price {
                    Some(p) => println!("{}: {} gil", name, p),
                    None => println!("{}: Price not available", name),
                }
            }
        }
        Err(err) => {
            eprintln!("Failed to fetch market data: {}", err);
        }
    }
}