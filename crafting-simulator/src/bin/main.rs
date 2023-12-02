use std::cmp::Ordering;

use crafting_simulator::{
    generator::RandomGenerator,
    model::{CraftStatus, CraftingReport, Recipe},
    presets::Presets as preset,
    simulator::Simulator as sim,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct CraftingScore {
    status: CraftStatus,
    durability: i16,
    progress_factor: u8,
    quality_factor: u8,
    cp: i16,
}

impl PartialOrd for CraftingScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.status == CraftStatus::Success && other.status != CraftStatus::Success {
            return Some(Ordering::Greater);
        }
        if self.status != CraftStatus::Success && other.status == CraftStatus::Success {
            return Some(Ordering::Less);
        }

        let quality_diff = self.quality_factor.cmp(&other.quality_factor);
        if quality_diff != Ordering::Equal {
            return Some(quality_diff);
        }

        return Some(Ordering::Equal);
    }
}

impl Ord for CraftingScore {
    fn cmp(&self, other: &Self) -> Ordering {
        Self::partial_cmp(&self, other).unwrap()
    }
}

fn score_report(recipe: &Recipe, report: &CraftingReport) -> CraftingScore {
    CraftingScore {
        status: report.status,
        durability: report.final_state.durability,
        progress_factor: (report.final_state.progress as u32 * 100 / recipe.difficulty as u32)
            as u8,
        quality_factor: (report.final_state.quality as u32 * 100 / recipe.quality_target as u32)
            as u8,
        cp: report.final_state.cp,
    }
}

fn main() {
    let target_recipe = preset::rlvl640_gear();
    let player = preset::l90_player_with_jhinga_biryani_hq_and_draught();

    let mut generator = RandomGenerator::from_lengths(10, 30);

    let best = (0..100_000)
        .map(|_| {
            let steps = generator.generate();
            let report = sim::run_steps(player, target_recipe, &steps);
            let score = score_report(&target_recipe, &report);
            (steps, report, score)
        })
        .max_by_key(|x| x.2);

    dbg!(best);
}
