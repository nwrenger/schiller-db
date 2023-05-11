use crate::db::project::{Criminal, DBIter, Database, Error, FromRow, Result};

impl Criminal {
    pub fn is_valid(&self) -> bool {
        !self.criminal.trim().is_empty()
    }
}

impl FromRow for Criminal {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Criminal> {
        Ok(Criminal {
            criminal: row.get("criminal")?,
            data: row.get("data")?,
        })
    }
}

/// Returns the criminal with the given `account`.
pub fn fetch(db: &Database, account: &str) -> Result<Criminal> {
    Ok(db.con.query_row(
        "select \
        criminal, \
        data \
        from criminals \
        where criminal=?",
        [account],
        Criminal::from_row,
    )?)
}

/// Performes a simple criminal search with the given `text`.
pub fn search(db: &Database, text: &str) -> Result<Vec<Criminal>> {
    let mut stmt = db.con.prepare(
        "select \
        criminal, \
        data \
        \
        from criminals \
        where criminal like '%'||?1||'%' \
        or data like '%'||?1||'%' \
        order by criminal",
    )?;
    let rows = stmt.query([text.trim()])?;
    DBIter::new(rows).collect()
}

/// Adds a new criminal.
pub fn add(db: &Database, criminal: &Criminal) -> Result<()> {
    if !criminal.is_valid() {
        return Err(Error::InvalidUser);
    }
    db.con.execute(
        "INSERT INTO criminals VALUES (?, ?)",
        rusqlite::params![criminal.criminal.trim(), criminal.data,],
    )?;
    Ok(())
}

/// Updates the criminal.
/// This includes all its criminals and data.
pub fn update(db: &Database, previous_account: &str, criminal: &Criminal) -> Result<()> {
    let previous_account = previous_account.trim();
    if previous_account.is_empty() || !criminal.is_valid() {
        return Err(Error::InvalidUser);
    }

    let transaction = db.transaction()?;
    // update date
    transaction.execute(
        "update criminals set criminal=?, data=? where criminal=?",
        rusqlite::params![criminal.criminal, criminal.data, previous_account,],
    )?;

    transaction.commit()?;
    Ok(())
}

/// Deletes the criminal.
/// This includes all its data.
pub fn delete(db: &Database, account: &str) -> Result<()> {
    let account = account.trim();
    if account.is_empty() {
        return Err(Error::InvalidUser);
    }
    let transaction = db.transaction()?;
    // remove date and presenters
    transaction.execute("delete from criminals where criminal=?", [account])?;
    transaction.commit()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::db::criminals;
    use crate::db::project::{create, Criminal, Database};
    #[test]
    fn add_update_remove_criminals() {
        let db = Database::memory().unwrap();
        create(&db).unwrap();

        let criminal = Criminal {
            criminal: "foo".to_string(),
            data: None,
        };
        criminals::add(&db, &criminal).unwrap();

        let result = criminals::search(&db, "").unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], criminal);

        criminals::update(
            &db,
            &criminal.criminal,
            &Criminal {
                data: Some("Car Stolen".into()),
                ..criminal.clone()
            },
        )
        .unwrap();
        let result = criminals::search(&db, "").unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].data, Some("Car Stolen".into()));

        criminals::delete(&db, &criminal.criminal).unwrap();
        let result = criminals::search(&db, "").unwrap();
        assert_eq!(result.len(), 0);
    }
}
