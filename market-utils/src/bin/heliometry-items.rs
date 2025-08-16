use ff14_utils::scrip_compare::print_script_sink_compare;

#[tokio::main]
async fn main() {
    let items = [
        (20, "Hydrophobic Preservative"),
        (20, "Shaaloani Coke"),
        (20, "Neo Abrasive"),
        (20, "Cronopio Skin"),
        (20, "Diatryma Pelt"),
        (20, "Dichromatic Compound"),
        (10, "Potsworn's Abrasive"),
        (10, "Pelupelu Yarn"),
        (10, "Purussaurus Skin"),
        (10, "Xbr'aal Varnish"),
        (10, "Airbright Coolant"),
        (10, "Glossy Dried Ether"),
    ];

    print_script_sink_compare(&items, 2000).await;
}
