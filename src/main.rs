#[macro_use]
extern crate rocket;

pub mod db;

use std::{borrow::Cow, path::Path};

use chrono::NaiveDate;

use db::project::{fetch_user_data, Database, Error, Presence, User};

use rocket::serde::json::Json;

#[get("/user/all")]
async fn all_users() -> Json<Result<Vec<User>, Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::user::search(&db, ""))
}

#[get("/user/fetch/<id>")]
async fn fetch_user(id: &str) -> Json<Result<User, Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::user::fetch(&db, id))
}

#[get("/user/search/<text>")]
async fn search_user(text: &str) -> Json<Result<Vec<User>, Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::user::search(&db, text))
}

#[post("/user/add", format = "json", data = "<user>")]
async fn add_user(user: Json<User>) -> Json<Result<(), Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::user::add(&db, &user))
}

#[put("/user/update", format = "json", data = "<user>")]
async fn update_user(user: Json<User>) -> Json<Result<(), Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::user::update(&db, &user.account, &user))
}

#[delete("/user/delete/<id>")]
async fn delete_user(id: &str) -> Json<Result<(), Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::user::delete(&db, id))
}

#[get("/presence/all")]
async fn all_presences() -> Json<Result<Vec<Presence>, Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::presence::search(&db, ""))
}

#[get("/presence/fetch/<account>/<date>")]
async fn fetch_presence(account: &str, date: &str) -> Json<Result<Presence, Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    let date = match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        Ok(_) => NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap(),
        Err(_) => {
            return Json(Err(Error::InvalidDate));
        }
    };
    Json(db::presence::fetch(&db, account, date))
}

#[get("/presence/search/<text>")]
async fn search_presence(text: &str) -> Json<Result<Vec<Presence>, Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::presence::search(&db, text))
}

#[post("/presence/add", format = "json", data = "<presence>")]
async fn add_presence(presence: Json<Presence>) -> Json<Result<(), Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::presence::add(&db, &presence))
}

#[put("/presence/update", format = "json", data = "<presence>")]
async fn update_presence(presence: Json<Presence>) -> Json<Result<(), Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::presence::update(
        &db,
        &presence.presenter,
        presence.date,
        &presence,
    ))
}

#[delete("/presence/delete/<account>/<date>")]
async fn delete_presence(account: &str, date: &str) -> Json<Result<(), Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    let date = match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        Ok(_) => NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap(),
        Err(_) => {
            return Json(Err(Error::InvalidDate));
        }
    };
    Json(db::presence::delete(&db, account, date))
}

#[launch]
fn rocket() -> _ {
    let path = Path::new("./pdm.db");
    match Database::open(Cow::from(path)) {
        Ok(_) => Database::open(Cow::from(path)).unwrap().0,
        Err(_) => {
            let db = Database::create(Cow::from(path)).unwrap();
            db::project::create(&db).unwrap();
            fetch_user_data(&db, Cow::from(Path::new("./benutzer.txt")), "|").unwrap();
            db
        }
    };
    rocket::build().mount(
        "/",
        routes![
            all_users,
            fetch_user,
            search_user,
            add_user,
            update_user,
            delete_user,
            all_presences,
            fetch_presence,
            search_presence,
            add_presence,
            update_presence,
            delete_presence
        ],
    )
}
