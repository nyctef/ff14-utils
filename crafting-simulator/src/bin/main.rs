use crafting_simulator::{
    generator::RandomGenerator,
    model::{CraftStatus, CraftingReport, PlayerStats, Recipe},
    presets::Presets as preset,
    simulator::Simulator as sim,
};
use derive_more::Constructor;
use itertools::Itertools;
use std::cmp::{Ordering, Reverse};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct CraftingScore {
    status: CraftStatus,
    durability: i16,
    progress_factor: u8,
    quality_factor: u8,
    steps: u8,
    cp: i16,
}

#[derive(Debug, Constructor, Clone)]
struct Candidate {
    steps: Vec<&'static str>,
    score: CraftingScore,
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

        let steps_diff = self.steps.cmp(&other.steps);
        if steps_diff != Ordering::Equal {
            return Some(steps_diff);
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
        steps: report.final_state.steps,
    }
}

fn score_steps(player: PlayerStats, recipe: Recipe, steps: Vec<&'static str>) -> Candidate {
    let report = sim::run_steps(player, recipe, &steps);
    let score = score_report(&recipe, &report);
    Candidate::new(steps, score)
}

fn main() {
    let recipe = preset::rlvl640_gear();
    let player = preset::l90_player_with_jhinga_biryani_hq_and_draught();

    let mut generator = RandomGenerator::from_lengths(10, 30);
    let mut best_per_generation: Vec<Candidate> = Vec::new();
    let mut candidates = (0..1000)
        .map(|_| score_steps(player, recipe, generator.generate()))
        .collect_vec();

    for generation in 0..10 {
        candidates.sort_by_key(|x| Reverse(x.score));

        best_per_generation.push(candidates[0].clone());

        candidates.drain(500..);
        candidates.extend((0..500).map(|_| score_steps(player, recipe, generator.generate())));
    }

    dbg!(best_per_generation.iter().map(|x| x.score).collect_vec());
    let best_overall = best_per_generation.iter().sorted_by_key(|x| x.score).last();
    dbg!(best_overall);
}
