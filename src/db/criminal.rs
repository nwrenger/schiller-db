use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::db::project::{DBIter, Database, Error, FromRow, Result};

/// Data object for a criminal.
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[cfg_attr(test, derive(PartialEq, Default))]
pub struct Criminal {
    pub account: String,
    pub kind: String,
    pub data: String,
}

impl Criminal {
    pub fn is_valid(&self) -> bool {
        !self.account.trim().is_empty()
            && !self.account.starts_with("#")
            && self.account != "."
            && !self.kind.trim().is_empty()
            && !self.kind.starts_with("#")
            && self.kind != "."
    }
}

impl FromRow for Criminal {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Criminal> {
        Ok(Criminal {
            account: row.get("account")?,
            kind: row.get("kind")?,
            data: row.get("data")?,
        })
    }
}

/// Returns the criminal with the given `account`.
pub fn fetch(db: &Database, account: &str, kind: &str) -> Result<Criminal> {
    Ok(db.con.query_row(
        "select \
        account, \
        kind, \
        data \
        from criminal \
        where account=?",
        rusqlite::params![account, kind],
        Criminal::from_row,
    )?)
}

/// Returns all kinds from the criminal table without duplicates
pub fn all_kinds(db: &Database) -> Result<Vec<String>> {
    let mut stmt = db.con.prepare(
        "select \
        kind \
        from criminal \
        order by kind",
    )?;

    let mut rows = stmt.query([])?;
    let mut kinds = Vec::new();
    let mut seen_kinds = HashSet::new();

    while let Some(row) = rows.next()? {
        let kind: String = row.get(0).unwrap();

        // Check if the kind has already been seen
        if seen_kinds.contains(&kind) {
            continue; // Skip the duplicate kind
        }

        kinds.push(kind.clone());
        seen_kinds.insert(kind);
    }

    Ok(kinds)
}

/// Performes a simple criminal search with the given `text`.
pub fn search(db: &Database, text: &str) -> Result<Vec<Criminal>> {
    let mut stmt = db.con.prepare(
        "select \
        account, \
        kind, \
        data \
        \
        from criminal \
        where account like '%'||?1||'%' \
        or kind like '%'||?1||'%' \
        or data like '%'||?1||'%' \
        order by account",
    )?;
    let rows = stmt.query([text.trim()])?;
    DBIter::new(rows).collect()
}

/// Adds a new criminal.
pub fn add(db: &Database, criminal: &Criminal) -> Result<()> {
    if !criminal.is_valid() {
        return Err(Error::UnprocessableEntity);
    }
    db.con.execute(
        "INSERT INTO criminal VALUES (?, ?, ?)",
        rusqlite::params![
            criminal.account.trim(),
            criminal.kind.trim(),
            criminal.data.trim()
        ],
    )?;
    Ok(())
}

/// Updates the criminal.
/// This includes all its data.
pub fn update(
    db: &Database,
    previous_account: &str,
    previous_kind: &str,
    criminal: &Criminal,
) -> Result<()> {
    let previous_account = previous_account.trim();
    if previous_account.is_empty() || !criminal.is_valid() {
        return Err(Error::UnprocessableEntity);
    }
    let previous_kind = previous_kind.trim();
    if previous_kind.is_empty() {
        return Err(Error::InvalidKind);
    }

    let transaction = db.transaction()?;
    // update date
    transaction.execute(
        "update criminal set account=?, kind=?, data=? where account=? and kind=?",
        rusqlite::params![
            criminal.account.trim(),
            criminal.kind.trim(),
            criminal.data.trim(),
            previous_account,
            previous_kind
        ],
    )?;

    transaction.commit()?;
    Ok(())
}

/// Deletes the criminal.
/// This includes all its data.
pub fn delete(db: &Database, account: &str, kind: &str) -> Result<()> {
    let account = account.trim();
    if account.is_empty() {
        return Err(Error::UnprocessableEntity);
    }
    let kind = kind.trim();
    if kind.is_empty() {
        return Err(Error::InvalidKind);
    }
    let transaction = db.transaction()?;
    // remove date and presenters
    transaction.execute(
        "delete from criminal where account=? and kind=?",
        rusqlite::params![account, kind],
    )?;
    transaction.commit()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::db::criminal::{self, Criminal};
    use crate::db::project::{create, Database};
    #[test]
    fn add_update_remove_criminal() {
        let db = Database::memory().unwrap();
        create(&db).unwrap();

        let criminal = Criminal {
            account: "foo".to_string(),
            kind: "Destroy".to_string(),
            data: "Car Destroyed".into(),
        };
        criminal::add(&db, &criminal).unwrap();

        let result = criminal::search(&db, "").unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], criminal);

        criminal::update(
            &db,
            &criminal.account,
            &criminal.kind,
            &Criminal {
                data: "Car Stolen".into(),
                ..criminal.clone()
            },
        )
        .unwrap();
        let result = criminal::search(&db, "").unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].data, "Car Stolen".to_string());

        criminal::delete(&db, &criminal.account, &criminal.kind).unwrap();
        let result = criminal::search(&db, "").unwrap();
        assert_eq!(result.len(), 0);
    }
}
