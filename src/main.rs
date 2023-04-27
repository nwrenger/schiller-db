pub mod db;
pub mod api;

use db::db::{User, Database};
use rusqlite::Result;

fn main() -> Result<()> {
    let db = Database::memory().unwrap();
    db::db::create(&db).unwrap();
    let me = User {
        account: "nils.wrenger".into(),
        forename: "Nils".into(),
        surname: "Wrenger".into(),
        role: "Klasse 10a".into(),
        criminal: false,
        data: None,
    };

    let you = User {
        account: "lars.wrenger".into(),
        forename: "Lars".into(),
        surname: "Wrenger".into(),
        role: "".into(),
        criminal: false,
        data: None,
    };

    api::user::add(&db, &me).unwrap();
    api::user::add(&db, &you).unwrap();

    println!("{:?}", api::user::search(&db, "Nils").unwrap());

    Ok(())
}