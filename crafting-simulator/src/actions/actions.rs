use std::collections::HashMap;

use super::focused_actions::*;
use super::progress_actions::*;
use super::quality_actions::*;
use super::repair_actions::*;
use crate::model::CraftingStep;

pub struct Actions {}
impl Actions {
    fn basic_synthesis() -> impl CraftingStep {
        BasicSynthesis::new(120, 0, 10, false)
    }

    fn careful_synthesis() -> impl CraftingStep {
        BasicSynthesis::new(180, 7, 10, false)
    }

    fn prudent_synthesis() -> impl CraftingStep {
        BasicSynthesis::new(180, 18, 5, true)
    }

    fn groundwork() -> impl CraftingStep {
        Groundwork {}
    }

    fn veneration() -> impl CraftingStep {
        Veneration::new()
    }

    fn basic_touch() -> impl CraftingStep {
        ComboTouch::new(100, 18, 18, None, 1, 10)
    }

    fn standard_touch() -> impl CraftingStep {
        ComboTouch::new(125, 32, 18, Some(1), 2, 10)
    }

    fn advanced_touch() -> impl CraftingStep {
        ComboTouch::new(150, 46, 18, Some(2), 0, 10)
    }

    fn prudent_touch() -> impl CraftingStep {
        BasicTouch::new(100, 25, 5, 1, true)
    }

    fn preparatory_touch() -> impl CraftingStep {
        BasicTouch::new(200, 40, 20, 2, false)
    }

    fn innovation() -> impl CraftingStep {
        Innovation {}
    }

    fn great_strides() -> impl CraftingStep {
        GreatStrides {}
    }

    fn byregots_blessing() -> impl CraftingStep {
        ByregotsBlessing {}
    }

    fn observe() -> impl CraftingStep {
        Observe {}
    }

    fn focused_synthesis() -> impl CraftingStep {
        FocusedStep::new(Box::new(BasicSynthesis::new(200, 5, 10, false)))
    }

    fn focused_touch() -> impl CraftingStep {
        FocusedStep::new(Box::new(BasicTouch::new(150, 18, 10, 1, false)))
    }

    fn muscle_memory() -> impl CraftingStep {
        MuscleMemory {}
    }

    fn manipulation() -> impl CraftingStep {
        Manipulation {}
    }

    fn waste_not() -> impl CraftingStep {
        WasteNot::new(4, 56)
    }

    fn waste_not_2() -> impl CraftingStep {
        WasteNot::new(8, 98)
    }

    fn masters_mend() -> impl CraftingStep {
        MastersMend {}
    }

    fn final_appraisal() -> impl CraftingStep {
        FinalAppraisal {}
    }

    fn tricks_of_the_trade() -> impl CraftingStep {
        TricksOfTheTrade {}
    }

    pub fn make_action_lookup() -> HashMap<&'static str, Box<dyn CraftingStep>> {
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
        m.insert("Waste Not", Box::new(Actions::waste_not()));
        m.insert("Waste Not II", Box::new(Actions::waste_not_2()));
        m.insert("Master's Mend", Box::new(Actions::masters_mend()));
        m.insert("Final Appraisal", Box::new(Actions::final_appraisal()));
        m.insert("Tricks of the Trade", Box::new(Actions::tricks_of_the_trade()));
        m
    }
}
