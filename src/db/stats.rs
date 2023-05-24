use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::db::project::{Database, FromRow, Result};


const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const PKG_LICENSE: &str = env!("CARGO_PKG_LICENSE");

/// Data object for Stats.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[cfg_attr(test, derive(PartialEq, Default))]
pub struct Stats {
    pub name: &'static str,
    pub version: &'static str,
    pub repo: &'static str,
    pub developer: &'static str,
    pub description: &'static str,
    pub license: &'static str,
    pub users: usize,
}

impl FromRow for Stats {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Stats> {
        Ok(Stats {
            name: PKG_NAME,
            version: PKG_VERSION,
            repo: PKG_REPOSITORY,
            developer: PKG_AUTHORS,
            description: PKG_DESCRIPTION,
            license: PKG_LICENSE,
            users: row.get("users")?,
        })
    }
}

pub fn fetch(db: &Database) -> Result<Stats> {
    const STATS: &str = "\
        select \
        (select count(*) from user) as users, \
    ";
    Ok(db.con.query_row(STATS, [], Stats::from_row)?)
}
