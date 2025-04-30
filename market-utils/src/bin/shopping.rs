use ff14_data::lookup::ItemLookup;
use ff14_utils::universalis::get_market_data_lookup;
use itertools::Itertools;
use std::io::{self, BufRead};
use thousands::Separable;

#[tokio::main]
async fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Read shopping list from stdin
    let mut shopping_list = Vec::new();
    for line in handle.lines() {
        if let Ok(entry) = line {
            if entry.trim().is_empty() {
                break;
            }
            let parts = entry.splitn(2, ' ').collect_vec();
            if parts.len() < 2 {
                eprintln!(
                    "Warning: Expected entry '{}' to be '<quantity> <item name>', skipping",
                    entry
                );
                continue;
            }
            if let Ok(quantity) = parts[0].parse::<u32>() {
                shopping_list.push((parts[1].to_string(), quantity));
            } else {
                eprintln!(
                    "Warning: Expected quantity '{}' to be a number, skipping",
                    parts[0]
                );
            }
        }
    }

    // Resolve item names to ItemId
    let item_lookup = ItemLookup::from_datamining_csv().await.unwrap();
    let mut resolved_items = Vec::new();
    for (name, quantity) in &shopping_list {
        if let Some(item) = item_lookup.item_by_name_opt(name) {
            resolved_items.push((item.id, *quantity));
        } else {
            eprintln!("Warning: Item '{}' not found in lookup", name);
        }
    }

    let item_ids = resolved_items.iter().map(|(id, _)| *id).collect_vec();

    eprintln!("Fetching market data for {} items...", item_ids.len());
    let market_data = get_market_data_lookup(&item_ids).await.unwrap();

    let mut total_cost = 0;
    let mut max_name_width = "Item Name".len();
    let mut max_quantity_width = "Quantity".len();
    let mut max_cost_width = "Cost (gil)".len();

    let mut table_rows = Vec::new();

    for (item_id, quantity) in &resolved_items {
        let name = &item_lookup.item_by_id(*item_id).name;
        let cost = market_data
            .get(item_id)
            .and_then(|data| data.listings.first().map(|listing| listing.price_per_item))
            .unwrap_or(0) as i64
            * *quantity as i64;

        total_cost += cost;
        let formatted_cost = cost.separate_with_commas();

        max_name_width = max_name_width.max(name.len());
        max_quantity_width = max_quantity_width.max(quantity.to_string().len());
        max_cost_width = max_cost_width.max(formatted_cost.len());

        table_rows.push((name, quantity, formatted_cost));
    }

    let total_width = max_name_width + max_quantity_width + max_cost_width + 4;
    println!(
        "{:<width_name$}  {:>width_quantity$}  {:>width_cost$}",
        "Item Name",
        "Quantity",
        "Cost (gil)",
        width_name = max_name_width,
        width_quantity = max_quantity_width,
        width_cost = max_cost_width
    );
    println!("{}", "-".repeat(total_width));

    for (name, quantity, formatted_cost) in table_rows {
        println!(
            "{:<width_name$}  {:>width_quantity$}  {:>width_cost$}",
            name,
            quantity,
            formatted_cost,
            width_name = max_name_width,
            width_quantity = max_quantity_width,
            width_cost = max_cost_width
        );
    }

    println!("{}", "-".repeat(total_width));
    println!(
        "{:<width_name$}  {:>width_quantity$}  {:>width_cost$}",
        "Total",
        "",
        total_cost.separate_with_commas(),
        width_name = max_name_width,
        width_quantity = max_quantity_width,
        width_cost = max_cost_width
    );
}
