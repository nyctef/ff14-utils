use color_eyre::{eyre::Context, Result};
use derive_more::Constructor;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ItemId(i32);

impl TryFrom<&String> for ItemId {
    type Error = color_eyre::Report;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let v: i32 = value
            .parse()
            .wrap_err_with(|| format!("Failed to parse {} as ItemId", value))?;
        Ok(ItemId(v))
    }
}

#[derive(Debug, PartialEq, Eq, Constructor)]
pub struct Item {
    pub id: ItemId,
    pub name: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct RecipeId(i32);

// TODO: macro for this impl? (both &str and &String would probably be useful)
impl TryFrom<&String> for RecipeId {
    type Error = color_eyre::Report;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let v: i32 = value
            .parse()
            .wrap_err_with(|| format!("Failed to parse {} as RecipeId", value))?;
        Ok(RecipeId(v))
    }
}

#[derive(Debug, PartialEq, Eq, Constructor)]
pub struct Recipe {
    pub id: RecipeId,
    pub ingredients: Vec<RecipeItem>,
    pub result: RecipeItem,
}

#[derive(Debug, PartialEq, Eq, Constructor)]
pub struct RecipeItem {
    pub item_id: ItemId,
    pub amount: u8,
}
