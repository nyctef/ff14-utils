use color_eyre::{eyre::Context, Result};
use derive_more::{Constructor, Display};
use itertools::Itertools;
use std::ops::Mul;

macro_rules! id {
    ($a:ident) => {
        #[derive(
            Debug, Display, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, Constructor,
        )]
        pub struct $a(i32);

        impl TryFrom<&String> for $a {
            type Error = color_eyre::Report;

            fn try_from(value: &String) -> Result<Self, Self::Error> {
                let v: i32 = value
                    .parse()
                    .wrap_err_with(|| format!("Failed to parse {} as id", value))?;
                Ok($a(v))
            }
        }

        impl $a {
            #[allow(dead_code)]
            pub const ZERO: $a = $a(0);
        }
    };
}

id!(ItemId);

#[derive(Debug, PartialEq, Eq, Constructor)]
pub struct Item {
    pub id: ItemId,
    pub name: String,
    pub name_singular: String,
    pub name_plural: String,
    pub ilvl: u32,
}

id!(RecipeId);

#[derive(Debug, PartialEq, Eq, Constructor)]
pub struct Recipe {
    pub id: RecipeId,
    pub ingredients: Vec<RecipeItem>,
    pub result: RecipeItem,
}

impl Mul<u32> for &Recipe {
    type Output = Recipe;

    fn mul(self, rhs: u32) -> Self::Output {
        Recipe::new(
            self.id,
            self.ingredients.iter().map(|i| i * rhs).collect_vec(),
            &self.result * rhs,
        )
    }
}

#[derive(Debug, PartialEq, Eq, Constructor)]
pub struct RecipeItem {
    pub item_id: ItemId,
    pub amount: u32,
}

impl Mul<u32> for &RecipeItem {
    type Output = RecipeItem;

    fn mul(self, rhs: u32) -> Self::Output {
        RecipeItem::new(self.item_id, self.amount * rhs)
    }
}

id!(MateriaId);

#[derive(Debug, PartialEq, Eq, Constructor)]
pub struct Materia {
    pub materia_id: MateriaId,
    pub materia_levels: Vec<MateriaLevel>,
    // pub base_param: BaseParamId,
}

#[derive(Debug, PartialEq, Eq, Constructor)]
pub struct MateriaLevel {
    pub item_id: ItemId,
    pub level: u8,
    pub bonus_value: i16,
}
