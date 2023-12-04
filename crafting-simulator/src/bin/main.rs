use crafting_simulator::{
    generator::{RandomFlip, RandomGenerator, RandomRemove},
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

        let quality_diff = self
            .quality_factor
            .min(100)
            .cmp(&other.quality_factor.min(100));
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
    let recipe = preset::rlvl555_collectible();
    let player = preset::l90_player_2();

    let random_generator = RandomGenerator::from_lengths(10, 30);
    let random_flip = RandomFlip::new();
    let random_remove = RandomRemove {};
    let mut best_per_generation: Vec<Candidate> = Vec::new();
    let mut candidates = (0..1000)
        .map(|_| score_steps(player, recipe, random_generator.generate()))
        .collect_vec();

    for _ in 0..1000 {
        candidates.sort_by_key(|x| Reverse(x.score));

        best_per_generation.push(candidates[0].clone());

        candidates.drain(200..);
        let mutated_candidates = candidates
            .iter()
            .map(|c| random_flip.apply(&c.steps))
            .map(|steps| score_steps(player, recipe, steps))
            .collect_vec();
        let simplified_candidates = candidates
            .iter()
            .map(|c| random_remove.apply(&c.steps))
            .map(|steps| score_steps(player, recipe, steps))
            .collect_vec();
        candidates.extend(mutated_candidates);
        candidates.extend(simplified_candidates);
        candidates
            .extend((0..300).map(|_| score_steps(player, recipe, random_generator.generate())));
    }

    // dbg!(best_per_generation.iter().map(|x| x.score).collect_vec());
    let best_overall = best_per_generation
        .iter()
        .sorted_by_key(|x| x.score)
        .last()
        .unwrap();
    dbg!(&best_overall.score);

    println!();
    println!(
        "{}",
        best_overall
            .steps
            .iter()
            .map(|s| format!("/ac \"{}\"", s))
            .join("\n")
    )
}
