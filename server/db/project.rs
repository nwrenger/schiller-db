use std::{
    borrow::Cow,
    fmt,
    fs::File,
    io::BufRead,
    path::{Path, PathBuf},
    ptr::addr_of,
};

use std::io::BufReader;

use rusqlite::{types::FromSql, Connection};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::login::NewLogin;
use crate::db::user::User;

macro_rules! error {
    ($($args:tt)*) => {
        println!($($args)*)
    };
}

/// Operation Error
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub enum Error {
    /// Default errors
    Arguments,
    Logic,
    NoProject,
    FileNotFound,
    FileOpen,
    SQL,
    Network,
    InvalidFormat,
    NothingFound,
    /// Specific errors
    InvalidUser,
    InvalidLogin,
    InvalidWorkless,
    InvalidCriminal,
    InvalidDate,
    InvalidKind,
    /// Server specific errors
    Unauthorized,
    ExceededLimit,
    PageNotFound,
    UnprocessableEntity,
    InternalError,
    /// Migration
    UnsupportedProjectVersion,
}

impl From<rusqlite::Error> for Error {
    fn from(e: rusqlite::Error) -> Self {
        error!("SQL: {e}");
        Self::SQL
    }
}

impl From<std::convert::Infallible> for Error {
    fn from(e: std::convert::Infallible) -> Self {
        error!("convert::Infallible: {e:?}");
        Self::Arguments
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        error!("File Error: {e:?}");
        Self::FileOpen
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait FromRow: Sized {
    fn from_row(stmt: &rusqlite::Row) -> rusqlite::Result<Self>;
}

impl<T: FromSql> FromRow for T {
    fn from_row(stmt: &rusqlite::Row) -> rusqlite::Result<Self> {
        stmt.get(1)
    }
}

impl<'a, T: FromRow> Iterator for DBIter<'a, T> {
    type Item = Result<T>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.rows.next() {
            Ok(row) => Some(T::from_row(row?).map_err(Into::into)),
            Err(e) => Some(Err(e.into())),
        }
    }
}

pub struct DBIter<'a, T> {
    rows: rusqlite::Rows<'a>,
    ty: std::marker::PhantomData<T>,
}

impl<'a, T> DBIter<'a, T> {
    pub fn new(rows: rusqlite::Rows<'a>) -> Self {
        DBIter {
            rows,
            ty: std::marker::PhantomData,
        }
    }
}

pub struct Database {
    path: PathBuf,
    pub con: rusqlite::Connection,
}
impl fmt::Debug for Database {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Database")
            .field("path", &self.path)
            .finish()
    }
}

impl Database {
    /// Creates a new database at the given path.
    pub fn create(path: Cow<'_, Path>) -> Result<Database> {
        if !path.exists() {
            let database = Database {
                con: rusqlite::Connection::open_with_flags(
                    &path,
                    rusqlite::OpenFlags::SQLITE_OPEN_CREATE
                        | rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE,
                )
                .map_err(|_| Error::FileOpen)?,
                path: path.into_owned(),
            };
            Ok(database)
        } else {
            Err(Error::FileOpen)
        }
    }

    /// Opens a database connection to the given project database.
    pub fn open(path: Cow<'_, Path>) -> Result<(Database, bool)> {
        if path.exists() {
            let database = Database {
                con: rusqlite::Connection::open_with_flags(
                    &path,
                    rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE,
                )
                .map_err(|_| Error::FileOpen)?,
                path: path.into_owned(),
            };
            Ok((database, true))
        } else {
            Err(Error::FileNotFound)
        }
    }

    /// Returns the filepath to this database.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// In memory database for testing purposes.
    pub fn memory() -> Result<Database> {
        Ok(Database {
            path: PathBuf::new(),
            con: rusqlite::Connection::open_in_memory()?,
        })
    }

    /// Creates a rollback point.
    /// If any statement on a transaction fails, all changes are rolled back
    /// to the point before this function is called.
    ///
    /// ## Safety
    /// This operation is only safe if called once.
    /// Stacking transactions on top of each other is not allowed!
    pub fn transaction(&self) -> rusqlite::Result<rusqlite::Transaction> {
        let con = unsafe { &mut *(addr_of!(self.con) as *mut Connection) };
        con.transaction()
    }
}

pub fn create(db: &Database) -> Result<()> {
    const CREATE_TABLES: &str = "\
    create table user ( \
        account text not null primary key, \
        forename text not null, \
        surname text not null, \
        role text not null); \
    \
    create table workless ( \
        account text not null, \
        old_company text not null, \
        date_of_dismiss text not null, \
        currently integer not null default 1, \
        new_company text not null, \
        total_time text text not null, \
        primary key (account, old_company, date_of_dismiss)); \
    \
    create table criminal ( \
        account text not null, \
        kind text not null, \
        accuser text not null, \
        police_consultant text not null, \
        lawyer_culprit text not null, \
        lawyer_accuser text not null, \
        facts text not null, \
        time_of_crime text not null, \
        location_of_crime text not null, \
        note text not null, \
        verdict text not null, \
        primary key (account, kind));
    \
    create table login ( \
        user text not null primary key, \
        hash text not null, \
        salt text not null, \
        access_user int default 0, \
        access_workless int default 0, \
        access_criminal int default 0); \
    ";

    let transaction = db.transaction()?;
    transaction.execute_batch(CREATE_TABLES)?;
    transaction.commit()?;
    Ok(())
}

//Fetches User Data from a file. Performance might not be the best.
//Ignore the Error messages!
pub fn fetch_user_data(db: &Database, path: Cow<'_, Path>, div: &str) -> Result<()> {
    if path.exists() {
        let reader = BufReader::new(File::open(path)?);
        for i in reader.lines() {
            let line = i?;
            let mut lines = line.split(div);
            let user = User {
                account: lines.next().unwrap().into(),
                forename: lines.next().unwrap().into(),
                surname: lines.next().unwrap().into(),
                role: lines.next().unwrap().into(),
            };
            if super::user::add(db, &user).is_err()
                && (user.role == "Lehrer"
                    || (user.role.starts_with("Klasse")
                        && super::user::fetch(db, &user.account)?.role != "Lehrer")
                        && !user.role.contains("Lehrer")
                        && !user.role.contains("Bio"))
            {
                super::user::update(db, &user.account, &user)?;
            }
        }
        Ok(())
    } else {
        Err(Error::FileNotFound)
    }
}

//Fetches Logins from a file. Performance might not be the best.
//Ignore the Error messages!
pub fn fetch_logins(db: &Database, path: Cow<'_, Path>, div: &str) -> Result<()> {
    if path.exists() {
        let reader = BufReader::new(File::open(path)?);
        for i in reader.lines() {
            let line = i?;
            let mut lines = line.split(div);
            let login = NewLogin {
                user: lines.next().unwrap().into(),
                password: lines.next().unwrap().into(),
                access_user: lines.next().unwrap().into(),
                access_workless: lines.next().unwrap().into(),
                access_criminal: lines.next().unwrap().into(),
            };
            super::login::add(db, login)?;
        }
        Ok(())
    } else {
        Err(Error::FileNotFound)
    }
}
