use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::db::project::{Database, Error, FromRow};

type Result<T> = std::result::Result<T, Error>;

/// Data object for book.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[cfg_attr(test, derive(PartialEq, Default))]
pub struct Stats {
    pub users: usize,
    pub criminals: usize,
}

impl FromRow for Stats {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Stats> {
        Ok(Stats {
            users: row.get("users")?,
            criminals: row.get("criminals")?,
        })
    }
}

pub fn fetch(db: &Database) -> Result<Stats> {
    const STATS: &str = "\
        select \
        (select count(*) from user) as users, \
        (select count(*) from user where criminal is 1) as criminals \
    ";
    Ok(db.con.query_row(STATS, [], Stats::from_row)?)
}
