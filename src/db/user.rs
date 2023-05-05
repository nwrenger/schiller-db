use crate::db::project::{DBIter, Database, Error, FromRow, User};

type Result<T> = std::result::Result<T, Error>;

impl User {
    pub fn is_valid(&self) -> bool {
        !self.account.trim().is_empty()
            && !self.forename.trim().is_empty()
            && !self.surname.trim().is_empty()
    }
}

impl FromRow for User {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<User> {
        Ok(User {
            account: row.get("account")?,
            forename: row.get("forename")?,
            surname: row.get("surname")?,
            role: row.get("role")?,
            criminal: row.get("criminal")?,
            data: row.get("data")?,
        })
    }
}

/// Returns the user with the given `id`.
pub fn fetch(db: &Database, id: &str) -> Result<User> {
    Ok(db.con.query_row(
        "select \
        account, \
        forename, \
        surname, \
        role, \
        criminal, \
        data \
        from user \
        where account=?",
        [id],
        User::from_row,
    )?)
}

/// Performes a simple user search with the given `text`.
pub fn search<'a>(db: &'a Database, text: &'a str) -> Result<Vec<User>> {
    let mut stmt = db.con.prepare(
        "select \
        account, \
        forename, \
        surname, \
        role, \
        criminal, \
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
        "update user set account=?, forename=?, surname=?, role=?, criminal=?, data=? where account=?",
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
        "update presence set presenter=? where presenter=?",
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
    transaction.execute("delete from presence where presenter=?", [account])?;
    transaction.commit()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::db::project::{create, Database, User};
    use crate::db::user;
    #[test]
    fn add_update_remove_users() {
        let db = Database::memory().unwrap();
        create(&db).unwrap();

        let user = User {
            account: "foo.bar".into(),
            forename: "Foo".into(),
            surname: "Bar".into(),
            role: "Demo".into(),
            criminal: false,
            data: None,
        };
        user::add(&db, &user).unwrap();

        let result = user::search(&db, "").unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], user);

        user::update(
            &db,
            &user.account,
            &User {
                role: "Teacher".into(),
                ..user.clone()
            },
        )
        .unwrap();
        let result = user::search(&db, "").unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].role, "Teacher");

        user::delete(&db, &user.account).unwrap();
        let result = user::search(&db, "").unwrap();
        assert_eq!(result.len(), 0);
    }
}
