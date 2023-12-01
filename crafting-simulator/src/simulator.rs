use std::collections::HashMap;

use crate::{actions::Actions, model::*};

pub struct Simulator;

impl Simulator {
    pub fn run_steps(
        initial_state: CraftingState,
        player: PlayerStats,
        recipe: Recipe,
        steps: &[&str],
    ) -> CraftingState {
        let actions = Self::make_action_lookup();
        let steps = steps.iter().map(|name| actions.get(name).unwrap());

        steps.fold(initial_state, |state, step| {
            let mut next_state = state;
            next_state.veneration_stacks = state.veneration_stacks.saturating_sub(1);
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
        m
    }
}
