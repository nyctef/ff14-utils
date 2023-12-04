use derive_more::Constructor;

use crate::model::*;

#[derive(Debug, Constructor)]
pub struct ItemLookup {
    items: Vec<Item>,
}

// TODO: this is pretty inefficient (especially since we're reading static csv files +
// potentially building large hash tables on every run)
//   - investigate something like https://github.com/rust-phf/rust-phf
//   - https://doc.rust-lang.org/cargo/reference/build-scripts.html
//   - https://doc.rust-lang.org/cargo/reference/build-scripts.html#change-detection
//   - https://crates.io/crates/lazy_static

impl ItemLookup {
    pub fn all(&self) -> impl Iterator<Item = &Item> {
        self.items.iter()
    }

    pub fn matching(&self, predicate: impl Fn(&&Item) -> bool) -> impl Iterator<Item = &Item> {
        self.items.iter().filter(predicate)
    }

    pub fn item_by_id(&self, id: ItemId) -> &Item {
        self.items.iter().find(|i| i.id == id).unwrap()
    }

    pub fn item_by_name(&self, name: &str) -> &Item {
        self.item_by_name_opt(name).unwrap()
    }

    pub fn item_by_name_opt(&self, name: &str) -> Option<&Item> {
        self.items.iter().find(|i| i.name == name)
    }

    pub fn name_containing(&self, name: &str) -> impl Iterator<Item = &Item> {
        // TODO: is there a more correct way to do this without copying name?
        let name = name.to_owned();
        self.matching(move |i| i.name.contains(&name))
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
