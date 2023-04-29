pub mod db;

use std::{borrow::Cow, path::Path};

use chrono::NaiveDate;

use db::project::{Database, Presence, User};

fn main() {
    let db: Database = match Database::open(Cow::from(Path::new("./my.db"))) {
        Ok(_) => Database::open(Cow::from(Path::new("./my.db"))).unwrap().0,
        Err(_) => {
            let db = Database::create(Cow::from(Path::new("./my.db"))).unwrap();
            db::project::create(&db).unwrap();
            db
        }
    };

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
        presenter: me.account.clone(),
        date: NaiveDate::from_ymd_opt(2023, 4, 29),
        data: None,
    };

    let other_presence = Presence {
        presenter: me.account.clone(),
        date: NaiveDate::from_ymd_opt(2023, 4, 30),
        data: Some("war 5 Min zu Spät".into()),
    };

    let new_presence = Presence {
        presenter: me.account.clone(),
        date: NaiveDate::from_ymd_opt(2023, 4, 2),
        data: Some("Oh doch eher 10 Min".into()),
    };

    let lars_presence = Presence {
        presenter: you.account.clone(),
        date: NaiveDate::from_ymd_opt(2023, 4, 2),
        data: None,
    };

    if Database::open(Cow::from(Path::new("./my.db"))).unwrap().1 {
        db::presence::delete(&db, &presence.presenter, presence.date).unwrap();
        db::presence::delete(&db, &lars_presence.presenter, lars_presence.date).unwrap();
        db::presence::delete(&db, &lars_presence.presenter, lars_presence.date).unwrap();
        db::user::delete(&db, &you.account).unwrap();
        db::user::delete(&db, &me.account).unwrap();
    }

    db::presence::add(&db, &presence).unwrap();
    db::presence::add(&db, &other_presence).unwrap();
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

    db::presence::update(
        &db,
        &me.account,
        NaiveDate::from_ymd_opt(2023, 4, 29),
        &new_presence,
    )
    .unwrap();

    println!(
        "Fetching user nils.wrenger:\n{:#?}",
        db::user::fetch(&db, "nils.wrenger").unwrap()
    );

    println!(
        "Fetching presence 2023-04-02 and nils.wrenger:\n{:#?}",
        db::presence::fetch(&db, "nils.wrenger", NaiveDate::from_ymd_opt(2023, 4, 2)).unwrap()
    );

    db::presence::delete(&db, &other_presence.presenter, other_presence.date).unwrap();
    db::user::update(&db, &me.account, &new_me).unwrap();
}
