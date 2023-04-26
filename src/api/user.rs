use crate::db::db::{User, Database, Error, DBIter, FromRow};

type Result<T> = std::result::Result<T, Error>;


/// Returns the user with the given `id`.
pub fn fetch(db: &Database, id: &str) -> Result<User> {
    Ok(db.con.query_row(
        "select \
        account, \
        forename, \
        surname, \
        role, \
        criminal \
        data \
        from user \
        where account=?",
        [id],
        User::from_row,
    )?)
}

/// Performes a simple user search with the given `text`.
pub fn search(db: &Database, text: &str) -> Result<Vec<User>> {
    let mut stmt = db.con.prepare(
        "select \
        account, \
        forename, \
        surname, \
        role, \
        criminal \
        data \
        \
        from user \
        where account like '%'||?1||'%' \
        or forename like '%'||?1||'%' \
        or surname like '%'||?1||'%' \
        or role like '%'||?1||'%' \
        order by account",
    )?;
    let rows = stmt.query([text.trim()])?;
    DBIter::new(rows).collect()
}

/// Adds a new user.
pub fn add(db: &Database, user: &User) -> Result<()> {
    if !user.is_valid() {
        return Err(Error::InvalidUser);
    }
    db.con.execute(
        "insert into user values (?, ?, ?, ?, ?, ?)",
        rusqlite::params![
            user.account.trim(),
            user.forename.trim(),
            user.surname.trim(),
            user.role.trim(),
            user.criminal,
            user.data,
        ],
    )?;
    Ok(())
}
/// Updates the user and all references if its account changes.
pub fn update(db: &Database, previous_account: &str, user: &User) -> Result<()> {
    let previous_account = previous_account.trim();
    if previous_account.is_empty() || !user.is_valid() {
        return Err(Error::InvalidUser);
    }
    let transaction = db.transaction()?;
    // update user
    transaction.execute(
        "update user set account=?, forename=?, surname=?, role=?, may_borrow=? where account=?",
        rusqlite::params![
            user.account.trim(),
            user.forename.trim(),
            user.surname.trim(),
            user.role.trim(),
            user.criminal,
            user.data,
            previous_account,
        ],
    )?;

    // update presence
    transaction.execute(
        "update medium set presence=? where presence=?",
        [user.account.trim(), previous_account],
    )?;

    transaction.commit()?;
    Ok(())
}

/// Deletes the user.
/// This includes all its presences.
pub fn delete(db: &Database, account: &str) -> Result<()> {
    let account = account.trim();
    if account.is_empty() {
        return Err(Error::InvalidUser);
    }
    let transaction = db.transaction()?;
    // remove user
    transaction.execute("delete from user where account=?", [account])?;

    //remove from presence
    transaction.execute(
        "update medium set presence='' \
        where reservation not in (select account from user);",
        [],
    )?;
    transaction.commit()?;
    Ok(())
}