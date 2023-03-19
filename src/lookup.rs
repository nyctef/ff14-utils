use derive_more::Constructor;

use crate::model::*;

#[derive(Debug, Constructor)]
pub struct ItemLookup {
    items: Vec<Item>,
}

impl ItemLookup {
    pub fn matching(&self, predicate: impl Fn(&&Item) -> bool) -> impl Iterator<Item = &Item> {
        self.items.iter().filter(predicate)
    }

    pub fn item_by_id(&self, id: ItemId) -> &Item {
        self.items.iter().find(|i| i.id == id).unwrap()
    }

    pub fn item_by_name(&self, name: &str) -> &Item {
        self.items.iter().find(|i| i.name == name).unwrap()
    }
}

#[derive(Debug, Constructor)]
pub struct RecipeLookup {
    recipes: Vec<Recipe>,
}

impl RecipeLookup {
    pub fn recipe_for_item(&self, id: ItemId) -> Option<&Recipe> {
        self.recipes.iter().find(|r| r.result.item_id == id)
    }
}
