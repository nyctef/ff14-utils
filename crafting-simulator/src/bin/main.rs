use crafting_simulator::{generator::RandomGenerator, presets::Presets as p};

fn main() {
    let target_recipe = p::rlvl640_gear();
    let player = p::l90_player_with_jhinga_biryani_hq();

    let mut generator = RandomGenerator::from_lengths(10, 30);

    dbg!(generator.generate());
}
