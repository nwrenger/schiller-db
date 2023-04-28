use crate::db::project::{DBIter, Database, Error, FromRow, Presence};

use chrono::NaiveDate;

type Result<T> = std::result::Result<T, Error>;

//maybe used for later
impl Presence {
    pub fn is_valid(&self) -> bool {
        if self.date != None {
            self.date.unwrap();
            true
        } else {
            false
        }
    }
}

impl FromRow for Presence {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Presence> {
        Ok(Presence {
            date: row.get("date")?,
            presenter: row.get("presenter")?,
            data: row.get("data")?,
        })
    }
}

/// Returns the presence with the given `id`.
pub fn fetch(db: &Database, id: &str) -> Result<Presence> {
    Ok(db.con.query_row(
        "select \
        date, \
        presenter, \
        data \
        from presence \
        where date=?",
        [id],
        Presence::from_row,
    )?)
}

/// Performes a simple presence search with the given `text`.
pub fn search(db: &Database, text: &str) -> Result<Vec<Presence>> {
    let mut stmt = db.con.prepare(
        "select \
        date, \
        presenter, \
        data \
        \
        from presence \
        where date like '%'||?1||'%' \
        or presenter like '%'||?1||'%' \
        or data like '%'||?1||'%' \
        order by presenter",
    )?;
    let rows = stmt.query([text.trim()])?;
    DBIter::new(rows).collect()
}

/// Adds a new date with presenters.
pub fn add(db: &Database, presence: &Presence) -> Result<()> {
    if !presence.is_valid() {
        return Err(Error::InvalidDate);
    }
    db.con.execute(
        "INSERT INTO presence VALUES (?, ?, ?)",
        rusqlite::params![presence.date.unwrap(), presence.presenter.trim(), presence.data],
    )?;
    Ok(())
}

/// Updates the presences.
/// This includes all its presenters, dates and data.
pub fn update(
    db: &Database,
    previous_account: &str,
    previous_date: Option<NaiveDate>,
    presence: &Presence,
) -> Result<()> {
    let previous_account = previous_account.trim();
    if previous_account.is_empty() {
        return Err(Error::InvalidUser);
    }
    if !presence.is_valid() || previous_date.is_none() {
        return Err(Error::InvalidDate);
    }

    let transaction = db.transaction()?;
    // update date
    transaction.execute(
        "update presence set date=?, presenter=?, data=? where date=? and presenter=?",
        rusqlite::params![
            presence.date.unwrap(),
            presence.presenter.trim(),
            presence.data,
            previous_date.unwrap(),
            previous_account,
        ],
    )?;

    transaction.commit()?;
    Ok(())
}

/// Deletes the date and presenter.
/// This includes all its data.
pub fn delete(db: &Database, account: &str, date: Option<NaiveDate>) -> Result<()> {
    let account = account.trim();
    if account.is_empty() {
        return Err(Error::InvalidUser);
    }
    if date.is_none() {
        return Err(Error::InvalidDate);
    }
    let transaction = db.transaction()?;
    // remove date and presenters
    transaction.execute(
        "delete from presence where presenter=? and date=?",
        rusqlite::params![
            account,
            date.unwrap(),
        ],
    )?;
    transaction.commit()?;
    Ok(())
}
