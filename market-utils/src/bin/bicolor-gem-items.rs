use color_eyre::eyre::Result;
use ff14_utils::scrip_compare::print_script_sink_compare;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    run().await
}

async fn run() -> Result<()> {
    let items = [
        (100, "Turali Bicolor Gemstone Voucher"),
        (3, "Silver Lobo Hide"),
        (3, "Alpaca Fillet"),
        (3, "Megamaguey Pineapple"),
        (3, "Hammerhead Crocodile Skin"),
        (3, "Swampmonk Thigh"),
        (3, "Poison Frog Secretions"),
        (3, "Lesser Apollyon Shell"),
        (3, "Br'aax Hide"),
        (3, "Branchbearer Fruit"),
        (3, "Ty'aitya Wingblade"),
        (3, "Rroneek Fleece"),
        (3, "Rroneek Chuck"),
        (3, "Nopalitender Tuna"),
        (3, "Tumbleclaw Weeds"),
        (3, "Gomphotherium Skin"),
        (3, "Alexandrian Axe Beak Wing"),
        (3, "Gargantua Hide"),
    ];
    let target_scrip_count = 1500;
    print_script_sink_compare(&items, target_scrip_count).await;

    Ok(())
}
