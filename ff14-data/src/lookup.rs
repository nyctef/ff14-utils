use crate::{embedded_csv, model::*};
use color_eyre::Result;
use derive_more::Constructor;

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
    /// Load item data from embedded CSV files
    pub async fn from_embedded() -> Result<ItemLookup> {
        Ok(ItemLookup::new(embedded_csv::read_items().await?))
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

    pub fn name_containing<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a Item> + 'a {
        self.matching(move |i| i.name.contains(name))
    }
}

#[derive(Debug, Constructor)]
pub struct RecipeLookup {
    recipes: Vec<Recipe>,
}

impl RecipeLookup {
    /// Load recipe data from embedded CSV files
    pub async fn from_embedded() -> Result<RecipeLookup> {
        Ok(RecipeLookup::new(embedded_csv::read_recipes().await?))
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
    /// Load materia data from embedded CSV files
    pub async fn from_embedded() -> Result<MateriaLookup> {
        let materia = embedded_csv::read_materia().await?;
        Ok(MateriaLookup::new(materia))
    }

    pub fn iter(&self) -> impl Iterator<Item = &Materia> {
        self.materia.iter()
    }
}
