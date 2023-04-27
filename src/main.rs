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

    println!("Deleted account:lars.wrenger: {:?}", db::user::search(&db, "La").unwrap());

    Ok(())
}