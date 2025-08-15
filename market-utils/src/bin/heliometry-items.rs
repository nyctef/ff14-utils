use ff14_data::lookup::ItemLookup;
use ff14_utils::universalis::get_market_data;
use itertools::Itertools;

#[tokio::main]
async fn main() {
    let items_lookup = ItemLookup::from_datamining_csv()
        .await
        .expect("Failed to load item data");

    let items = [
        "Hydrophobic Preservative",
        "Shaaloani Coke",
        "Neo Abrasive",
        "Cronopio Skin",
        "Diatryma Pelt",
        "Dichromatic Compound",
    ];

    let items = items
        .iter()
        .filter_map(|&name| items_lookup.item_by_name_opt(name))
        .collect_vec();
    let item_ids = items.iter().map(|item| item.id).collect_vec();

    let market_data = get_market_data(&item_ids)
        .await
        .expect("Failed to fetch market data");

    for &item in &items {
        let price = market_data
            .iter()
            .find(|data| data.item_id == item.id)
            .and_then(|data| data.listings.first().map(|listing| listing.price_per_item));
        match price {
            Some(p) => println!("{}: {} gil", item.name, p),
            None => println!("{}: Price not available", item.name),
        }
    }
}
