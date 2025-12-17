use color_eyre::eyre::{eyre, Context, Result};
use ff14_utils::scrip_compare::print_script_sink_compare;
use itertools::Itertools;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let mut target_scrip_count = 10_000;

    let args = env::args().collect_vec();
    match &args[1..] {
        [] => {}
        [count] => {
            target_scrip_count = count.parse::<u32>().wrap_err("Failed to parse count")?;
        }
        _ => return Err(eyre!("Usage: orange-scrips [script amount]")),
    }

    let items = [
        (8400, "Star Crew Jacket"),
        (4800, "Star Crew Gloves"),
        (7200, "Star Crew Trousers"),
        (4800, "Star Crew Boots"),
        (4800, "Star Captain Hat"),
        (8400, "Star Captain Coat"),
        (4800, "Star Captain Gloves"),
        (7200, "Star Captain Trousers"),
        (4800, "Star Captain Boots"),
        (6000, "The Faces We Wear - Reading Glasses"),
        (9600, "Ballroom Etiquette - Bearing Insult"),
        (6000, "Cosmic Exploration Framer's Kit"),
        (6000, "Cosmic Constructs Framer's Kit"),
        (
            3000,
            "The Faces We Wear - Ornamented Leather Eyepatch (Left)",
        ),
        (
            3000,
            "The Faces We Wear - Ornamented Leather Eyepatch (Right)",
        ),
        (4000, "Vacuum Suit Card"),
        (6000, "Namingway Card"),
        (6000, "Hey, Cid! Orchestrion Roll"),
        (6000, "The Airship Orchestrion Roll"),
        (6000, "Carrots of Passion Orchestrion Roll"),
        (3000, "Stellar Lamppost"),
        (3000, "Cosmochair"),
        (29000, "Interstellar Dhalmel Whistle"),
        (600, "Ruby Red Dye"),
        (600, "Cherry Pink Dye"),
        (600, "Carmine Red Dye"),
        (600, "Neon Pink Dye"),
        (600, "Bright Orange Dye"),
        (600, "Canary Yellow Dye"),
        (600, "Vanilla Yellow Dye"),
        (600, "Neon Yellow Dye"),
        (600, "Neon Green Dye"),
        (600, "Dragoon Blue Dye"),
        (600, "Turquoise Blue Dye"),
        (600, "Azure Blue Dye"),
        (600, "Violet Purple Dye"),
        (1500, "Gunmetal Black Dye"),
        (1500, "Pearl White Dye"),
        (1500, "Metallic Brass Dye"),
        (450, "Gatherer's Guerdon Materia XI"),
        (450, "Gatherer's Guile Materia XI"),
        (450, "Gatherer's Grasp Materia XI"),
        (450, "Craftsman's Competence Materia XI"),
        (450, "Craftsman's Cunning Materia XI"),
        (450, "Craftsman's Command Materia XI"),
        (900, "Gatherer's Guerdon Materia XII"),
        (900, "Gatherer's Guile Materia XII"),
        (900, "Gatherer's Grasp Materia XII"),
        (900, "Craftsman's Competence Materia XII"),
        (900, "Craftsman's Cunning Materia XII"),
        (900, "Craftsman's Command Materia XII"),
        (250, "Condensed Solution"),
        (1000, "Mason's Abrasive"),
    ];

    print_script_sink_compare(&items, target_scrip_count).await;

    Ok(())
}
