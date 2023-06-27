use std::collections::HashSet;

use crate::db::project::{DBIter, Database, Error, FromRow, Result};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Data object for an workless.
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[cfg_attr(test, derive(PartialEq, Default))]
pub struct Workless {
    pub account: String,
    pub old_company: String,
    pub date_of_dismiss: NaiveDate,
    pub currently: bool,
    pub new_company: String,
    pub total_time: String,
}

impl Workless {
    pub fn is_valid(&self) -> bool {
        !self.account.trim().is_empty()
            && self.account.starts_with(char::is_alphabetic)
            && !self.old_company.trim().is_empty()
            && self.old_company.starts_with(char::is_alphanumeric)
    }
}

impl FromRow for Workless {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Workless> {
        Ok(Workless {
            account: row.get("account")?,
            old_company: row.get("old_company")?,
            date_of_dismiss: row.get("date_of_dismiss")?,
            currently: row.get("currently")?,
            new_company: row.get("new_company")?,
            total_time: row.get("total_time")?,
        })
    }
}

/// Returns the workless with the given `account`, company (old) and `date` (of dismiss).
pub fn fetch(db: &Database, account: &str, old_company: &str, date: NaiveDate) -> Result<Workless> {
    Ok(db.con.query_row(
        "select \
        account, \
        old_company, \
        date_of_dismiss, \
        currently, \
        new_company, \
        total_time \
        \
        from workless \
        where account=? and old_company=? and date_of_dismiss",
        rusqlite::params![account, old_company, date],
        Workless::from_row,
    )?)
}

/// Returns all dates (of dismiss) from the workless table without duplicates
pub fn all_dates(db: &Database) -> Result<Vec<String>> {
    let mut stmt = db.con.prepare(
        "select \
        date_of_dismiss \
        from workless \
        order by date_of_dismiss desc",
    )?;

    let mut rows = stmt.query([])?;
    let mut dates = Vec::new();
    let mut seen_dates = HashSet::new();

    while let Some(row) = rows.next()? {
        let date: String = row.get(0).unwrap();

        // Check if the date has already been seen
        if seen_dates.contains(&date) {
            continue; // Skip the duplicate date
        }

        dates.push(date.clone());
        seen_dates.insert(date);
    }

    Ok(dates)
}

/// Returns all roles from the workless table without duplicates
pub fn all_roles(db: &Database, date: &str, name: &str) -> Result<Vec<String>> {
    let mut stmt = db.con.prepare(
        "SELECT \
        DISTINCT user.role \
        FROM workless \
        INNER JOIN user ON workless.account = user.account \
        WHERE workless.date_of_dismiss like ?1 \
        and workless.account like '%'||?2||'%' \
        ORDER BY user.role ASC",
    )?;

    let mut rows = stmt.query(rusqlite::params![date, name.trim()])?;
    let mut roles = Vec::new();
    let mut seen_roles = HashSet::new();

    while let Some(row) = rows.next()? {
        let role: String = row.get(0).unwrap();

        // Check if the role has already been seen
        if seen_roles.contains(&role) {
            continue; // Skip the duplicate role
        }

        roles.push(role.clone());
        seen_roles.insert(role);
    }

    Ok(roles)
}

/// Searches with roles etc. from the workless table
pub fn search_role(
    db: &Database,
    name: &str,
    role: &str,
    date: &str,
    limit: usize,
) -> Result<Vec<Workless>> {
    let mut stmt = db.con.prepare(
        "SELECT workless.*
        FROM workless
        INNER JOIN user ON workless.account = user.account
        WHERE workless.account LIKE '%' || ?1 || '%'
        AND user.role LIKE ?2
        AND workless.date_of_dismiss LIKE ?3
        ORDER BY CASE
            WHEN workless.account LIKE ?1 || '%' THEN 0
            ELSE 1
        END ASC, workless.account ASC
        LIMIT ?4",
    )?;

    let rows = stmt.query(rusqlite::params![
        name.trim(),
        role.trim(),
        date.trim(),
        limit
    ])?;
    DBIter::new(rows).collect()
}

/// Parameters for the advanced search
///
/// Adding the '%' char allows every number of every character in this place
#[derive(Debug, Clone, Default)]
pub struct WorklessSearch<'a> {
    pub name: &'a str,
    pub old_company: &'a str,
    pub date: &'a str,
}

impl<'a> WorklessSearch<'a> {
    pub fn new(name: &'a str, old_company: &'a str, date: &'a str) -> WorklessSearch<'a> {
        Self {
            name,
            old_company,
            date,
        }
    }
}

/// Performes a simple workless search with the given `text`.
pub fn search(db: &Database, params: WorklessSearch, limit: usize) -> Result<Vec<Workless>> {
    let mut stmt = db.con.prepare(
        "select \
        account, \
        old_company, \
        date_of_dismiss, \
        currently, \
        new_company, \
        total_time \
        \
        from workless \
        where account like '%'||?1||'%' \
        and old_company like '%'||?2||'%' \
        and date_of_dismiss like ?3 \
        order by case \
            when account like ?1 || '%' then 0 \
            else 1 \
        end asc, account asc \
        limit ?4",
    )?;
    let rows = stmt.query(rusqlite::params![
        params.name.trim(),
        params.old_company.trim(),
        params.date.trim(),
        limit
    ])?;
    DBIter::new(rows).collect()
}

/// Adds a new date with presenters.
pub fn add(db: &Database, workless: &Workless) -> Result<()> {
    if !workless.is_valid() {
        return Err(Error::InvalidAbsence);
    }
    db.con.execute(
        "INSERT INTO workless VALUES (?, ?, ?, ?, ?, ?)",
        rusqlite::params![
            workless.account.trim(),
            workless.old_company.trim(),
            workless.date_of_dismiss,
            workless.currently,
            workless.new_company.trim(),
            workless.total_time.trim()
        ],
    )?;
    Ok(())
}

/// Updates the Absences.
pub fn update(
    db: &Database,
    previous_account: &str,
    previos_old_company: &str,
    previous_date: NaiveDate,
    workless: &Workless,
) -> Result<()> {
    let previous_account = previous_account.trim();
    let previos_old_company = previos_old_company.trim();
    if previous_account.is_empty() || previos_old_company.is_empty() || !workless.is_valid() {
        return Err(Error::InvalidAbsence);
    }

    let transaction = db.transaction()?;
    // update date
    transaction.execute(
        "update workless set account=?, old_company=?, date_of_dismiss=?, currently=?, new_company=?, total_time=? where account=? and old_company=? and date_of_dismiss=?",
        rusqlite::params![
            workless.account.trim(),
            workless.old_company.trim(),
            workless.date_of_dismiss,
            workless.currently,
            workless.new_company.trim(),
            workless.total_time.trim(),
            previous_account,
            previos_old_company,
            previous_date,
        ],
    )?;

    transaction.commit()?;
    Ok(())
}

/// Deletes the workless by account, old company and date.
pub fn delete(db: &Database, account: &str, old_company: &str, date: NaiveDate) -> Result<()> {
    let account = account.trim();
    let old_company = old_company.trim();
    if account.is_empty() || old_company.is_empty() {
        return Err(Error::InvalidAbsence);
    }
    let transaction = db.transaction()?;
    // remove date and presenters
    transaction.execute(
        "delete from workless where account=? and old_company=? and date_of_dismiss=?",
        rusqlite::params![account, old_company, date],
    )?;
    transaction.commit()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::db::project::{create, Database};
    use crate::db::workless::{self, Workless};
    #[test]
    fn add_update_remove_workless() {
        let db = Database::memory().unwrap();
        create(&db).unwrap();

        let workless = Workless {
            account: "foo.bar".into(),
            old_company: "bars".into(),
            date_of_dismiss: NaiveDate::from_ymd_opt(2023, 06, 01).unwrap(),
            currently: false,
            new_company: "fuzz".into(),
            total_time: "24h".into(),
        };
        workless::add(&db, &workless).unwrap();

        let result =
            workless::search(&db, workless::WorklessSearch::new("%", "%", "%"), 200).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], workless);

        workless::update(
            &db,
            &workless.account,
            &workless.old_company,
            NaiveDate::from_ymd_opt(2023, 06, 01).unwrap(),
            &Workless {
                total_time: "20h".into(),
                ..workless.clone()
            },
        )
        .unwrap();
        let result =
            workless::search(&db, workless::WorklessSearch::new("%", "%", "%"), 200).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].total_time, "20h".to_string());

        workless::delete(
            &db,
            &workless.account,
            &workless.old_company,
            NaiveDate::from_ymd_opt(2023, 06, 01).unwrap(),
        )
        .unwrap();
        let result =
            workless::search(&db, workless::WorklessSearch::new("%", "%", "%"), 200).unwrap();
        assert_eq!(result.len(), 0);
    }
}
