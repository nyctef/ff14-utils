use crate::{embedded_data, model::*};
use color_eyre::Result;
use derive_more::Constructor;

#[derive(Debug, Constructor)]
pub struct ItemLookup {
    items: Vec<Item>,
}

impl ItemLookup {
    pub fn from_embedded() -> Result<ItemLookup> {
        Ok(ItemLookup::new(embedded_data::read_items()?))
    }

    pub fn all(&self) -> impl Iterator<Item = &Item> {
        self.items.iter()
    }

    pub fn matching(&self, predicate: impl Fn(&&Item) -> bool) -> impl Iterator<Item = &Item> {
        self.items.iter().filter(predicate)
    }

    pub fn item_by_id(&self, id: ItemId) -> &Item {
        let index = embedded_data::get_item_index_by_id(id)
            .expect("Item not found");
        &self.items[index]
    }

    pub fn item_by_name(&self, name: &str) -> &Item {
        self.item_by_name_opt(name).unwrap()
    }

    pub fn item_by_name_opt(&self, name: &str) -> Option<&Item> {
        let index = embedded_data::get_item_index_by_name(name)?;
        self.items.get(index)
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
    pub fn from_embedded() -> Result<RecipeLookup> {
        Ok(RecipeLookup::new(embedded_data::read_recipes()?))
    }

    pub fn recipe_for_item(&self, id: ItemId) -> Option<&Recipe> {
        let index = embedded_data::get_recipe_index_by_result_item(id)?;
        self.recipes.get(index)
    }
}

#[derive(Debug, Constructor)]
pub struct MateriaLookup {
    materia: Vec<Materia>,
}

impl MateriaLookup {
    pub fn from_embedded() -> Result<MateriaLookup> {
        let materia = embedded_data::read_materia()?;
        Ok(MateriaLookup::new(materia))
    }

    pub fn iter(&self) -> impl Iterator<Item = &Materia> {
        self.materia.iter()
    }
}
