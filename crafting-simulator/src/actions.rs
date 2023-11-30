use crate::model::{CraftingState, CraftingStep, PlayerStats, Recipe};
pub struct BasicSynthesis;

impl CraftingStep for BasicSynthesis {
    fn apply(&self, state: &CraftingState, stats: &PlayerStats, recipe: &Recipe) -> CraftingState {
        // based on some of the calculations in https://github.com/ffxiv-teamcraft/simulator/tree/dec02537f2ac0ec8c1bd61d85bc45f7b4b34e301/src/model/actions
        // ignoring recipe level difference for now
        let base_progression = (stats.craftsmanship * 10) / recipe.rlvl.progress_divider as u16 + 2;

        let progression_modified_by_level =
            (base_progression as f32 * recipe.rlvl.progress_modifier as f32 * 0.01f32) as u16;

        let potency = 120;

        // todo: muscle memory, veneration

        dbg!(base_progression, progression_modified_by_level, potency);

        let total_increase = (progression_modified_by_level * potency) / 100;

        let durability_cost = 10;

        CraftingState {
            progress: state.progress + total_increase,
            durability: state.durability - durability_cost,
            ..*state
        }
    }
}

pub static BASIC_SYNTHESIS: BasicSynthesis = BasicSynthesis;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{CraftingStep, Recipe, RLVL640};

    // basically just setting up scenarios on teamcraft and checking that these numbers match theirs

    #[test]
    fn basic_synthesis_1() {
        let stats = PlayerStats::level_90(4014, 3574, 500);
        let recipe = Recipe {
            rlvl: RLVL640,
            difficulty: 6600,
            durability: 70,
            quality_target: 14040,
        };
        let initial_state = CraftingState::initial(&stats, &recipe);
        let step = &BASIC_SYNTHESIS;
        let new_state = step.apply(&initial_state, &stats, &recipe);
        assert_eq!(297, new_state.progress);
        assert_eq!(60, new_state.durability)
    }
}
