use color_eyre::eyre::{eyre, Result};
use grep::{
    self,
    regex::RegexMatcher,
    searcher::{sinks::UTF8, SearcherBuilder},
};
use itertools::Itertools;
use std::{
    env,
    fs::{self, DirEntry, Metadata},
    path::PathBuf,
    time::SystemTime,
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli_args = env::args().into_iter().skip(1).collect::<Vec<_>>();
    let folder = match &cli_args[..] {
        [f] => f,
        _ => return Err(eyre!("Usage: quest-text-helper [path to ACT logs folder]")),
    };

    let folder_files: Result<Vec<_>> = fs::read_dir(folder)?
        .into_iter()
        .map(|f| -> Result<(PathBuf, SystemTime)> {
            let f = f?;
            Ok((f.path(), f.metadata()?.modified()?))
        })
        .collect();
    let mut folder_files = folder_files?;
    folder_files.sort_by_key(|f| f.1);
    let newest_file = folder_files.pop().unwrap().0;
    dbg!(&newest_file);

    let pattern = "\\|003D\\|";
    let matcher = RegexMatcher::new_line_matcher(pattern)?;
    let mut searcher = SearcherBuilder::new().build();

    let mut lines = Vec::new();
    searcher.search_path(
        matcher,
        newest_file,
        UTF8(|_lnum, line| {
            lines.push(line.to_string());
            // Ok means we accepted the line successfully + true means keep searching
            Ok(true)
        }),
    )?;

    dbg!(lines);

    Ok(())
}
