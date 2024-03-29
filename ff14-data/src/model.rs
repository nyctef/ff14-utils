use color_eyre::{eyre::Context, Result};
use derive_more::{Constructor, Display};
use itertools::Itertools;
use std::{iter, ops::Mul};

use crate::lookup::RecipeLookup;

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

        impl Into<i32> for $a {
            fn into(self) -> i32 {
                self.0
            }
        }
        impl Into<usize> for $a {
            fn into(self) -> usize {
                usize::try_from(self.0).expect("id into usize")
            }
        }
        impl Into<u16> for $a {
            fn into(self) -> u16 {
                u16::try_from(self.0).expect("id into u16")
            }
        }
        impl Into<$a> for i32 {
            fn into(self) -> $a {
                $a::new(self)
            }
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
    pub can_be_hq: bool,
}

id!(RecipeId);

#[derive(Debug, PartialEq, Eq, Constructor)]
pub struct Recipe {
    pub id: RecipeId,
    pub ingredients: Vec<RecipeItem>,
    pub result: RecipeItem,

    pub rlvl: RecipeLevel,
    pub difficulty: u16,
    pub durability: u16,
    pub quality_target: u16,
    // TODO: can these be determined from the rlvl, or are they sometimes recipe-specific?
    pub required_craftsmanship: u16,
    pub required_control: u16,
}

impl Mul<u32> for &Recipe {
    type Output = Recipe;

    fn mul(self, rhs: u32) -> Self::Output {
        Recipe {
            ingredients: self.ingredients.iter().map(|i| i * rhs).collect_vec(),
            result: &self.result * rhs,
            rlvl: self.rlvl.clone(),
            ..*self
        }
    }
}

impl Recipe {
    // TODO: is there a nice way to make the lifetimes work if we don't collect_vec() here?
    pub fn relevant_item_ids(&self, recipes: &RecipeLookup) -> impl Iterator<Item = ItemId> {
        iter::once(self.result.item_id)
            .chain(
                self.ingredients
                    .iter()
                    .flat_map(|ri| ri.relevant_item_ids(recipes)),
            )
            .collect_vec()
            .into_iter()
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

impl RecipeItem {
    // TODO: is there a nice way to make the lifetimes work if we don't collect_vec() here?
    pub fn relevant_item_ids(&self, recipes: &RecipeLookup) -> impl Iterator<Item = ItemId> + '_ {
        iter::once(self.item_id).chain(
            recipes
                .recipe_for_item(self.item_id)
                .iter()
                .flat_map(|r| r.relevant_item_ids(recipes))
                .collect_vec(),
        )
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

id!(RecipeLevelId);

#[derive(Debug, PartialEq, Eq, Constructor, Clone)]
pub struct RecipeLevel {
    pub rlvl: RecipeLevelId,
    pub progress_divider: u8,
    pub progress_modifier: u8,
    pub quality_divider: u8,
    pub quality_modifier: u8,
    // these base values can be modified by a corresponding _factor on individual recipes
    pub base_difficulty: u16,
    pub base_durability: u16,
    pub base_quality_target: u16,
    // a rough UI indicator of recipe difficulty
    pub stars: u8,
}

id!(BonusStatId);

#[derive(Debug, PartialEq, Eq, Constructor, Clone)]
pub struct FoodBonus {
    pub bonus_id: BonusStatId,
    // TODO: intern?
    pub name: String,
    pub max: u8,
    pub max_hq: u8,
    pub value: u8,
    pub value_hq: u8,
}

id!(FoodId);
#[derive(Debug, PartialEq, Eq, Constructor, Clone)]
pub struct Food {
    pub food_id: FoodId,
    // TODO: since there's only two possible values here, it'd be nice to intern these strings
    pub item_ui_category_name: String,
    pub name: String,
    pub bonuses: Vec<FoodBonus>,
}
