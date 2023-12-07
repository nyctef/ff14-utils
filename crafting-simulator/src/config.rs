use crate::model::PlayerStats;
use color_eyre::{
    eyre::{eyre, Report},
    Result,
};
use itertools::Itertools;
use std::{fs, path::Path};
use toml::Table;

pub fn read_jobs_from_config(path: &Path) -> Result<Vec<PlayerStats>> {
    let file_contents = fs::read_to_string(path)?;
    let data = file_contents.parse::<Table>()?;
    let jobs = data["jobs"]
        .as_array()
        .ok_or_else(|| eyre!("failed to parse toml jobs as array"))
        .map(|js| {
            js.iter()
                .flat_map(|j| {
                    j.as_table()
                        .ok_or_else(|| eyre!("failed to parse job entry as table"))
                })
                .flat_map(|j| {
                    Ok::<PlayerStats, Report>(PlayerStats::level_90(
                        j["craftsmanship"]
                            .as_integer()
                            .ok_or_else(|| eyre!("failed to parse job csms as int"))?
                            as u16,
                        j["control"]
                            .as_integer()
                            .ok_or_else(|| eyre!("failed to parse job control as int"))?
                            as u16,
                        j["cp"]
                            .as_integer()
                            .ok_or_else(|| eyre!("failed to parse job cp as int"))?
                            as u16,
                    ))
                })
        })?
        .collect_vec();
    Ok(jobs)
}
