use crate::{actions::Actions, model::*};
use itertools::Itertools;
use std::collections::HashMap;

pub struct Simulator;

impl Simulator {
    pub fn run_steps(player: PlayerStats, recipe: Recipe, steps: &[&str]) -> CraftingState {
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
        steps.iter().fold(initial_state, |state, step| {
            let mut next_state = state;
            next_state.veneration_stacks = state.veneration_stacks.saturating_sub(1);
            next_state.innovation_stacks = state.innovation_stacks.saturating_sub(1);
            next_state = step.apply(&next_state, &player, &recipe);
            next_state
        })
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
        m
    }
}
