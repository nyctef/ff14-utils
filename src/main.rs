use color_eyre::eyre::Result;
use rustc_hash::FxHashMap;
use std::path::PathBuf;

mod csv;
mod model;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let csv_base = PathBuf::from("../ffxiv-datamining/csv");
    let items = csv::read_items(&csv_base).await?;
    let materia = csv::read_materia(&csv_base).await?;

    let items_by_id = items.iter().map(|i| (i.id, i)).collect::<FxHashMap<_, _>>();

    let all_materia = materia
        .iter()
        .flat_map(|m| m.materia_levels.iter())
        .filter(|ml| ml.level >= 9)
        .map(|ml| (ml.item_id, &items_by_id.get(&ml.item_id).unwrap().name));
    for m in all_materia {
        println!("{:>8}: {}", m.0, m.1);
    }

    Ok(())
}
