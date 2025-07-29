use color_eyre::eyre::Result;
use ff14_data::{
    lookup::{ItemLookup, RecipeLookup},
    model::*,
};
use ff14_utils::{recipe_calculation::process_recipe_item, universalis::get_market_data_lookup};
use itertools::Itertools;
use thousands::Separable;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let items_lookup = ItemLookup::from_datamining_csv().await?;
    let recipes_lookup = RecipeLookup::from_datamining_csv().await?;

    let items = items_lookup
        .matching(|i| i.ilvl == 740 && i.name.starts_with("Ceremonial"))
        .map(|i| {
            (
                &i.name,
                num_certs_for(i.equip_slot_category.unwrap()),
                recipes_lookup.recipe_for_item(i.id).unwrap(),
            )
        })
        .collect_vec();

    let all_ids = items
        .iter()
        .map(|(_, _, recipe)| recipe)
        .flat_map(|r| r.relevant_item_ids(&recipes_lookup))
        .collect_vec();
    let market_data = get_market_data_lookup(&all_ids).await?;

    let result_lines = items
        .iter()
        .map(|(_, certs, recipe)| {
            (
                certs,
                process_recipe_item(
                    0,
                    &recipe.result,
                    &items_lookup,
                    &market_data,
                    &recipes_lookup,
                    true,
                )
                .1,
            )
        })
        .map(|(certs, lines)| (certs, lines.into_iter().last().unwrap()))
        .sorted_by_key(|(_, line)| line.market_price);

    for (certs, line) in result_lines {
        if let Some(price) = line.market_price {
            println!(
                "{:<50}: {} or ~{} per cert",
                line.name_and_amount,
                price.separate_with_commas(),
                (price / line.amount / certs).separate_with_commas()
            );
        }
    }

    Ok(())
}

fn num_certs_for(slot: EquipSlotCategory) -> u32 {
    match slot {
        EquipSlotCategory::TwoHandWeapon => 17,
        EquipSlotCategory::MainHand => 10,
        EquipSlotCategory::OffHand => 7,

        EquipSlotCategory::Body => 17,
        EquipSlotCategory::Legs => 17,

        EquipSlotCategory::Head => 11,
        EquipSlotCategory::Gloves => 11,
        EquipSlotCategory::Feet => 11,

        EquipSlotCategory::Ears => 7,
        EquipSlotCategory::Neck => 7,
        EquipSlotCategory::Wrists => 7,
        EquipSlotCategory::Ring => 7,
        _ => panic!("EquipSlot {} not matched", slot),
    }
}
