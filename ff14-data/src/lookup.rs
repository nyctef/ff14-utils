use crate::{csv, model::*};
use color_eyre::Result;
use derive_more::Constructor;
use std::path::PathBuf;

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
    // TODO: consider making a from_xivapi_json version of these methods
    pub async fn from_datamining_csv() -> Result<ItemLookup> {
        let csv_base = PathBuf::from("../ffxiv-datamining/csv");
        Ok(ItemLookup::new(csv::read_items(&csv_base).await?))
    }

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

    pub fn name_containing<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &Item> + 'a {
        self.matching(move |i| i.name.contains(name))
    }
}

#[derive(Debug, Constructor)]
pub struct RecipeLookup {
    recipes: Vec<Recipe>,
}

impl RecipeLookup {
    pub async fn from_datamining_csv() -> Result<RecipeLookup> {
        let csv_base = PathBuf::from("../ffxiv-datamining/csv");
        Ok(RecipeLookup::new(csv::read_recipes(&csv_base).await?))
    }

    pub fn recipe_for_item(&self, id: ItemId) -> Option<&Recipe> {
        self.recipes.iter().find(|r| r.result.item_id == id)
    }
}

#[derive(Debug, Constructor)]
pub struct MateriaLookup {
    materia: Vec<Materia>,
}

impl MateriaLookup {
    pub async fn from_datamining_csv() -> Result<MateriaLookup> {
        let csv_base = PathBuf::from("../ffxiv-datamining/csv");
        let materia = csv::read_materia(&csv_base).await?;
        Ok(MateriaLookup::new(materia))
    }

    pub fn iter(&self) -> impl Iterator<Item = &Materia> {
        self.materia.iter()
    }
}
