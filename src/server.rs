use rocket::{
    delete, get,
    http::Status,
    outcome::Outcome,
    post, put,
    request::{self, FromRequest},
    serde::json::Json,
    Request,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use std::{borrow::Cow, env, path::Path};

use crate::db;
use chrono::NaiveDate;

use db::project::{Absence, Criminal, Database, Error, Result, User};
use db::stats::Stats;

pub struct GeneralApiKey;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GeneralApiKey {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let key_e: String = env::var("SNDM_KEY_E").unwrap_or("".to_string());
        let key_p: String = env::var("SNDM_KEY_P").unwrap_or("".to_string());
        let key_a: String = env::var("SNDM_KEY_A").unwrap_or("".to_string());
        match request.headers().get("server_api_key").next() {
            Some(key) if key == key_e || key == key_p || key == key_a => {
                Outcome::Success(GeneralApiKey)
            }
            _ => Outcome::Failure((Status::Unauthorized, Error::Unauthorized)),
        }
    }
}

pub struct AdminApiKey;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminApiKey {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let key_a: String = env::var("SNDM_KEY_A").unwrap_or("".to_string());
        match request.headers().get("server_api_key").next() {
            Some(key) if key == key_a => Outcome::Success(AdminApiKey),
            _ => Outcome::Failure((Status::Unauthorized, Error::Unauthorized)),
        }
    }
}

pub struct WriteApiKey;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for WriteApiKey {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let key_w: String = env::var("SNDM_KEY_W").unwrap_or("".to_string());
        let key_a: String = env::var("SNDM_KEY_A").unwrap_or("".to_string());
        match request.headers().get("write_api_key").next() {
            Some(key) if key == key_w || key == key_a => Outcome::Success(WriteApiKey),
            _ => Outcome::Failure((Status::Unauthorized, Error::Unauthorized)),
        }
    }
}

pub struct EmploymentApiKey;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for EmploymentApiKey {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let key_e: String = env::var("SNDM_KEY_E").unwrap_or("".to_string());
        let key_a: String = env::var("SNDM_KEY_A").unwrap_or("".to_string());
        match request.headers().get("server_api_key").next() {
            Some(key) if key == key_e || key == key_a => Outcome::Success(EmploymentApiKey),
            _ => Outcome::Failure((Status::Unauthorized, Error::Unauthorized)),
        }
    }
}

pub struct PoliceApiKey;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for PoliceApiKey {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let key_p: String = env::var("SNDM_KEY_E").unwrap_or("".to_string());
        let key_a: String = env::var("SNDM_KEY_A").unwrap_or("".to_string());
        match request.headers().get("server_api_key").next() {
            Some(key) if key == key_p || key == key_a => Outcome::Success(PoliceApiKey),
            _ => Outcome::Failure((Status::Unauthorized, Error::Unauthorized)),
        }
    }
}

/// Data object for Infos.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct Info {
    status: String,
    message: String,
    source: String,
    developer_team: Vec<String>,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Got Infos", body = Info),
        (status = 500, description = "Something internally went wrong", body = Error, example = json!({"Err": Error::InternalError})),
    )
)]
#[get("/info")]
pub async fn info() -> Json<Info> {
    Json(Info {
        status: "Up and Running!".into(),
        message: "Welcome to the sndm!".into(),
        source: "https://github.com/nwrenger/sndm".into(),
        developer_team: vec!["Nils Wrenger".into(), "Leonard Böttcher".into()],
    })
}

#[utoipa::path(
    responses(
        (status = 200, description = "Got Stats", body = Stats),
        (status = 401, description = "Unauthorized to view Stats", body = Error, example = json!({"Err": Error::Unauthorized})),
        (status = 500, description = "Something internally went wrong", body = Error, example = json!({"Err": Error::InternalError})),
    ),
    security (
        ("server_api_key" = [])
    )
)]
#[get("/stats")]
pub async fn stats(_api_key_1: PoliceApiKey) -> Json<Result<Stats>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::stats::fetch(&db))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Got a User by a specific id", body = User),
        (status = 401, description = "Unauthorized to fetch a User", body = Error, example = json!({"Err": Error::Unauthorized})),
        (status = 500, description = "Something internally went wrong", body = Error, example = json!({"Err": Error::InternalError})),
    ),
    params(
        ("id", description = "The unique user id")
    ),
    security (
        ("server_api_key" = [])
    )
)]
#[get("/user/fetch/<id>")]
pub async fn fetch_user(_api_key: GeneralApiKey, id: &str) -> Json<Result<User>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::user::fetch(&db, id))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Searched all Users", body = Vec<User>),
        (status = 401, description = "Unauthorized to search all Users", body = Error, example = json!({"Err": Error::Unauthorized})),
        (status = 500, description = "Something internally went wrong", body = Error, example = json!({"Err": Error::InternalError})),
    ),
    security (
        ("server_api_key" = [])
    )
)]
#[get("/user/search?<text>")]
pub async fn search_user(_api_key: GeneralApiKey, text: Option<&str>) -> Json<Result<Vec<User>>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::user::search(&db, text.unwrap_or_default()))
}

#[utoipa::path(
    request_body = User,
    responses(
        (status = 200, description = "Add a User sended successfully"),
        (status = 401, description = "Unauthorized to add a User", body = Error, example = json!({"Err": Error::Unauthorized})),
        (status = 422, description = "The Json is parsed in a wrong format", body = Error, example = json!({"Err": Error::UnprocessableEntity})),
        (status = 500, description = "Something internally went wrong", body = Error, example = json!({"Err": Error::InternalError})),
    ),
    security (
        ("server_api_key" = []),
        ("write_api_key" = [])
    )
)]
#[post("/user", format = "json", data = "<user>")]
pub async fn add_user(
    _api_key: AdminApiKey,
    _api_key_write: WriteApiKey,
    user: Json<User>,
) -> Json<Result<()>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::user::add(&db, &user))
}

#[utoipa::path(
    request_body = User,
    responses(
        (status = 200, description = "Update a User sended successfully"),
        (status = 401, description = "Unauthorized to update a User", body = Error, example = json!({"Err": Error::Unauthorized})),
        (status = 422, description = "The Json is parsed in a wrong format", body = Error, example = json!({"Err": Error::UnprocessableEntity})),
        (status = 500, description = "Something internally went wrong", body = Error, example = json!({"Err": Error::InternalError})),
    ),
    security (
        ("server_api_key" = []),
        ("write_api_key" = [])
    )
)]
#[put("/user", format = "json", data = "<user>")]
pub async fn update_user(
    _api_key: AdminApiKey,
    _api_key_write: WriteApiKey,
    user: Json<User>,
) -> Json<Result<()>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::user::update(&db, &user.account, &user))
}

#[utoipa::path(
    responses(
        (status = 200, description = "User delete sended successfully"),
        (status = 401, description = "Unauthorized to delete Users", body = Error, example = json!({"Err": Error::Unauthorized})),
        (status = 500, description = "Something internally went wrong", body = Error, example = json!({"Err": Error::InternalError})),
    ),
    params(
        ("id", description = "The unique user id")
    ),
    security(
        ("server_api_key" = []),
        ("write_api_key" = [])
    )
)]
#[delete("/user/<id>")]
pub async fn delete_user(
    _api_key: AdminApiKey,
    _api_key_write: WriteApiKey,
    id: &str,
) -> Json<Result<()>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::user::delete(&db, id))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Got an Absence by a specific account and date", body = Absence),
        (status = 401, description = "Unauthorized to fetch an Absence", body = Error, example = json!({"Err": Error::Unauthorized})),
        (status = 500, description = "Something internally went wrong", body = Error, example = json!({"Err": Error::InternalError})),
    ),
    params(
        ("account", description = "The unique user account"),
        ("date", description = "The date")
    ),
    security (
        ("server_api_key" = [])
    )
)]
#[get("/absence/fetch/<account>/<date>")]
pub async fn fetch_absence(
    _api_key: EmploymentApiKey,
    account: &str,
    date: &str,
) -> Json<Result<Absence>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    let date = match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            return Json(Err(Error::InvalidDate));
        }
    };
    Json(db::absence::fetch(&db, account, date))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Searched all Absences", body = Vec<Absence>),
        (status = 401, description = "Unauthorized to search all Absences", body = Error, example = json!({"Err": Error::Unauthorized})),
        (status = 500, description = "Something internally went wrong", body = Error, example = json!({"Err": Error::InternalError})),
    ),
    security (
        ("server_api_key" = [])
    )
)]
#[get("/absence/search?<text>")]
pub async fn search_absence(
    _api_key: EmploymentApiKey,
    text: Option<&str>,
) -> Json<Result<Vec<Absence>>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::absence::search(&db, text.unwrap_or_default()))
}

#[utoipa::path(
    request_body = Absence,
    responses(
        (status = 200, description = "Add an Absence sended successfully"),
        (status = 401, description = "Unauthorized to add a Absence", body = Error, example = json!({"Err": Error::Unauthorized})),
        (status = 422, description = "The Json is parsed in a wrong format", body = Error, example = json!({"Err": Error::UnprocessableEntity})),
        (status = 500, description = "Something internally went wrong", body = Error, example = json!({"Err": Error::InternalError})),
    ),
    security (
        ("server_api_key" = []),
        ("write_api_key" = [])
    )
)]
#[post("/absence", format = "json", data = "<absence>")]
pub async fn add_absence(
    _api_key: EmploymentApiKey,
    _api_key_write: WriteApiKey,
    absence: Json<Absence>,
) -> Json<Result<()>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::absence::add(&db, &absence))
}

#[utoipa::path(
    request_body = Absence,
    responses(
        (status = 200, description = "Update an Absence sended successfully"),
        (status = 401, description = "Unauthorized to update an Absence", body = Error, example = json!({"Err": Error::Unauthorized})),
        (status = 422, description = "The Json is parsed in a wrong format", body = Error, example = json!({"Err": Error::UnprocessableEntity})),
        (status = 500, description = "Something internally went wrong", body = Error, example = json!({"Err": Error::InternalError})),
    ),
    security (
        ("server_api_key" = []),
        ("write_api_key" = [])
    )
)]
#[put("/absence", format = "json", data = "<absence>")]
pub async fn update_absence(
    _api_key: EmploymentApiKey,
    _api_key_write: WriteApiKey,
    absence: Json<Absence>,
) -> Json<Result<()>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::absence::update(
        &db,
        &absence.account,
        absence.date,
        &absence,
    ))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Absence delete sended successfully"),
        (status = 401, description = "Unauthorized to delete Absences", body = Error, example = json!({"Err": Error::Unauthorized})),
        (status = 500, description = "Something internally went wrong", body = Error, example = json!({"Err": Error::InternalError})),
    ),
    params(
        ("account", description = "The unique user account"),
        ("date", description = "The date")
    ),
    security(
        ("server_api_key" = []),
        ("write_api_key" = [])
    )
)]
#[delete("/absence/<account>/<date>")]
pub async fn delete_absence(
    _api_key: EmploymentApiKey,
    _api_key_write: WriteApiKey,
    account: &str,
    date: &str,
) -> Json<Result<()>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    let date = match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            return Json(Err(Error::InvalidDate));
        }
    };
    Json(db::absence::delete(&db, account, date))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Got a Criminal by a specific account", body = Criminal),
        (status = 401, description = "Unauthorized to fetch a Criminal", body = Error, example = json!({"Err": Error::Unauthorized})),
        (status = 500, description = "Something internally went wrong", body = Error, example = json!({"Err": Error::InternalError})),
    ),
    params(
        ("account", description = "The unique user account"),
    ),
    security (
        ("server_api_key" = [])
    )
)]
#[get("/criminal/fetch/<account>")]
pub async fn fetch_criminal(_api_key: PoliceApiKey, account: &str) -> Json<Result<Criminal>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::criminal::fetch(&db, account))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Searched all Criminals", body = Vec<Criminal>),
        (status = 401, description = "Unauthorized to search all Criminals", body = Error, example = json!({"Err": Error::Unauthorized})),
        (status = 500, description = "Something internally went wrong", body = Error, example = json!({"Err": Error::InternalError})),
    ),
    security (
        ("server_api_key" = [])
    )
)]
#[get("/criminal/search?<text>")]
pub async fn search_criminal(
    _api_key: PoliceApiKey,
    text: Option<&str>,
) -> Json<Result<Vec<Criminal>>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::criminal::search(&db, text.unwrap_or_default()))
}

#[utoipa::path(
    request_body = Criminal,
    responses(
        (status = 200, description = "Add a criminal sended successfully"),
        (status = 401, description = "Unauthorized to add a Crimials", body = Error, example = json!({"Err": Error::Unauthorized})),
        (status = 422, description = "The Json is parsed in a wrong format", body = Error, example = json!({"Err": Error::UnprocessableEntity})),
        (status = 500, description = "Something internally went wrong", body = Error, example = json!({"Err": Error::InternalError})),
    ),
    security (
        ("server_api_key" = []),
        ("write_api_key" = [])
    )
)]
#[post("/criminal", format = "json", data = "<criminal>")]
pub async fn add_criminal(
    _api_key: PoliceApiKey,
    _api_key_write: WriteApiKey,
    criminal: Json<Criminal>,
) -> Json<Result<()>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::criminal::add(&db, &criminal))
}

#[utoipa::path(
    request_body = Criminal,
    responses(
        (status = 200, description = "Update a absence sended successfully"),
        (status = 401, description = "Unauthorized to update a absence", body = Error, example = json!({"Err": Error::Unauthorized})),
        (status = 422, description = "The Json is parsed in a wrong format", body = Error, example = json!({"Err": Error::UnprocessableEntity})),
        (status = 500, description = "Something internally went wrong", body = Error, example = json!({"Err": Error::InternalError})),
    ),
    security (
        ("server_api_key" = []),
        ("write_api_key" = [])
    )
)]
#[put("/criminal", format = "json", data = "<criminal>")]
pub async fn update_criminal(
    _api_key: PoliceApiKey,
    _api_key_write: WriteApiKey,
    criminal: Json<Criminal>,
) -> Json<Result<()>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::criminal::update(&db, &criminal.account, &criminal))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Criminal delete sended successfully"),
        (status = 401, description = "Unauthorized to delete Criminal", body = Error, example = json!({"Err": Error::Unauthorized})),
        (status = 500, description = "Something internally went wrong", body = Error, example = json!({"Err": Error::InternalError})),
    ),
    params(
        ("account", description = "The unique user account"),
    ),
    security(
        ("server_api_key" = []),
        ("write_api_key" = [])
    )
)]
#[delete("/criminal/<account>")]
pub async fn delete_criminal(
    _api_key: PoliceApiKey,
    _api_key_write: WriteApiKey,
    account: &str,
) -> Json<Result<()>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::criminal::delete(&db, account))
}
