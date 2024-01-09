use color_eyre::{eyre::eyre, Result};
use crafting_simulator::arg_utils::{food_from_arg_value, potion_from_arg_value, recipe_from_arg_value};
use crafting_simulator::model::CraftStatus;
use crafting_simulator::simulator::Simulator as sim;
use crafting_simulator::{buffs::apply_buff_hq, config};
use itertools::Itertools;
use std::{io::Read, path::Path};

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = parse_args()?;

    let config = config::read_jobs_from_config(Path::new("./jobs.toml"))?;

    let food = food_from_arg_value(args.food.as_deref())?;
    let potion = potion_from_arg_value(args.potion.as_deref())?;
    let recipe = recipe_from_arg_value(&args.recipe)?;

    // read list of crafting steps from stdin
    let mut steps = String::new();
    std::io::stdin().read_to_string(&mut steps)?;
    // TODO: can we remove the requirement for run_steps taking a 'static str to avoid this
    // .leak()?
    let steps = steps.leak().trim().lines().collect_vec();

    for (job, mut player) in config {
        // TODO: more deduping with crafting-simulator bin
        if let Some(food) = &food {
            player = apply_buff_hq(&player, food);
        }
        if let Some(potion) = &potion {
            player = apply_buff_hq(&player, potion);
        }
        println!("testing steps for {}", job);
        let report = sim::run_steps(player, &recipe, &steps);
        let quality_factor = report.final_state.quality as f64 / recipe.quality_target as f64;
        print!(
            "{}",
            if report.status != CraftStatus::Success {
                // red: craft failed
                "\x1b[31m"
            } else if quality_factor >= 1.0 {
                // cyan: craft succeded with no need for hq mats
                "\x1b[36m"
            } else if quality_factor >= 0.5 {
                // green: craft succeeded but needs hq mats
                "\x1b[32m"
            } else {
                // yellow: quality too low
                "\x1b[33m"
            }
        );
        println!(
            "status: {:?} progress: {} quality: {} ({:.2}%)",
            report.status, report.final_state.progress, report.final_state.quality, quality_factor * 100.0
        );
        // reset color
        print!("\x1b[0m");

        println!();
    }

    Ok(())
}

struct Args {
    recipe: String,
    food: Option<String>,
    potion: Option<String>,
}

fn parse_args() -> Result<Args> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!(
            r"
USAGE: check-recipe --recipe <recipe>

FLAGS:
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
    -h, --help          (optional) show this message
    "
        );
        return Err(eyre!(""));
    }

    let args = Args {
        recipe: pargs.value_from_str(["-r", "--recipe"])?,
        food: pargs.opt_value_from_str(["-f", "--food"])?,
        potion: pargs.opt_value_from_str(["-p", "--potion"])?,
    };

    let remaining = pargs.finish();
    if !remaining.is_empty() {
        return Err(eyre!("Unrecognised arguments: {:?}", remaining));
    }

    Ok(args)
}
