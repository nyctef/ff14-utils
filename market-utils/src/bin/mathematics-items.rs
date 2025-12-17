use ff14_utils::scrip_compare::print_script_sink_compare;

#[tokio::main]
async fn main() {
    let items = [
        (20, "Turali Pigment"),
        (20, "Mastodon Pelt"),
        (20, "Everkeep Resin"),
        (20, "Insulating Varnish"),
        (20, "Double Duracoat"),
        (20, "Yollal Extract"),
        (10, "Shaaloani Coke"),
        (10, "Neo Abrasive"),
        (10, "Cronopio Skin"),
        (10, "Diatryma Pelt"),
        (10, "Dichromatic Compound"),
        (10, "Potsworn's Abrasive"),
        (10, "Pelupelu Yarn"),
        (10, "Purussaurus Skin"),
        (10, "Xbr'aal Varnish"),
        (10, "Airbright Coolant"),
        (10, "Glossy Dried Ether"),
    ];

    print_script_sink_compare(&items, 2000).await;
}
