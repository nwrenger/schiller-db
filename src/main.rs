pub mod db;

use std::{borrow::Cow, path::Path};

use chrono::NaiveDate;

use db::project::{Database, Presence, User};

fn main() {
    let db: Database = match Database::open(Cow::from(Path::new("./my.db"))) {
        Ok(_) => Database::open(Cow::from(Path::new("./my.db"))).unwrap().0,
        _ => Database::create(Cow::from(Path::new("./my.db"))).unwrap(),
    };
    if !Database::open(Cow::from(Path::new("./my.db"))).unwrap().1 {
        db::project::create(&db).unwrap();
    }
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
        date: NaiveDate::from_ymd_opt(2023, 4, 2),
        presenter: me.account.clone(),
    };

    if Database::open(Cow::from(Path::new("./my.db"))).unwrap().1 {
        db::presence::delete(&db, presence.date).unwrap();
        db::presence::delete(&db, lars_presence.date).unwrap();
        db::user::delete(&db, &you.account).unwrap();
        db::user::delete(&db, &me.account).unwrap();
    }

    db::presence::add(&db, &presence).unwrap();
    db::presence::add(&db, &lars_presence).unwrap();
    db::user::add(&db, &you).unwrap();
    db::user::add(&db, &me).unwrap();

    println!(
        "All sorted by 'La':{:#?}",
        db::user::search(&db, "La").unwrap()
    );
    println!(
        "All sorted by 'La':{:#?}",
        db::presence::search(&db, "wre").unwrap()
    );

    db::presence::update(&db, &me.account, &new_presence).unwrap();
    println!(
        "Updated nils.wrenger, sorted by '':{:#?}",
        db::presence::search(&db, "").unwrap()
    );

    db::user::update(&db, &me.account, &new_me).unwrap();
}
