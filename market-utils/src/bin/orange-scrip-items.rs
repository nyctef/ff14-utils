use color_eyre::eyre::{eyre, Context, Result};
use ff14_utils::scrip_compare::print_script_sink_compare;
use itertools::Itertools;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

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

    print_script_sink_compare(&items, target_scrip_count).await;

    Ok(())
}
