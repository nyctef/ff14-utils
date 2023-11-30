use crate::model::{CraftingState, CraftingStep, PlayerStats, Recipe};
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

        let total_increase = (progression_modified_by_level * self.potency) / 100;

        CraftingState {
            progress: state.progress + total_increase,
            durability: state.durability - self.durability_cost as u16,
            cp: state.cp - self.cp_cost as u16,
            ..*state
        }
    }
}

pub static BASIC_SYNTHESIS: BasicSynthesis = BasicSynthesis {
    potency: 120,
    cp_cost: 0,
    durability_cost: 10,
};
pub static CAREFUL_SYNTHESIS: BasicSynthesis = BasicSynthesis {
    potency: 180,
    cp_cost: 7,
    durability_cost: 10,
};
pub static PRUDENT_SYNTHESIS: BasicSynthesis = BasicSynthesis {
    potency: 180,
    cp_cost: 18,
    durability_cost: 5,
};

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
        let step = &BASIC_SYNTHESIS;

        let new_state = step.apply(&initial_state, &L90_PLAYER, &RLVL640_GEAR);

        assert_eq!(297, new_state.progress);
        assert_eq!(60, new_state.durability);
    }

    #[test]
    fn careful_synthesis_1() {
        let initial_state = CraftingState::initial(&L90_PLAYER, &RLVL640_GEAR);
        let step = &CAREFUL_SYNTHESIS;

        let new_state = step.apply(&initial_state, &L90_PLAYER, &RLVL640_GEAR);

        assert_eq!(446, new_state.progress);
        assert_eq!(60, new_state.durability);
        assert_eq!(500 - 7, new_state.cp);
    }
}
