use crate::{actions::Actions, model::*};
use itertools::Itertools;
use std::collections::HashMap;

pub struct Simulator;

impl Simulator {
    pub fn run_steps(player: PlayerStats, recipe: Recipe, steps: &[&str]) -> CraftingReport {
        let initial_state = CraftingState::initial(&player, &recipe);
        let actions = Self::make_action_lookup();
        let steps: Vec<_> = steps
            .iter()
            .map(|name| {
                actions
                    .get(name)
                    .ok_or_else(|| format!("Unknown action: {}", name))
            })
            .try_collect()
            .unwrap();
        let final_state = steps.iter().fold(initial_state, |prev_state, step| {
            let mut next = prev_state;
            next.cp = next.cp.saturating_sub(step.cp_cost(&next) as i16);
            next.durability = next
                .durability
                .saturating_sub(step.durability_cost() as i16);

            // TODO warn or error if cp+durability are now negative and/or zero

            next = step.apply(&next, &player, &recipe);

            if next.manipulation_stacks > 0 && next.manipulation_delay == 0 {
                next.durability = i16::min(next.durability + 5, recipe.durability as i16);
            }

            next.veneration_stacks = next.veneration_stacks.saturating_sub(step.num_steps());
            next.innovation_stacks = next.innovation_stacks.saturating_sub(step.num_steps());
            next.muscle_memory_stacks = next.muscle_memory_stacks.saturating_sub(step.num_steps());
            next.great_strides_stacks = next.great_strides_stacks.saturating_sub(step.num_steps());
            next.manipulation_stacks = next.manipulation_stacks.saturating_sub(step.num_steps());
            next.manipulation_delay = next.manipulation_delay.saturating_sub(step.num_steps());
            next.steps += step.num_steps();
            next
        });

        CraftingReport {
            final_state,
            issues: Vec::new(),
        }
    }

    fn make_action_lookup() -> HashMap<&'static str, Box<dyn CraftingStep>> {
        let mut m: HashMap<&str, Box<dyn CraftingStep>> = HashMap::new();
        m.insert("Basic Synthesis", Box::new(Actions::basic_synthesis()));
        m.insert("Careful Synthesis", Box::new(Actions::careful_synthesis()));
        m.insert("Prudent Synthesis", Box::new(Actions::prudent_synthesis()));
        m.insert("Groundwork", Box::new(Actions::groundwork()));
        m.insert("Veneration", Box::new(Actions::veneration()));
        m.insert("Basic Touch", Box::new(Actions::basic_touch()));
        m.insert("Prudent Touch", Box::new(Actions::prudent_touch()));
        m.insert("Preparatory Touch", Box::new(Actions::preparatory_touch()));
        m.insert("Innovation", Box::new(Actions::innovation()));
        m.insert("Great Strides", Box::new(Actions::great_strides()));
        m.insert("Byregot's Blessing", Box::new(Actions::byregots_blessing()));
        m.insert("Observe", Box::new(Actions::observe()));
        m.insert("Focused Synthesis", Box::new(Actions::focused_synthesis()));
        m.insert("Focused Touch", Box::new(Actions::focused_touch()));
        m.insert("Standard Touch", Box::new(Actions::standard_touch()));
        m.insert("Advanced Touch", Box::new(Actions::advanced_touch()));
        m.insert("Muscle Memory", Box::new(Actions::muscle_memory()));
        m.insert("Manipulation", Box::new(Actions::manipulation()));
        m
    }
}
