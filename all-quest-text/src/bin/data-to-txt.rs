use color_eyre::eyre::Result;
use itertools::Itertools;
use std::ffi::OsStr;
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

            let tden = result["TextData_en"].as_object();
            if !tden.is_some() {
                // some empty quests have no text data
                continue;
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
        }
    }

    Ok(())
}

struct TextData {
    dialog: Vec<TextDataItem>,
    journal: Vec<TextDataItem>,
    system: Vec<TextDataItem>,
    todo: Vec<TextDataItem>,
}

struct TextDataItem {}

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
