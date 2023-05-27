use std::collections::HashSet;

use crate::db::project::{DBIter, Database, Error, FromRow, Result};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Data object for an absence.
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[cfg_attr(test, derive(PartialEq, Default))]
pub struct Absence {
    pub account: String,
    pub date: NaiveDate,
    pub time: Option<String>,
}

impl Absence {
    pub fn is_valid(&self) -> bool {
        !self.account.trim().is_empty()
    }
}

impl FromRow for Absence {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Absence> {
        Ok(Absence {
            account: row.get("account")?,
            date: row.get("date")?,
            time: row.get("time")?,
        })
    }
}

/// Returns the absence with the given `account` and `date`.
pub fn fetch(db: &Database, account: &str, date: NaiveDate) -> Result<Absence> {
    Ok(db.con.query_row(
        "select \
        account, \
        date, \
        time \
        from absence \
        where account=? and date=?",
        rusqlite::params![account, date],
        Absence::from_row,
    )?)
}

/// Returns all dates from the absence table without duplicates
pub fn all_dates(db: &Database) -> Result<Vec<String>> {
    let mut stmt = db.con.prepare(
        "select \
        date \
        from absence \
        order by date",
    )?;

    let mut rows = stmt.query([])?;
    let mut dates = Vec::new();
    let mut seen_dates = HashSet::new();

    while let Some(row) = rows.next()? {
        let role: String = row.get(0).unwrap();

        // Check if the role has already been seen
        if seen_dates.contains(&role) {
            continue; // Skip the duplicate role
        }

        dates.push(role.clone());
        seen_dates.insert(role);
    }

    Ok(dates)
}


/// Performes a simple absence search with the given `text`.
pub fn search(db: &Database, text: &str) -> Result<Vec<Absence>> {
    let mut stmt = db.con.prepare(
        "select \
        account, \
        date, \
        time \
        \
        from absence \
        where account like '%'||?1||'%' \
        or date like '%'||?1||'%' \
        or time like '%'||?1||'%' \
        order by account",
    )?;
    let rows = stmt.query([text.trim()])?;
    DBIter::new(rows).collect()
}

/// Adds a new date with presenters.
pub fn add(db: &Database, absence: &Absence) -> Result<()> {
    if !absence.is_valid() {
        return Err(Error::InvalidUser);
    }
    db.con.execute(
        "INSERT INTO absence VALUES (?, ?, ?)",
        rusqlite::params![absence.account.trim(), absence.date, absence.time,],
    )?;
    Ok(())
}

/// Updates the Absences.
pub fn update(
    db: &Database,
    previous_account: &str,
    previous_date: NaiveDate,
    absence: &Absence,
) -> Result<()> {
    let previous_account = previous_account.trim();
    if previous_account.is_empty() || !absence.is_valid() {
        return Err(Error::InvalidUser);
    }

    let transaction = db.transaction()?;
    // update date
    transaction.execute(
        "update absence set account=?, date=?, time=? where account=? and date=?",
        rusqlite::params![
            absence.account.trim(),
            absence.date,
            absence.time,
            previous_account,
            previous_date,
        ],
    )?;

    transaction.commit()?;
    Ok(())
}

/// Deletes the absence by account and date.
pub fn delete(db: &Database, account: &str, date: NaiveDate) -> Result<()> {
    let account = account.trim();
    if account.is_empty() {
        return Err(Error::InvalidUser);
    }
    let transaction = db.transaction()?;
    // remove date and presenters
    transaction.execute(
        "delete from absence where account=? and date=?",
        rusqlite::params![account, date],
    )?;
    transaction.commit()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::db::absence::{self, Absence};
    use crate::db::project::{create, Database};
    #[test]
    fn add_update_remove_absences() {
        let db = Database::memory().unwrap();
        create(&db).unwrap();

        let absence = Absence {
            account: "foo.bar".into(),
            date: NaiveDate::from_ymd_opt(2023, 4, 26).unwrap(),
            time: None,
        };
        absence::add(&db, &absence).unwrap();

        let result = absence::search(&db, "").unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], absence);

        absence::update(
            &db,
            &absence.account,
            NaiveDate::from_ymd_opt(2023, 4, 26).unwrap(),
            &Absence {
                time: Some("5 Mins Late".into()),
                ..absence.clone()
            },
        )
        .unwrap();
        let result = absence::search(&db, "").unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].time, Some("5 Mins Late".into()));

        absence::delete(
            &db,
            &absence.account,
            NaiveDate::from_ymd_opt(2023, 4, 26).unwrap(),
        )
        .unwrap();
        let result = absence::search(&db, "").unwrap();
        assert_eq!(result.len(), 0);
    }
}
