use derive_more::{Constructor, From};

#[derive(Debug, PartialEq, Eq, From)]
pub struct ItemId(u32);

#[derive(Debug, PartialEq, Eq)]
pub struct Item {
    id: ItemId,
    name: String,
}

#[derive(Debug, PartialEq, Eq, From)]
pub struct RecipeId(u32);

#[derive(Debug, PartialEq, Eq, Constructor)]
pub struct Recipe {
    id: RecipeId,
    items: Vec<RecipeItem>,
}

#[derive(Debug, PartialEq, Eq, Constructor)]
pub struct RecipeItem {
    item_id: ItemId,
    amount: u8,
}
