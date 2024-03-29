use axum::{extract::State, http::StatusCode, response::Html, routing::get, Json, Router, Server};
use color_eyre::eyre::{eyre, Context, Result};
use grep::{
    self,
    regex::RegexMatcher,
    searcher::{sinks::UTF8, SearcherBuilder},
};
use itertools::Itertools;
use serde::Serialize;
use std::{env, fs, path::PathBuf, time::SystemTime};

#[derive(Clone)]
struct ServerState {
    folder_path: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let folder_to_watch = get_folder_to_watch_from_args()?;
    run_server(folder_to_watch).await?;

    Ok(())
}

async fn run_server(folder_to_watch: String) -> Result<()> {
    let server_state = ServerState {
        folder_path: folder_to_watch,
    };
    let router = Router::new()
        .route("/", get(root_get))
        .route("/api/recent_lines", get(api_recent_lines_get))
        .with_state(server_state);

    // todo: listen on arbitrary port, then open that page (using open crate)
    let server = Server::bind(&"0.0.0.0:51603".parse().unwrap()).serve(router.into_make_service());
    let addr = server.local_addr();
    println!("Listening on {addr}");

    server.await.wrap_err(eyre!("running server"))?;
    Ok(())
}

async fn root_get() -> Html<String> {
    Html(include_str!("./index.html").to_string())
}

#[axum_macros::debug_handler]
async fn api_recent_lines_get(
    State(state): State<ServerState>,
) -> Result<Json<Vec<ChatlogLine>>, (StatusCode, String)> {
    let lines = get_matching_lines(&state.folder_path)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{:#}", e)))?;
    // todo: a nicer way to take_last(20)?
    let lines = lines.into_iter().rev().take(20).rev().collect_vec();

    Ok(Json(lines))
}

fn get_folder_to_watch_from_args() -> Result<String> {
    let mut cli_args = env::args().skip(1).collect::<Vec<_>>();
    let err = Err(eyre!("Usage: quest-text-helper [path to ACT logs folder]"));

    if cli_args.len() != 1 {
        err
    } else {
        Ok(cli_args.pop().unwrap())
    }
}

#[derive(Serialize)]
struct ChatlogLine {
    timestamp: String,
    speaker: String,
    text: String,
}

fn get_matching_lines(folder: &str) -> Result<Vec<ChatlogLine>, color_eyre::Report> {
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
    // dbg!(&newest_file);
    let pattern = "\\|(003D|0BB9|0039)\\|";
    let matcher = RegexMatcher::new_line_matcher(pattern)?;
    let mut searcher = SearcherBuilder::new().build();
    let mut lines = Vec::new();
    searcher.search_path(
        matcher,
        newest_file,
        UTF8(|_lnum, line| {
            // TODO: can we do this as part of the regex? Would feel cleaner
            let parts = line.split('|').collect_vec();
            lines.push(ChatlogLine {
                timestamp: parts[1].to_string(),
                speaker: parts[3].to_string(),
                text: parts[4].to_string(),
            });
            // Ok means we accepted the line successfully + true means keep searching
            Ok(true)
        }),
    )?;
    Ok(lines)
}
