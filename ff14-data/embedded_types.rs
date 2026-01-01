// Shared struct definitions for rkyv serialization/deserialization
// This file is included by both build.rs and embedded_data.rs

// TODO: can we move more of the processing into build.rs, so these types
// end up closer to the ones in model.rs?

use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, Clone)]
pub struct ItemRow {
    pub id: i32,
    pub name: String,
    pub singular: String,
    pub plural: String,
    pub ilvl: u32,
    pub can_be_hq: bool,
    pub equip_slot: u32,
}

#[derive(Archive, Deserialize, Serialize, Debug, Clone)]
pub struct RecipeRow {
    pub id: i32,
    pub number: u32,
    pub craft_type: u32,
    pub recipe_level_table: i32,
    pub item_result: i32,
    pub amount_result: u32,
    pub item_ingredients: Vec<i32>,
    pub amount_ingredients: Vec<u32>,
    pub difficulty_factor: u16,
    pub quality_factor: u16,
    pub durability_factor: u16,
    pub required_craftsmanship: u16,
    pub required_control: u16,
}

#[derive(Archive, Deserialize, Serialize, Debug, Clone)]
pub struct RecipeLevelRow {
    pub id: i32,
    pub progress_divider: u8,
    pub progress_modifier: u8,
    pub quality_divider: u8,
    pub quality_modifier: u8,
    pub difficulty: u16,
    pub durability: u16,
    pub quality: u16,
    pub stars: u8,
}

#[derive(Archive, Deserialize, Serialize, Debug, Clone)]
pub struct MateriaRow {
    pub id: i32,
    pub item_ids: Vec<i32>,
    pub values: Vec<i16>,
}

#[derive(Archive, Deserialize, Serialize, Debug)]
pub struct EmbeddedData {
    pub items: Vec<ItemRow>,
    pub recipes: Vec<RecipeRow>,
    pub recipe_levels: Vec<RecipeLevelRow>,
    pub materia: Vec<MateriaRow>,
}
