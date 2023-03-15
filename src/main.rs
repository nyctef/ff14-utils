use tokio_stream::StreamExt;
use tokio::fs::File;
use color_eyre::eyre::Result;

async fn print_ten_rows(file_in:&str) -> Result<Vec<String>> {
    // Function reads CSV file that has column named "region" at second position (index = 1).
    // It writes to new file only rows with region equal to passed argument
    // and removes region column.
    let mut reader = csv_async::AsyncReader::from_reader(
        File::open(file_in).await?
    );
    let mut records = reader.records().take(10);
    let mut result = vec![];
    while let Some(record) = records.next().await {
        let record = record?;
        result.push(record.iter().map(|x| x.to_owned()).collect::<Vec<_>>().join("|"));
    }
    Ok(result)
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let result = print_ten_rows(
        "../ffxiv-datamining/csv/Recipe.csv"
    ).await?;

    dbg!(result);

    Ok(())
}