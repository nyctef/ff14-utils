use crate::model::{CraftingState, CraftingStep, PlayerStats, Recipe};
use std::collections::HashMap;

pub struct BasicSynthesis {
    potency: u16,
    cp_cost: u8,
    durability_cost: u8,
}

impl CraftingStep for BasicSynthesis {
    fn apply(&self, state: &CraftingState, stats: &PlayerStats, recipe: &Recipe) -> CraftingState {
        // based on some of the calculations in https://github.com/ffxiv-teamcraft/simulator/tree/dec02537f2ac0ec8c1bd61d85bc45f7b4b34e301/src/model/actions
        // ignoring recipe level difference for now
        let base_progression = (stats.craftsmanship * 10) / recipe.rlvl.progress_divider as u16 + 2;

        let progression_modified_by_level =
            (base_progression as f32 * recipe.rlvl.progress_modifier as f32 * 0.01f32) as u16;

        // todo: muscle memory, veneration

        let mut potency = self.potency;
        if state.veneration_stacks > 0 {
            potency += potency / 2;
        }

        let total_increase = (progression_modified_by_level * potency) / 100;

        CraftingState {
            progress: state.progress + total_increase,
            durability: state.durability - self.durability_cost as u16,
            cp: state.cp - self.cp_cost as u16,
            ..*state
        }
    }
}

pub struct Veneration {}

impl CraftingStep for Veneration {
    fn apply(
        &self,
        state: &CraftingState,
        _stats: &PlayerStats,
        _recipe: &Recipe,
    ) -> CraftingState {
        CraftingState {
            cp: state.cp - 18,
            veneration_stacks: 4,
            ..*state
        }
    }
}

pub struct Actions {}
impl Actions {
    pub fn basic_synthesis() -> impl CraftingStep {
        BasicSynthesis {
            potency: 120,
            cp_cost: 0,
            durability_cost: 10,
        }
    }

    pub fn careful_synthesis() -> impl CraftingStep {
        BasicSynthesis {
            potency: 180,
            cp_cost: 7,
            durability_cost: 10,
        }
    }

    pub fn prudent_synthesis() -> impl CraftingStep {
        BasicSynthesis {
            potency: 180,
            cp_cost: 18,
            durability_cost: 5,
        }
    }

    pub fn groundwork() -> impl CraftingStep {
        BasicSynthesis {
            potency: 360,
            cp_cost: 18,
            durability_cost: 20,
        }
    }

    pub fn veneration() -> impl CraftingStep {
        Veneration {}
    }
}

pub fn make_action_lookup() -> HashMap<&'static str, Box<dyn CraftingStep>> {
    let mut m: HashMap<&str, Box<dyn CraftingStep>> = HashMap::new();
    m.insert("Basic Synthesis", Box::new(Actions::basic_synthesis()));
    m.insert("Careful Synthesis", Box::new(Actions::careful_synthesis()));
    m.insert("Prudent Synthesis", Box::new(Actions::prudent_synthesis()));
    m.insert("Groundwork", Box::new(Actions::groundwork()));
    m.insert("Veneration", Box::new(Actions::veneration()));
    m
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{CraftingStep, Recipe, RLVL640};

    // basically just setting up scenarios on teamcraft and checking that these numbers match theirs

    static RLVL640_GEAR: Recipe = Recipe {
        rlvl: RLVL640,
        difficulty: 6600,
        durability: 70,
        quality_target: 14040,
    };
    static L90_PLAYER: PlayerStats = PlayerStats {
        player_lvl: 560,
        craftsmanship: 4014,
        control: 3574,
        cp: 500,
    };

    #[test]
    fn basic_synthesis_1() {
        let initial_state = CraftingState::initial(&L90_PLAYER, &RLVL640_GEAR);
        let step = Actions::basic_synthesis();

        let new_state = step.apply(&initial_state, &L90_PLAYER, &RLVL640_GEAR);

        assert_eq!(297, new_state.progress);
        assert_eq!(60, new_state.durability);
    }

    #[test]
    fn careful_synthesis_1() {
        let initial_state = CraftingState::initial(&L90_PLAYER, &RLVL640_GEAR);
        let step = Actions::careful_synthesis();

        let new_state = step.apply(&initial_state, &L90_PLAYER, &RLVL640_GEAR);

        assert_eq!(446, new_state.progress);
        assert_eq!(60, new_state.durability);
        assert_eq!(500 - 7, new_state.cp);
    }

    #[test]
    fn veneration_increases_next_synthesis_step_by_50_percent() {
        let initial_state = CraftingState::initial(&L90_PLAYER, &RLVL640_GEAR);
        let actions = make_action_lookup();
        let steps = ["Veneration", "Basic Synthesis"]
            .iter()
            .map(|name| actions.get(name).unwrap());

        let final_state = steps.fold(initial_state, |state, step| {
            step.apply(&state, &L90_PLAYER, &RLVL640_GEAR)
        });

        assert_eq!(446, final_state.progress);
        assert_eq!(500 - 18, final_state.cp);
    }
}
