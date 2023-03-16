use color_eyre::{eyre::Context, Result};
use derive_more::{Constructor, From};

#[derive(Debug, PartialEq, Eq)]
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
    id: ItemId,
    name: String,
}

#[derive(Debug, PartialEq, Eq, From)]
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
    id: RecipeId,
    items: Vec<RecipeItem>,
}

#[derive(Debug, PartialEq, Eq, Constructor)]
pub struct RecipeItem {
    item_id: ItemId,
    amount: u8,
}
