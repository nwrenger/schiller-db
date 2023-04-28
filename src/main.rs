pub mod db;

use chrono::NaiveDate;

use db::project::{User, Presence, Database};

fn main() {
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

    let presence = Presence {
        date: NaiveDate::from_ymd_opt(2023, 4, 29),
        presenter: me.account.clone(),
    };

    let lars_presence = Presence {
        date: NaiveDate::from_ymd_opt(2023, 4, 2),
        presenter: you.account.clone(),
    };

    let new_presence = Presence {
        date: NaiveDate::from_ymd_opt(2023, 3, 29),
        presenter: me.account.clone(),
    };

    db::presence::add(&db, &presence).unwrap();
    db::presence::add(&db, &lars_presence).unwrap();
    db::user::add(&db, &you).unwrap();
    db::user::add(&db, &me).unwrap();

    println!("All sorted by 'La':{:?}", db::user::search(&db, "La").unwrap());
    println!("All sorted by 'n':{:?}", db::presence::search(&db, "n").unwrap());

    db::presence::update(&db, &presence, &new_presence).unwrap();
    println!("Updated nils.wrenger, sorted by '':{:?}", db::presence::search(&db, "").unwrap());
    
    // db::presence::delete(&db, NaiveDate::from_ymd_opt(2023, 3, 29)).unwrap();
    db::user::update(&db, &me.account, &new_me).unwrap();
    db::user::delete(&db, &me.account).unwrap();

    println!("Deleted account:nils.wrenger: {:?}", db::user::search(&db, "").unwrap());
    println!("Deleted Date: {:?}", db::presence::search(&db, "").unwrap())

}