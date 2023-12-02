use crafting_simulator::model::CraftStatus;
use crafting_simulator::presets::Presets as p;
use crafting_simulator::simulator::Simulator as s;

#[test]
fn can_craft_garnet_cotton_with_100_percent_quality() {
    let recipe = p::rlvl640_intermediate();
    let report = s::run_steps(
        p::l90_player_with_jhinga_biryani_hq_and_draught(),
        recipe,
        &[
            "Muscle Memory",
            "Manipulation",
            "Veneration",
            "Waste Not",
            "Groundwork",
            "Careful Synthesis",
            "Careful Synthesis",
            "Preparatory Touch",
            "Prudent Touch",
            "Prudent Touch",
            "Manipulation",
            "Prudent Touch",
            "Innovation",
            "Prudent Touch",
            "Basic Touch",
            "Standard Touch",
            "Advanced Touch",
            "Innovation",
            "Observe",
            "Focused Touch",
            "Great Strides",
            "Byregot's Blessing",
            "Basic Synthesis",
        ],
    );

    dbg!(&report);
    assert_eq!(CraftStatus::Success, report.status);
    assert!(report.final_state.quality >= recipe.quality_target);
}
