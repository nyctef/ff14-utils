use crate::model::PlayerStats;
use color_eyre::{
    eyre::{eyre, Report},
    Result,
};
use itertools::Itertools;
use std::{fs, path::Path};
use toml::{map::Map, Table, Value};

fn unwrap_u16(map: &Map<String, Value>, property_name: &'static str) -> Result<u16> {
    Ok(map
        .get(property_name)
        .ok_or_else(|| eyre!("failed to get property {}", property_name))?
        .as_integer()
        .ok_or_else(|| eyre!("failed to parse property {} as int", property_name))? as u16)
}

fn parse_player_stats((name, j): (&String, &Map<String, Value>)) -> Result<(String, PlayerStats)> {
    Ok::<_, Report>((
        name.to_string(),
        PlayerStats::level_90(
            unwrap_u16(j, "craftsmanship")?,
            unwrap_u16(j, "control")?,
            unwrap_u16(j, "cp")?,
        ),
    ))
}

pub fn read_jobs_from_config(path: &Path) -> Result<Vec<(String, PlayerStats)>> {
    let file_contents =
        fs::read_to_string(path).map_err(|e| eyre!("Failed to read file {:?} {}", path, e))?;
    let data = file_contents.parse::<Table>()?;

    let as_table = data["jobs"]
        .as_table()
        .ok_or_else(|| eyre!("failed to parse toml jobs as array"))?;

    let jobs: Vec<_> = as_table
        .iter()
        .map(|(name, j)| {
            Ok::<_, Report>((
                name,
                j.as_table()
                    .ok_or_else(|| eyre!("failed to parse job entry as table"))?,
            ))
        })
        .map(|x| x.and_then(parse_player_stats))
        .try_collect()?;
    Ok(jobs)
}
