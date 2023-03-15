use std::error::Error;
use std::process;
use tokio_stream::StreamExt;
use tokio::fs::File;

async fn filter_by_region(region:&str, file_in:&str, file_out:&str) -> Result<(), Box<dyn Error>> {
    // Function reads CSV file that has column named "region" at second position (index = 1).
    // It writes to new file only rows with region equal to passed argument
    // and removes region column.
    let mut rdr = csv_async::AsyncReader::from_reader(
        File::open(file_in).await?
    );
    let mut wri = csv_async::AsyncWriter::from_writer(
        File::create(file_out).await?
    );
    wri.write_record(rdr
        .headers()
        .await?.into_iter()
        .filter(|h| *h != "region")
    ).await?;
    let mut records = rdr.records();
    while let Some(record) = records.next().await {
        let record = record?;
        match record.get(1) {
            Some(reg) if reg == region => 
                wri.write_record(record
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != 1)
                    .map(|(_, s)| s)
                ).await?,
            _ => {},
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = filter_by_region(
        "MA",
        "/tmp/all_regions.csv",
        "/tmp/MA_only.csv"
    ).await {
        eprintln!("error running filter_by_region: {}", err);
        process::exit(1);
    }
}