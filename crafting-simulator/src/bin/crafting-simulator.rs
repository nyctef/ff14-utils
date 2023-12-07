use color_eyre::{eyre::eyre, Result};
use crafting_simulator::{
    config,
    generator::{RandomFlip, RandomGenerator, RandomRemove},
    model::{CraftStatus, CraftingReport, PlayerStats, Recipe},
    presets::Presets as preset,
    simulator::Simulator as sim,
};
use derive_more::Constructor;
use itertools::Itertools;
use std::{
    cmp::{Ordering, Reverse},
    path::Path,
};

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

fn score_steps(player: PlayerStats, recipe: &Recipe, steps: Vec<&'static str>) -> Candidate {
    let report = sim::run_steps(player, recipe, &steps);
    let score = score_report(&recipe, &report);
    Candidate::new(steps, score)
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = parse_args()?;

    let config = config::read_jobs_from_config(Path::new("./jobs.toml"))?;

    let recipe = match args.recipe.as_str() {
        "l90_4s_mat" => Ok(preset::l90_4star_intermediate()),
        "l90_4s_gear" => Ok(preset::l90_4star_gear()),
        "l90_3s_mat" => Ok(preset::l90_3star_intermediate()),
        "l90_3s_gear" => Ok(preset::l90_3star_gear()),
        other => Err(eyre!("Unrecognised recipe type {}", other)),
    }?;

    preset::l90_4star_intermediate();

    let player = config
        .iter()
        .find(|(name, _)| *name == args.job_name)
        .expect("expected a job")
        .1;

    let random_generator = RandomGenerator::from_lengths(10, 30);
    let random_flip = RandomFlip::new();
    let random_remove = RandomRemove {};
    let mut best_per_generation: Vec<Candidate> = Vec::new();
    let mut candidates = (0..1000)
        .map(|_| score_steps(player, &recipe, random_generator.generate()))
        .collect_vec();

    let generations = args.generations.unwrap_or(1000);
    for _ in 0..generations {
        candidates.sort_by_key(|x| Reverse(x.score));

        best_per_generation.push(candidates[0].clone());

        candidates.drain(200..);
        let mutated_candidates = candidates
            .iter()
            .map(|c| random_flip.apply(&c.steps))
            .map(|steps| score_steps(player, &recipe, steps))
            .collect_vec();
        let simplified_candidates = candidates
            .iter()
            .map(|c| random_remove.apply(&c.steps))
            .map(|steps| score_steps(player, &recipe, steps))
            .collect_vec();
        candidates.extend(mutated_candidates);
        candidates.extend(simplified_candidates);
        candidates
            .extend((0..300).map(|_| score_steps(player, &recipe, random_generator.generate())));
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
    );
    Ok(())
}

struct Args {
    job_name: String,
    recipe: String,
    generations: Option<u32>,
}

fn parse_args() -> Result<Args> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!(
            r"
USAGE: crafting-simulator --job WVR --recipe l90_4s_mat

FLAGS:
    -j, --job           references a job listed in jobs.toml
    -r, --recipe        one of:
                            l90_4s_mat
                            l90_4s_gear
                            l90_3s_mat
                            l90_3s_gear
    -g, --generations   number of generations to search through
    -h, --help          show this message
    "
        );
        return Err(eyre!(""));
    }

    let args = Args {
        job_name: pargs.value_from_str(["-j", "--job"])?,
        recipe: pargs.value_from_str(["-r", "--recipe"])?,
        generations: pargs.opt_value_from_str(["-g", "--generations"])?,
    };

    let remaining = pargs.finish();
    if !remaining.is_empty() {
        return Err(eyre!("Unrecognised arguments: {:?}", remaining));
    }

    Ok(args)
}
