use color_eyre::eyre::Result;
use std::time::Duration;
use tokio::{fs, time};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // leave main as soon as possible since rust-analyzer
    // doesn't like proc macros
    run().await
}

async fn run() -> Result<()> {
    let client = reqwest::Client::new();

    let mut page = 1;
    loop {
        println!("requesting page: {}", page);

        let res = client
            .get("https://xivapi.com/Quest")
            .query(&vec![
                ("columns", "Url,Name_en,Name_ja,TextData_en,TextData_ja"),
                ("page", &page.to_string()),
                ("limit", "500"),
            ])
            .send()
            .await?;
        let body = res.text().await?;

        // save the page to a cache
        fs::create_dir_all("data").await?;
        fs::write(format!("data/{}.json", page), &body).await?;

        // deserialize the page to find the next page number
        let json: serde_json::Value = serde_json::from_str(&body)?;
        let next_page = json["Pagination"]["PageNext"].as_i64();
        if let Some(next_page) = next_page {
            page = next_page as i32;
            // sleep for 2 seconds to avoid hitting the api too hard
            time::sleep(Duration::from_secs(2)).await;
        } else {
            break;
        }
    }
    Ok(())
}
