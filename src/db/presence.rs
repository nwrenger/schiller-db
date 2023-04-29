use crate::db::project::{DBIter, Database, Error, FromRow, Presence};

use chrono::NaiveDate;

type Result<T> = std::result::Result<T, Error>;

//maybe used for later
impl Presence {
    pub fn is_valid(&self) -> bool {
        if !self.presenter.trim().is_empty() || self.date != None {
            true
        } else {
            false
        }
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
pub fn fetch(db: &Database, account: &str, date: Option<NaiveDate>) -> Result<Presence> {
    if date.is_none() {
        return Err(Error::InvalidDate);
    }
    Ok(db.con.query_row(
        "select \
        presenter, \
        date, \
        data \
        from presence \
        where presenter=? and date=?",
        rusqlite::params![account, date.unwrap()],
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
        return Err(Error::InvalidDate);
    }
    db.con.execute(
        "INSERT INTO presence VALUES (?, ?, ?)",
        rusqlite::params![
            presence.presenter.trim(),
            presence.date.unwrap(),
            presence.data,
        ],
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
        "update presence set presenter=?, date=?, data=? where presenter=? and date=?",
        rusqlite::params![
            presence.presenter.trim(),
            presence.date.unwrap(),
            presence.data,
            previous_account,
            previous_date.unwrap(),
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
        rusqlite::params![account, date.unwrap(),],
    )?;
    transaction.commit()?;
    Ok(())
}
