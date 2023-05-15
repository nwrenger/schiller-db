use crate::db::project::{Criminal, DBIter, Database, Error, FromRow, Result};

impl Criminal {
    pub fn is_valid(&self) -> bool {
        !self.account.trim().is_empty()
    }
}

impl FromRow for Criminal {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Criminal> {
        Ok(Criminal {
            account: row.get("account")?,
            data: row.get("data")?,
        })
    }
}

/// Returns the criminal with the given `account`.
pub fn fetch(db: &Database, account: &str) -> Result<Criminal> {
    Ok(db.con.query_row(
        "select \
        account, \
        data \
        from criminal \
        where account=?",
        [account],
        Criminal::from_row,
    )?)
}

/// Performes a simple criminal search with the given `text`.
pub fn search(db: &Database, text: &str) -> Result<Vec<Criminal>> {
    let mut stmt = db.con.prepare(
        "select \
        account, \
        data \
        \
        from criminal \
        where account like '%'||?1||'%' \
        or data like '%'||?1||'%' \
        order by account",
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
        "INSERT INTO criminal VALUES (?, ?)",
        rusqlite::params![criminal.account.trim(), criminal.data,],
    )?;
    Ok(())
}

/// Updates the criminal.
/// This includes all its data.
pub fn update(db: &Database, previous_account: &str, criminal: &Criminal) -> Result<()> {
    let previous_account = previous_account.trim();
    if previous_account.is_empty() || !criminal.is_valid() {
        return Err(Error::InvalidUser);
    }

    let transaction = db.transaction()?;
    // update date
    transaction.execute(
        "update criminal set account=?, data=? where account=?",
        rusqlite::params![criminal.account, criminal.data, previous_account,],
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
    transaction.execute("delete from criminal where account=?", [account])?;
    transaction.commit()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::db::criminal;
    use crate::db::project::{create, Criminal, Database};
    #[test]
    fn add_update_remove_criminal() {
        let db = Database::memory().unwrap();
        create(&db).unwrap();

        let criminal = Criminal {
            account: "foo".to_string(),
            data: "Car Destroyed".into(),
        };
        criminal::add(&db, &criminal).unwrap();

        let result = criminal::search(&db, "").unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], criminal);

        criminal::update(
            &db,
            &criminal.account,
            &Criminal {
                data: "Car Stolen".into(),
                ..criminal.clone()
            },
        )
        .unwrap();
        let result = criminal::search(&db, "").unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].data, "Car Stolen".to_string());

        criminal::delete(&db, &criminal.account).unwrap();
        let result = criminal::search(&db, "").unwrap();
        assert_eq!(result.len(), 0);
    }
}
