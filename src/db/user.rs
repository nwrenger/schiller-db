use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::db::project::{DBIter, Database, Error, FromRow, Result};

/// Data object for a user.
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[cfg_attr(test, derive(PartialEq, Default))]
pub struct User {
    pub account: String,
    pub forename: String,
    pub surname: String,
    pub role: String,
}

impl User {
    pub fn is_valid(&self) -> bool {
        !self.account.trim().is_empty()
            && self.account.starts_with(char::is_alphabetic)
            && !self.forename.trim().is_empty()
            && !self.surname.trim().is_empty()
            && !self.role.trim().is_empty()
    }
}

impl FromRow for User {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<User> {
        Ok(User {
            account: row.get("account")?,
            forename: row.get("forename")?,
            surname: row.get("surname")?,
            role: row.get("role")?,
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
        role \
        from user \
        where account=?",
        [id],
        User::from_row,
    )?)
}

use std::collections::HashSet;

/// Returns all roles from the user table without duplicates
pub fn all_roles(db: &Database) -> Result<Vec<String>> {
    let mut stmt = db.con.prepare(
        "select \
        role \
        from user \
        order by role",
    )?;

    let mut rows = stmt.query([])?;
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

/// Parameters for the advanced search
///
/// Adding the '%' char allows every number of every character in this place
#[derive(Debug, Clone, Default)]
pub struct UserSearch<'a> {
    pub name: &'a str,
    pub role: &'a str,
}

impl<'a> UserSearch<'a> {
    pub fn new(name: &'a str, role: &'a str) -> UserSearch<'a> {
        Self { name, role }
    }
}

/// Performes a simple user search with the given `text`.
pub fn search(db: &Database, params: UserSearch, limit: usize) -> Result<Vec<User>> {
    let mut stmt = db.con.prepare(
        "select \
        account, \
        forename, \
        surname, \
        role \
        \
        from user \
        where (account like '%'||?1||'%' \
            or forename like '%'||?1||'%' \
            or surname like '%'||?1||'%') \
        and role like ?2 \
        order by case \
            when account like ?1 || '%' then 0 \
            else 1 \
        end asc, account asc \
        limit ?3",
    )?;
    let rows = stmt.query(rusqlite::params![
        params.name.trim(),
        params.role.trim(),
        limit
    ])?;
    DBIter::new(rows).collect()
}

/// Adds a new user.
pub fn add(db: &Database, user: &User) -> Result<()> {
    if !user.is_valid() {
        return Err(Error::InvalidUser);
    }
    db.con.execute(
        "insert into user values (?, ?, ?, ?)",
        rusqlite::params![
            user.account.trim(),
            user.forename.trim(),
            user.surname.trim(),
            user.role.trim(),
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
        "update user set account=?, forename=?, surname=?, role=? where account=?",
        rusqlite::params![
            user.account.trim(),
            user.forename.trim(),
            user.surname.trim(),
            user.role.trim(),
            previous_account,
        ],
    )?;

    // update absence
    transaction.execute(
        "update absence set account=? where account=?",
        [user.account.trim(), previous_account],
    )?;

    // update criminal
    transaction.execute(
        "update criminal set account=? where account=?",
        [user.account.trim(), previous_account],
    )?;

    // update login
    transaction.execute(
        "update login set user=? where user=?",
        [user.account.trim(), previous_account],
    )?;

    transaction.commit()?;
    Ok(())
}

/// Deletes the user.
/// This includes all its absences.
pub fn delete(db: &Database, account: &str) -> Result<()> {
    let account = account.trim();
    if account.is_empty() {
        return Err(Error::InvalidUser);
    }
    let transaction = db.transaction()?;
    // remove user
    transaction.execute("delete from user where account=?", [account])?;

    //remove from absence
    transaction.execute("delete from absence where account=?", [account])?;
    //remove from criminal
    transaction.execute("delete from criminal where account=?", [account])?;
    transaction.commit()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::db::project::{create, Database};
    use crate::db::user::{self, User, UserSearch};
    #[test]
    fn add_update_remove_users_all_roles() {
        let db = Database::memory().unwrap();
        create(&db).unwrap();

        let user = User {
            account: "foo.bar".into(),
            forename: "Foo".into(),
            surname: "Bar".into(),
            role: "Demo".into(),
        };
        user::add(&db, &user).unwrap();

        let result = user::search(&db, UserSearch::new("", "%"), 0).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], user);

        let result = user::all_roles(&db).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "Demo".to_string());

        user::update(
            &db,
            &user.account,
            &User {
                role: "Teacher".into(),
                ..user.clone()
            },
        )
        .unwrap();
        let result = user::search(&db, UserSearch::new("%foo%", "%"), 0).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].role, "Teacher");

        user::delete(&db, &user.account).unwrap();
        let result = user::search(&db, UserSearch::new("no one", "%"), 0).unwrap();
        assert_eq!(result.len(), 0);
    }
}
