use color_eyre::eyre::Result;
use itertools::Itertools;
use serde_json::{
    map::Entry::Occupied,
    Value::{Array as JArray, Null as JNull, String as JString},
};
use std::{ffi::OsStr, ops::ControlFlow};
use tokio::fs;

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}

async fn run() -> Result<()> {
    let mut files = fs::read_dir("data").await?;

    while let Some(file) = files.next_entry().await? {
        eprintln!("file: {:?}", file);
        let path = file.path();
        if path.extension().and_then(OsStr::to_str) != Some("json") {
            eprintln!("skipping non-json file: {:?}", path);
            continue;
        }
        let body = fs::read_to_string(path).await?;
        let json: serde_json::Value = serde_json::from_str(&body)?;
        let results = json["Results"].as_array().unwrap();
        for result in results {
            let url = result["Url"].as_str().unwrap();
            let name_en = result["Name_en"].as_str().unwrap();
            let name_ja = result["Name_ja"].as_str().unwrap();
            println!("quest {}", url);
            println!("  name_en: {}", replace_ff14_icons(name_en));
            println!("  name_ja: {}", replace_ff14_icons(name_ja));

            let parsed_en = parse_textdata(result["TextData_en"].as_object(), 1);
            let parsed_ja = parse_textdata(result["TextData_ja"].as_object(), 1);

            if parsed_en.is_none() && parsed_ja.is_none() {
                continue;
            }
            let parsed_en = parsed_en.unwrap();
            let parsed_ja = parsed_ja.unwrap();
            println!("  dialog:");
            // fails: assert!(parsed_en.dialog.len() == parsed_ja.dialog.len());
            // TODO: is there some reasonable way we can pair up individual dialog lines?
            for ja_line in &parsed_ja.dialog {
                println!("    {}: {}", ja_line.npc, replace_ff14_icons(&ja_line.text));
            }
            for en_line in &parsed_en.dialog {
                println!("    {}: {}", en_line.npc, replace_ff14_icons(&en_line.text));
            }
        }
    }

    Ok(())
}

fn parse_textdata(
    tden: Option<&serde_json::Map<String, serde_json::Value>>,
    npc_index: usize,
) -> Option<TextData> {
    if !tden.is_some() {
        // some empty quests have no text data
        return None;
    }
    let tden = tden.unwrap();
    assert!(
        tden.keys().all(|k| k == "Dialogue"
            || k == "Journal"
            || k == "System"
            || k == "ToDo"
            || k == "Todo" // yes, really
            || k == "QA_Question"
            || k == "QA_Answer"
            || k == "Pop"
            || k == "Access"
            || k == "Instance"
            || k == "BattleTalk"),
        "unexpected text data key: {:?}",
        tden.keys().collect_vec()
    );
    if !tden.contains_key("Dialogue") {
        // Looking at you, "Leves of Ishgard"...
        return None;
    }
    // for now we'll just look at regular people talking
    let dialogue = tden["Dialogue"].as_array().unwrap();
    let dialogue = dialogue
        .iter()
        .map(|v| {
            let key = v["Key"].as_str().unwrap().to_string();
            let npc = match v["Npc"] {
                JArray(ref a) => a[npc_index]
                    .as_str()
                    .unwrap_or("<null>")
                    .to_string(),
                JString(ref s) => s.to_string(),
                JNull => "<null>".to_string(),
                _ => panic!("unexpected Npc value: {:?}", v["Npc"]),
            };
            let text = v["Text"].as_str().unwrap().to_string();
            let order = v["Order"].as_u64().unwrap() as u32;
            TextDataItem {
                key,
                npc,
                text,
                order,
            }
        })
        .collect_vec();
    Some(TextData { dialog: dialogue })
}

struct TextData {
    dialog: Vec<TextDataItem>,
    // journal: Vec<TextDataItem>,
    // system: Vec<TextDataItem>,
    // todo: Vec<TextDataItem>,
}

struct TextDataItem {
    key: String,
    // TODO: this is going to be duplicated a lot, but deduping makes for annoying lifetimes
    npc: String,
    text: String,
    order: u32,
}

fn replace_ff14_icons(text: &str) -> String {
    // ff14 uses various unicode code points in the private use area for icons
    //
    // https://thewakingsands.github.io/ffxiv-axis-font-icons/ seems to have the
    // full list
    let quest_sync_icon = "\u{e0be}";
    let unicode_down_arrow_in_circle = "\u{2b8b}";

    // TODO: what if we want to do lots of these replacements?
    // calling .replace() multiple times is likely to be suboptimal
    // (although it's probably not too bad on the short strings we
    // care about here)
    text.replace(quest_sync_icon, unicode_down_arrow_in_circle)
}
