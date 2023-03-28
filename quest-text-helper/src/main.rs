use axum::{extract::State, http::StatusCode, routing::get, Router, Server};
use color_eyre::eyre::{eyre, Context, Result};
use grep::{
    self,
    regex::RegexMatcher,
    searcher::{sinks::UTF8, SearcherBuilder},
};
use itertools::Itertools;
use std::{env, fs, path::PathBuf, time::SystemTime};

#[derive(Clone)]
struct ServerState {
    folder_path: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let server_state = ServerState {
        folder_path: get_folder_to_watch_from_args()?,
    };
    let router = Router::new()
        .route("/", get(root_get))
        .with_state(server_state);

    // todo: listen on arbitrary port, then open that page (using open crate)
    let server = Server::bind(&"0.0.0.0:51603".parse().unwrap()).serve(router.into_make_service());
    let addr = server.local_addr();
    println!("Listening on {addr}");

    server.await.wrap_err(eyre!("running server"))?;

    Ok(())
}

#[axum_macros::debug_handler]
async fn root_get(State(state): State<ServerState>) -> Result<String, (StatusCode, String)> {
    let lines = get_matching_lines(&state.folder_path)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{:#}", e)))?;
    // todo: a nicer way to take_last(10)?
    let lines = lines.iter().rev().take(10).rev().collect_vec();

    dbg!(lines);
    Ok("hello!".to_string())
}

fn get_folder_to_watch_from_args() -> Result<String> {
    let mut cli_args = env::args().skip(1).collect::<Vec<_>>();
    let err = Err(eyre!("Usage: quest-text-helper [path to ACT logs folder]"));

    return if cli_args.len() != 1 {
        err
    } else {
        Ok(cli_args.pop().unwrap())
    };
}

fn get_matching_lines(folder: &str) -> Result<Vec<String>, color_eyre::Report> {
    let folder_files: Result<Vec<_>> = fs::read_dir(folder)
        .wrap_err_with(|| format!("Failed to read files from folder: {folder}"))?
        .map(|f| -> Result<(PathBuf, SystemTime)> {
            let f = f?;
            Ok((f.path(), f.metadata()?.modified()?))
        })
        .collect();
    let mut folder_files = folder_files?;
    folder_files.sort_by_key(|f| f.1);
    let newest_file = folder_files
        .pop()
        .ok_or_else(|| eyre!("Folder {folder} appears to be empty"))?
        .0;
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
    Ok(lines)
}
