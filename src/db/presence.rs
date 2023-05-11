use crate::db::project::{DBIter, Database, Error, FromRow, Presence, Result};

use chrono::NaiveDate;

impl Presence {
    pub fn is_valid(&self) -> bool {
        !self.presenter.trim().is_empty()
    }
}

impl FromRow for Presence {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Presence> {
        Ok(Presence {
            presenter: row.get("presenter")?,
            date: row.get("date")?,
            data: row.get("data")?,
        })
    }
}

/// Returns the presence with the given `account` and `date`.
pub fn fetch(db: &Database, account: &str, date: NaiveDate) -> Result<Presence> {
    Ok(db.con.query_row(
        "select \
        presenter, \
        date, \
        data \
        from presence \
        where presenter=? and date=?",
        rusqlite::params![account, date],
        Presence::from_row,
    )?)
}

/// Performes a simple presence search with the given `text`.
pub fn search(db: &Database, text: &str) -> Result<Vec<Presence>> {
    let mut stmt = db.con.prepare(
        "select \
        presenter, \
        date, \
        data \
        \
        from presence \
        where presenter like '%'||?1||'%' \
        or date like '%'||?1||'%' \
        or data like '%'||?1||'%' \
        order by presenter",
    )?;
    let rows = stmt.query([text.trim()])?;
    DBIter::new(rows).collect()
}

/// Adds a new date with presenters.
pub fn add(db: &Database, presence: &Presence) -> Result<()> {
    if !presence.is_valid() {
        return Err(Error::InvalidUser);
    }
    db.con.execute(
        "INSERT INTO presence VALUES (?, ?, ?)",
        rusqlite::params![presence.presenter.trim(), presence.date, presence.data,],
    )?;
    Ok(())
}

/// Updates the presences.
/// This includes all its presenters, dates and data.
pub fn update(
    db: &Database,
    previous_account: &str,
    previous_date: NaiveDate,
    presence: &Presence,
) -> Result<()> {
    let previous_account = previous_account.trim();
    if previous_account.is_empty() || !presence.is_valid() {
        return Err(Error::InvalidUser);
    }

    let transaction = db.transaction()?;
    // update date
    transaction.execute(
        "update presence set presenter=?, date=?, data=? where presenter=? and date=?",
        rusqlite::params![
            presence.presenter.trim(),
            presence.date,
            presence.data,
            previous_account,
            previous_date,
        ],
    )?;

    transaction.commit()?;
    Ok(())
}

/// Deletes the date and presenter.
/// This includes all its data.
pub fn delete(db: &Database, account: &str, date: NaiveDate) -> Result<()> {
    let account = account.trim();
    if account.is_empty() {
        return Err(Error::InvalidUser);
    }
    let transaction = db.transaction()?;
    // remove date and presenters
    transaction.execute(
        "delete from presence where presenter=? and date=?",
        rusqlite::params![account, date],
    )?;
    transaction.commit()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::db::presence;
    use crate::db::project::{create, Database, Presence};
    #[test]
    fn add_update_remove_presences() {
        let db = Database::memory().unwrap();
        create(&db).unwrap();

        let presence = Presence {
            presenter: "foo.bar".into(),
            date: NaiveDate::from_ymd_opt(2023, 4, 26).unwrap(),
            data: None,
        };
        presence::add(&db, &presence).unwrap();

        let result = presence::search(&db, "").unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], presence);

        presence::update(
            &db,
            &presence.presenter,
            NaiveDate::from_ymd_opt(2023, 4, 26).unwrap(),
            &Presence {
                data: Some("5 Mins Late".into()),
                ..presence.clone()
            },
        )
        .unwrap();
        let result = presence::search(&db, "").unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].data, Some("5 Mins Late".into()));

        presence::delete(
            &db,
            &presence.presenter,
            NaiveDate::from_ymd_opt(2023, 4, 26).unwrap(),
        )
        .unwrap();
        let result = presence::search(&db, "").unwrap();
        assert_eq!(result.len(), 0);
    }
}
