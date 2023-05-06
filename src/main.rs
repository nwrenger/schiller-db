#[macro_use]
extern crate rocket;

pub mod db;

use std::{borrow::Cow, path::Path};

use chrono::NaiveDate;

use db::project::{fetch_user_data, Database, Error, Presence, User};
use db::stats::Stats;

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

struct ApiKey<'r>(&'r str);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if `key` is a valid API key string.
        fn is_valid(key: &str) -> bool {
            key == "test"
        }

        match req.headers().get_one("x-api-key") {
            None => Outcome::Failure((Status::BadRequest, Error::MissingApiKey)),
            Some(key) if is_valid(key) => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Failure((Status::BadRequest, Error::InvalidApiKey)),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Info {
    status: String,
    message: String,
    source: String,
    developer_team: Vec<String>,
}

#[get("/info")]
fn info() -> Json<Result<Info, Error>> {
    Json::from(Ok(Info {
        status: "Up and Running!".into(),
        message: "Welcome to the PDM!".into(),
        source: "https://github.com/NWrenger/pdm".into(),
        developer_team: vec!["Leonard Böttcher".into(), "Nils Wrenger".into()]
    }))
}

#[get("/stats")]
async fn stats(_key: ApiKey<'_>) -> Json<Result<Stats, Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::stats::fetch(&db))
}

#[get("/user/all")]
async fn all_users(_key: ApiKey<'_>) -> Json<Result<Vec<User>, Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::user::search(&db, ""))
}

#[get("/user/fetch/<id>")]
async fn fetch_user(_key: ApiKey<'_>, id: &str) -> Json<Result<User, Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::user::fetch(&db, id))
}

#[get("/user/search/<text>")]
async fn search_user(_key: ApiKey<'_>, text: &str) -> Json<Result<Vec<User>, Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::user::search(&db, text))
}

#[post("/user", format = "json", data = "<user>")]
async fn add_user(_key: ApiKey<'_>, user: Json<User>) -> Json<Result<(), Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::user::add(&db, &user))
}

#[put("/user", format = "json", data = "<user>")]
async fn update_user(_key: ApiKey<'_>, user: Json<User>) -> Json<Result<(), Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::user::update(&db, &user.account, &user))
}

#[delete("/user/<id>")]
async fn delete_user(_key: ApiKey<'_>, id: &str) -> Json<Result<(), Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::user::delete(&db, id))
}

#[get("/presence/all")]
async fn all_presences(_key: ApiKey<'_>) -> Json<Result<Vec<Presence>, Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::presence::search(&db, ""))
}

#[get("/presence/fetch/<account>/<date>")]
async fn fetch_presence(
    _key: ApiKey<'_>,
    account: &str,
    date: &str,
) -> Json<Result<Presence, Error>> {
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
async fn search_presence(_key: ApiKey<'_>, text: &str) -> Json<Result<Vec<Presence>, Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::presence::search(&db, text))
}

#[post("/presence", format = "json", data = "<presence>")]
async fn add_presence(_key: ApiKey<'_>, presence: Json<Presence>) -> Json<Result<(), Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::presence::add(&db, &presence))
}

#[put("/presence", format = "json", data = "<presence>")]
async fn update_presence(_key: ApiKey<'_>, presence: Json<Presence>) -> Json<Result<(), Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::presence::update(
        &db,
        &presence.presenter,
        presence.date,
        &presence,
    ))
}

#[delete("/presence/<account>/<date>")]
async fn delete_presence(_key: ApiKey<'_>, account: &str, date: &str) -> Json<Result<(), Error>> {
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
            info,
            stats,
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
