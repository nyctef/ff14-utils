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
        _ => return Err(eyre!("Usage: purple-scrip-items [script amount]")),
    }

    let items = [
        (250, "Craftsman's Competence Materia XI"),
        (250, "Craftsman's Cunning Materia XI"),
        (250, "Craftsman's Command Materia XI"),
        (200, "Craftsman's Competence Materia X"),
        (200, "Craftsman's Cunning Materia X"),
        (200, "Craftsman's Command Materia X"),
        (200, "Craftsman's Competence Materia IX"),
        (200, "Craftsman's Cunning Materia IX"),
        (200, "Craftsman's Command Materia IX"),
        (200, "Craftsman's Competence Materia VIII"),
        (200, "Craftsman's Cunning Materia VIII"),
        (200, "Craftsman's Command Materia VIII"),
        (200, "Craftsman's Competence Materia VII"),
        (200, "Craftsman's Cunning Materia VII"),
        (200, "Craftsman's Command Materia VII"),
        (200, "Craftsman's Competence Materia VI"),
        (200, "Craftsman's Cunning Materia VI"),
        (200, "Craftsman's Command Materia VI"),
        (200, "Craftsman's Competence Materia V"),
        (200, "Craftsman's Cunning Materia V"),
        (200, "Craftsman's Command Materia V"),
        (250, "Gripgel"),
        (125, "Immutable Solution"),
        (15, "Crafter's Delineation"),
    ];

    print_script_sink_compare(&items, target_scrip_count).await;

    Ok(())
}
