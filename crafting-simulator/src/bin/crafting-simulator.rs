use color_eyre::{eyre::eyre, Result};
use crafting_simulator::{
    buffs::apply_buff_hq,
    config,
    generator::{RandomFlip, RandomGenerator, RandomRemove},
    model::{CraftStatus, CraftingReport, PlayerStats, Recipe},
    presets::Presets as preset,
    simulator::Simulator as sim,
};

use derive_more::Constructor;

use itertools::Itertools;
use rand::{thread_rng, Rng};
use std::{
    cmp::{Ordering, Reverse},
    fmt::Display,
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct CraftingScore {
    status: CraftStatus,
    durability: i16,
    progress_factor: u8,
    quality_factor: u8,
    step_count: u8,
    cp: i16,
}

impl Display for CraftingScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:?}: {}dur {}prog {}q {}steps {}cp",
            self.status,
            self.durability,
            self.progress_factor,
            self.quality_factor,
            self.step_count,
            self.cp
        ))
    }
}

impl CraftingScore {
    fn as_num(&self) -> f64{
        let mut score = 0_f64;

        // we build up `score` using various orders of magnitude
        // to ensure that some factors dominate others. When
        // plotting the score, taking the log of the score
        // will be necessary to make the chart look reasonable.

        // 0_000_000_xxx: cp diff
        // 0_000_0xx_000: steps diff
        // 0_0xx_000_000: quality diff
        // x_000_000_000: was crafting a success

        if self.status == CraftStatus::Success {
            score += 1_000_000_000.;
        }

        score += (self.quality_factor.min(100) as f64) * 1_000_000.;

        // if progress and quality are satisfied, try improving some other aspect
        // to provide more room for future improvements

        let steps_score = (100. - self.step_count as f64).max(0.);
        score += steps_score * 1_000.;

        let cp_remaining_score = (1000. - self.cp as f64).max(0.);
        score += cp_remaining_score;

        score
    }
}

#[derive(Debug, Constructor, Clone)]
struct Candidate {
    steps: Vec<&'static str>,
    score: CraftingScore,
    actual_steps: Vec<&'static str>,
}

impl PartialOrd for CraftingScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_num().partial_cmp(&other.as_num())
    }
}

impl Ord for CraftingScore {
    fn cmp(&self, other: &Self) -> Ordering {
        // technically we're comparing floats so this could actually panic in theory,
        // but that should only happen on NaNs
        Self::partial_cmp(self, other).unwrap()
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
        step_count: report.final_state.steps,
    }
}

fn score_steps(player: PlayerStats, recipe: &Recipe, steps: Vec<&'static str>) -> Candidate {
    let report = sim::run_steps(player, recipe, &steps);
    let score = score_report(recipe, &report);
    Candidate::new(steps, score, report.step_log)
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let ctrlc_pressed = Arc::new(AtomicBool::new(false));
    let ctrlc_sender = ctrlc_pressed.clone();
    ctrlc::set_handler(move || ctrlc_sender.store(true, SeqCst))?;

    let args = parse_args()?;

    let config = config::read_jobs_from_config(Path::new("./jobs.toml"))?;

    // TODO: make these pattern matches part of the argument parsing itself?
    let recipe = match args.recipe.as_str() {
        "l90_4s_mat" => Ok(preset::l90_4star_intermediate()),
        "l90_4s_gear" => Ok(preset::l90_4star_gear()),
        "l90_3s_mat" => Ok(preset::l90_3star_intermediate()),
        "l90_3s_gear" => Ok(preset::l90_3star_gear()),
        "l90_relic_tier3" => Ok(preset::l90_relic_tier3()),
        other => Err(eyre!("Unrecognised recipe type {}", other)),
    }?;
    let food = args.food.map(|f| match f.as_str() {
        "tsai_tou" => Ok(preset::tsai_tou_vounou()),
        "jhinga_biryani" => Ok(preset::jhinga_biryani()),
        other => Err(eyre!("Unrecognised food type {}", other)),
    });
    let potion = args.potion.map(|f| match f.as_str() {
        "cunning_draught" => Ok(preset::cunning_draught()),
        other => Err(eyre!("Unrecognised potion type {}", other)),
    });

    let mut player = config
        .iter()
        .find(|(name, _)| *name == args.job_name)
        .expect("expected a job")
        .1;

    if let Some(food) = food {
        // TODO: we'd like to properly validate these args,
        // but not sure how to nicely handle Option<Result<_>>
        // where we only care about errors when the Option is Some
        player = apply_buff_hq(&player, food.unwrap());
    }
    if let Some(potion) = potion {
        player = apply_buff_hq(&player, potion.unwrap());
    }

    let random_generator = RandomGenerator::from_lengths(10, 30);
    let random_flip = RandomFlip::new();
    let random_remove = RandomRemove {};
    // TODO: this'd probably be nicer as a BinaryHeap or something
    // maybe a more custom struct with BinaryHeap + HashSet to eliminate
    // duplicates and track recent scores over time?
    // it's just a bit annoying since we'd have to manually implement
    // PartialEq/Eq/PartialOrd/Ord for Candidate to delegate to the score
    let mut best_per_generation: Vec<Candidate> = Vec::new();
    let mut candidates = (0..1000)
        .map(|_| score_steps(player, &recipe, random_generator.generate()))
        .collect_vec();

    let generations = args.generations.unwrap_or(1000);
    let mut rng = thread_rng();
    for g in 0..generations {
        if ctrlc_pressed.load(SeqCst) {
            break;
        }

        candidates.sort_by_key(|x| Reverse(x.score));

        let candidates_count = candidates.len();
        if g % 100 == 0 {
            println!("g{} | {} | {}", g, candidates[0].score, candidates_count);
        }

        best_per_generation.push(candidates[0].clone());

        // TODO: maybe detect if the score hasn't changed in some number
        // of generations, and throw away the current best cohort to reset
        // the simulation and try for another optimum

        // higher scoring candidates have lower indexes, so they should have
        // a lower chance of "dying" this generation
        candidates = candidates
            .into_iter()
            .enumerate()
            .filter(|(i, _c)| (*i < 10) || (rng.gen_range(0..candidates_count) < *i))
            .map(|(_, c)| c)
            .collect_vec();
        // make sure that we don't get unlucky and just allow a whole ton of candidates
        candidates.drain(500..);
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
            .actual_steps
            .iter()
            .map(|s| format!("/ac \"{}\"", s))
            .join("\n")
    );
    Ok(())
}

struct Args {
    log_stats: bool,
    job_name: String,
    recipe: String,
    food: Option<String>,
    potion: Option<String>,
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

    -f, --food          (optional, assumes HQ) one of:
                            tsai_tou
                            jhinga_biryani

    -p, --potion        (optional, assumes HQ) one of:
                            cunning_draught

    -g, --generations   (optional) number of generations to search through

    -l, --log-stats     (optional) (very verbose) print csv stats per generation

    -h, --help          (optional) show this message
    "
        );
        return Err(eyre!(""));
    }

    let args = Args {
        log_stats: pargs.contains(["-l", "--log-stats"]),
        job_name: pargs.value_from_str(["-j", "--job"])?,
        recipe: pargs.value_from_str(["-r", "--recipe"])?,
        generations: pargs.opt_value_from_str(["-g", "--generations"])?,
        food: pargs.opt_value_from_str(["-f", "--food"])?,
        potion: pargs.opt_value_from_str(["-p", "--potion"])?,
    };

    let remaining = pargs.finish();
    if !remaining.is_empty() {
        return Err(eyre!("Unrecognised arguments: {:?}", remaining));
    }

    Ok(args)
}
