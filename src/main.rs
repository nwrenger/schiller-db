pub mod db;

use db::project::{User, Database};
use rusqlite::Result;

fn main() -> Result<()> {
    let db = Database::memory().unwrap();
    db::project::create(&db).unwrap();
    let me = User {
        account: "nils.wrenger".into(),
        forename: "Nils".into(),
        surname: "Wrenger".into(),
        role: "Klasse 10a".into(),
        criminal: false,
        data: None,
    };

    let new_me = User {
        account: "nils.wrenger".into(),
        forename: "Nils".into(),
        surname: "Wrenger".into(),
        role: "Klasse 10a".into(),
        criminal: true,
        data: Some("Hat nen Schuh geklaut!".into()),
    };

    let you = User {
        account: "lars.wrenger".into(),
        forename: "Lars".into(),
        surname: "Wrenger".into(),
        role: "".into(),
        criminal: false,
        data: None,
    };

    db::user::add(&db, &you).unwrap();
    db::user::add(&db, &me).unwrap();

    println!("All sorted by 'La':{:?}", db::user::search(&db, "La").unwrap());

    db::user::delete(&db, &you.account).unwrap();
    db::user::update(&db, &me.account, &new_me).unwrap();

    println!("Deleted account:lars.wrenger: {:?}", db::user::search(&db, "N").unwrap());

    Ok(())
}