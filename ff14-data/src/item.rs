use color_eyre::{eyre::Context, Result};
use derive_more::{Constructor, Display};
use itertools::Itertools;
use std::{iter, ops::Mul};

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
