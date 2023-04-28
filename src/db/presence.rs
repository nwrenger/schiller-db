use crate::db::project::{Presence, Database, Error, DBIter, FromRow};

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
        Ok(Presence { date: row.get("date")?, presenter: row.get("presenter")?, })
    }
}

/// Returns the presence with the given `id`.
pub fn fetch(db: &Database, id: &str) -> Result<Presence> {
    Ok(db.con.query_row(
        "select \
        date, \
        presenter \
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
        presenter \
        \
        from presence \
        where date like '%'||?1||'%' \
        or presenter like '%'||?1||'%' \
        order by date",
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
        "INSERT INTO presence VALUES (?, ?)",
        rusqlite::params![
            presence.date.unwrap(),
            presence.presenter,
        ],
    )?;
    Ok(())
}


/// Updates the presences.
/// This includes all its presenters and dates.
pub fn update(db: &Database, previous_presence: &Presence, presence: &Presence) -> Result<()> {
    if !presence.is_valid() {
        return Err(Error::InvalidDate);
    }
    if !previous_presence.is_valid() {
        return Err(Error::InvalidDate);
    }
    let transaction = db.transaction()?;
    // update date
    transaction.execute(
        "update presence set date=?, presenter=? where date=?",
        rusqlite::params![
            presence.date.unwrap(),
            presence.presenter,
            previous_presence.date.unwrap(),
        ],
    )?;

    transaction.commit()?;
    Ok(())
}

/// Deletes the date.
/// This includes all its presenters.
pub fn delete(db: &Database, date: Option<NaiveDate>) -> Result<()> {
    if date.is_some() {
        let transaction = db.transaction()?;
        // remove date and presenters
        transaction.execute("delete from presence where date=?", [Some(date)])?;
        transaction.commit()?;
        Ok(())
    } else {
        Err(Error::InvalidDate)
    }
}