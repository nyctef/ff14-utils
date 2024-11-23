use crate::model::Recipe;
use color_eyre::eyre::Result;
use ironworks::{excel::Excel, ffxiv, ffxiv::Language, file::exl, Ironworks};

#[derive(Debug)]
pub struct RecipeLookup2 {
    recipes: Vec<Recipe>,
}

impl RecipeLookup2 {
    pub async fn from_game_data() -> Result<RecipeLookup2> {
        let ironworks = Ironworks::new();

        let excel = Excel::with()
            .language(Language::English)
            .build(&ironworks, ffxiv::Mapper::new());
        let recipes = excel.sheet("Recipe");
        dbg!(&recipes);

        panic!("TODO");
    }
}
