use crate::db::project::{Database, Error, FromRow, Result};
use rusqlite::types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use serde::Deserialize;
use utoipa::ToSchema;

#[repr(i64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, ToSchema)]
pub enum Permission {
    None,
    ReadOnly,
    Write,
}

impl FromSql for Permission {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        i64::column_result(value).and_then(|value| match value {
            0 => Ok(Permission::None),
            1 => Ok(Permission::ReadOnly),
            2 => Ok(Permission::Write),
            _ => Err(rusqlite::types::FromSqlError::OutOfRange(2)),
        })
    }
}

impl ToSql for Permission {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok((*self as i64).into())
    }
}

#[derive(Deserialize, PartialEq, Debug, ToSchema)]
pub struct Login {
    pub user: String,
    pub password: String,
    pub access_user: Permission,
    pub access_absence: Permission,
    pub access_criminal: Permission,
}

impl Login {
    pub fn is_valid(&self) -> bool {
        !self.user.trim().is_empty() && !self.user.contains(':')
    }
}

impl FromRow for Login {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Login> {
        Ok(Login {
            user: row.get("user")?,
            password: row.get("password")?,
            access_user: row.get("access_user")?,
            access_absence: row.get("access_absence")?,
            access_criminal: row.get("access_criminal")?,
        })
    }
}

/// Returns the login with the given `user` and `password`.
pub fn fetch(db: &Database, user: &str, password: &str) -> Result<Login> {
    let mut stmt = db.con.prepare(
        "select \
        user, \
        password, \
        access_user, \
        access_absence, \
        access_criminal \
        from login \
        where user=? and password=?
        limit 1",
    )?;
    let mut result = stmt.query([user, password])?;
    Ok(Login::from_row(result.next()?.ok_or(Error::NothingFound)?)?)
}

/// Adds a new date with presenters.
pub fn add(db: &Database, login: &Login) -> Result<()> {
    if !login.is_valid() {
        return Err(Error::InvalidUser);
    }
    db.con.execute(
        "INSERT INTO login VALUES (?, ?, ?, ?, ?)",
        rusqlite::params![
            login.user.trim(),
            login.password.trim(),
            login.access_user,
            login.access_absence,
            login.access_criminal
        ],
    )?;
    Ok(())
}

/// Deletes the login by user.
pub fn delete(db: &Database, user: &str) -> Result<()> {
    let user = user.trim();
    if user.is_empty() {
        return Err(Error::InvalidUser);
    }
    // remove non-admin users
    db.con
        .execute("delete from login where user=?", rusqlite::params![user])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    //TODO: Tests

    use crate::db::login::{self, Login, Permission};
    use crate::db::project::{create, Database};

    #[test]
    fn fetch_add_delete_logins() {
        let db = Database::memory().unwrap();
        create(&db).unwrap();

        let login = Login {
            user: "nils.wrenger".into(),
            password: "123456".into(),
            access_user: Permission::ReadOnly,
            access_absence: Permission::Write,
            access_criminal: Permission::None,
        };
        login::add(&db, &login).unwrap();

        let result = login::fetch(&db, &login.user, &login.password);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), login);

        login::delete(&db, &login.user).unwrap();

        let result = login::fetch(&db, &login.user, &login.password);
        println!("{result:?}");
        assert!(result.is_err());
    }
}
