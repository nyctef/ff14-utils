use super::focused_actions::*;
use super::progress_actions::*;
use super::quality_actions::*;
use crate::model::CraftingStep;

pub struct Actions {}
impl Actions {
    pub fn basic_synthesis() -> impl CraftingStep {
        BasicSynthesis::new(120, 0, 10)
    }

    pub fn careful_synthesis() -> impl CraftingStep {
        BasicSynthesis::new(180, 7, 10)
    }

    pub fn prudent_synthesis() -> impl CraftingStep {
        BasicSynthesis::new(180, 18, 5)
    }

    pub fn groundwork() -> impl CraftingStep {
        BasicSynthesis::new(360, 18, 20)
    }

    pub fn veneration() -> impl CraftingStep {
        Veneration::new()
    }

    pub fn basic_touch() -> impl CraftingStep {
        ComboTouch::new(100, 18, 18, None, 1, 10)
    }

    pub fn standard_touch() -> impl CraftingStep {
        ComboTouch::new(125, 32, 18, Some(1), 2, 10)
    }

    pub fn advanced_touch() -> impl CraftingStep {
        ComboTouch::new(150, 46, 18, Some(2), 0, 10)
    }

    // TODO: standard and advanced touch (probably easiest to implement these as combo steps?)

    pub fn prudent_touch() -> impl CraftingStep {
        BasicTouch::new(100, 25, 5)
    }

    pub fn preparatory_touch() -> impl CraftingStep {
        BasicTouch::new(200, 40, 20)
    }

    pub fn innovation() -> impl CraftingStep {
        Innovation {}
    }

    pub fn great_strides() -> impl CraftingStep {
        GreatStrides {}
    }

    pub fn byregots_blessing() -> impl CraftingStep {
        ByregotsBlessing {}
    }

    pub fn observe() -> impl CraftingStep {
        Observe {}
    }

    pub fn focused_synthesis() -> impl CraftingStep {
        FocusedStep::new(Box::new(BasicSynthesis::new(200, 5, 10)))
    }

    pub fn focused_touch() -> impl CraftingStep {
        FocusedStep::new(Box::new(BasicTouch::new(150, 18, 10)))
    }
}
