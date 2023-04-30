pub mod db;

use std::{borrow::Cow, path::Path};

use chrono::NaiveDate;

use db::project::{Database, Presence, User, fetch_user_data};

fn main() {
    let path = Path::new("./pdm.db");
    let db: Database = match Database::open(Cow::from(path)) {
        Ok(_) => Database::open(Cow::from(path)).unwrap().0,
        Err(_) => {
                let db = Database::create(Cow::from(path)).unwrap();
                db::project::create(&db).unwrap();
                fetch_user_data(&db, Cow::from(Path::new("./benutzer.txt")), "|").unwrap();
                db
            }
        };
    println!("{:#?}", db::stats::fetch(&db).unwrap());
    //todo Swagger UI
    
}
