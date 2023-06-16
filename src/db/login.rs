use crate::db::project::{Database, Error, FromRow, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use log::warn;
use rand::RngCore;
use rusqlite::types::{FromSql, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use utoipa::ToSchema;

#[repr(i64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, ToSchema, Default)]
pub enum Permission {
    #[default]
    None,
    ReadOnly,
    Write,
}

impl From<&str> for Permission {
    fn from(s: &str) -> Self {
        match s {
            "0" => Permission::None,
            "1" => Permission::ReadOnly,
            "2" => Permission::Write,
            _ => unimplemented!("Unknown permission variant"),
        }
    }
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

#[derive(Debug, PartialEq, Deserialize, Serialize, ToSchema)]
pub struct Permissions {
    pub access_user: Permission,
    pub access_absence: Permission,
    pub access_criminal: Permission,
}

impl FromRow for Permissions {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Permissions> {
        Ok(Permissions {
            access_user: row.get("access_user")?,
            access_absence: row.get("access_absence")?,
            access_criminal: row.get("access_criminal")?,
        })
    }
}

#[derive(Deserialize, PartialEq, Debug, ToSchema)]
pub struct Login {
    pub user: String,
    pub hash: String,
    pub salt: String,
    pub access_user: Permission,
    pub access_absence: Permission,
    pub access_criminal: Permission,
}

impl Login {
    pub fn is_valid(&self) -> bool {
        !self.user.trim().is_empty()
            && self.user.starts_with(char::is_alphabetic)
            && !self.user.contains(':')
            && !self.hash.trim().is_empty()
            && !self.salt.trim().is_empty()
    }
    pub fn compute_hash(salt: &str, password: &str) -> Result<String> {
        let Ok(salt) = BASE64.decode(salt) else {
            warn!("salt could not be decoded");
            return Err(Error::Unauthorized);
        };

        let mut hasher = sha2::Sha256::new();
        hasher.update(password);
        hasher.update(salt);
        Ok(BASE64.encode(hasher.finalize()))
    }
    pub fn check_password(&self, password: &str) -> bool {
        if let Ok(hash) = Self::compute_hash(&self.salt, password) {
            hash == self.hash
        } else {
            false
        }
    }
}

impl FromRow for Login {
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Login> {
        Ok(Login {
            user: row.get("user")?,
            hash: row.get("hash")?,
            salt: row.get("salt")?,
            access_user: row.get("access_user")?,
            access_absence: row.get("access_absence")?,
            access_criminal: row.get("access_criminal")?,
        })
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, ToSchema)]
pub struct NewLogin {
    pub user: String,
    pub password: String,
    pub access_user: Permission,
    pub access_absence: Permission,
    pub access_criminal: Permission,
}

impl NewLogin {
    pub fn salted(self) -> Result<Login> {
        let NewLogin {
            user,
            password,
            access_user,
            access_absence,
            access_criminal,
        } = self;
        let password = password.trim().to_string();
        if user.trim().is_empty() || password.is_empty() {
            return Err(Error::InvalidLogin);
        }

        let mut salt = [0; 32];
        rand::thread_rng().fill_bytes(&mut salt);
        let salt = BASE64.encode(salt);

        let hash = Login::compute_hash(&salt, &password)?;
        Ok(Login {
            user,
            hash,
            salt,
            access_user,
            access_absence,
            access_criminal,
        })
    }
}

/// Returns the login with the given `user` and `password`.
pub fn fetch(db: &Database, user: &str) -> Result<Login> {
    let mut stmt = db.con.prepare(
        "select \
        user, \
        hash, \
        salt, \
        access_user, \
        access_absence, \
        access_criminal \
        from login \
        where user=?
        limit 1",
    )?;
    let mut result = stmt.query([user])?;
    Ok(Login::from_row(result.next()?.ok_or(Error::NothingFound)?)?)
}

/// Returns the permissions of the user with the given `user`.
pub fn fetch_permission(db: &Database, user: &str) -> Result<Permissions> {
    let mut stmt = db.con.prepare(
        "select \
        access_user, \
        access_absence, \
        access_criminal \
        from login \
        where user=?
        limit 1",
    )?;
    let mut result = stmt.query([user])?;
    Ok(Permissions::from_row(
        result.next()?.ok_or(Error::NothingFound)?,
    )?)
}

pub fn all_logins(db: &Database) -> Result<Vec<String>> {
    let mut stmt = db.con.prepare(
        "select \
        user \
        from login \
        order by user",
    )?;

    let mut rows = stmt.query([])?;
    let mut users = Vec::new();
    while let Some(row) = rows.next()? {
        users.push(row.get(0)?);
    }

    Ok(users)
}

/// Adds a new login.
pub fn add(db: &Database, login: NewLogin) -> Result<()> {
    let login = login.salted()?;

    db.con.execute(
        "INSERT INTO login VALUES (?, ?, ?, ?, ?, ?)",
        rusqlite::params![
            login.user.trim(),
            login.hash,
            login.salt,
            login.access_user,
            login.access_absence,
            login.access_criminal
        ],
    )?;
    Ok(())
}

/// Updates a login.
/// This includes only it's user and password.
pub fn update(db: &Database, user: &str, password: &str) -> Result<()> {
    let login = NewLogin {
        user: user.to_string(),
        password: password.to_string(),
        ..Default::default()
    }
    .salted()?;

    db.con.execute(
        "update login set hash=?, salt=? where user=?",
        rusqlite::params![login.hash, login.salt, login.user.trim(),],
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

    use crate::db::login::{self, NewLogin, Permission};
    use crate::db::project::{create, Database};

    #[test]
    fn fetch_add_delete_logins() {
        let db = Database::memory().unwrap();
        create(&db).unwrap();

        let login = NewLogin {
            user: "nils.wrenger".into(),
            password: "123456".into(),
            access_user: Permission::ReadOnly,
            access_absence: Permission::Write,
            access_criminal: Permission::None,
        };
        login::add(&db, login.clone()).unwrap();

        let result = login::fetch(&db, &login.user);
        assert!(result.is_ok());
        assert!(result.unwrap().check_password(&login.password));

        let result = login::fetch_permission(&db, &login.user);
        assert!(result.is_ok());

        let result = login::all_logins(&db).unwrap();
        assert_eq!(result.len(), 1);

        login::delete(&db, &login.user).unwrap();

        let result = login::fetch(&db, &login.user);
        println!("{result:?}");
        assert!(result.is_err());
    }
}
