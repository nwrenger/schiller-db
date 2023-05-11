use rocket::{
    delete, get,
    http::Status,
    outcome::Outcome,
    post, put,
    request::{self, FromRequest},
    response::Responder,
    serde::json::Json,
    Request,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use std::{borrow::Cow, path::Path};

use crate::db;
use chrono::NaiveDate;

use db::project::{Database, Error, Presence, Result, User, Criminal};
use db::stats::Stats;

/// Server operation error.
#[derive(Serialize, ToSchema, Responder, Debug)]
pub enum ServerError {
    /// When unauthorized to complete operation
    #[response(status = 401)]
    Unauthorized(String),

    ///When a wrong format is used
    #[response(status = 422)]
    UnprocessableEntity(String),
}

pub struct GeneralApiKey;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GeneralApiKey {
    type Error = ServerError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request.headers().get("server_api_key").next() {
            Some(key) if key == "e" || key == "p" => Outcome::Success(GeneralApiKey),
            None => Outcome::Failure((
                Status::Unauthorized,
                ServerError::Unauthorized(String::from("missing api key")),
            )),
            _ => Outcome::Failure((
                Status::Unauthorized,
                ServerError::Unauthorized(String::from(
                    "invalid api key/missing permissions",
                )),
            )),
        }
    }
}

pub struct WriteApiKey;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for WriteApiKey {
    type Error = ServerError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request.headers().get("write_api_key").next() {
            Some(key) if key == "w" => Outcome::Success(WriteApiKey),
            None => Outcome::Failure((
                Status::Unauthorized,
                ServerError::Unauthorized(String::from("missing api key")),
            )),
            _ => Outcome::Failure((
                Status::Unauthorized,
                ServerError::Unauthorized(String::from(
                    "invalid api key/missing permissions for writing data",
                )),
            )),
        }
    }
}

pub struct EmploymentApiKey;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for EmploymentApiKey {
    type Error = ServerError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request.headers().get("server_api_key").next() {
            Some(key) if key == "e" => Outcome::Success(EmploymentApiKey),
            None => Outcome::Failure((
                Status::Unauthorized,
                ServerError::Unauthorized(String::from("missing api key")),
            )),
            _ => Outcome::Failure((
                Status::Unauthorized,
                ServerError::Unauthorized(String::from(
                    "invalid api key/missing permissions for criminals",
                )),
            )),
        }
    }
}

pub struct PoliceApiKey;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for PoliceApiKey {
    type Error = ServerError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request.headers().get("server_api_key").next() {
            Some(key) if key == "p" => Outcome::Success(PoliceApiKey),
            None => Outcome::Failure((
                Status::Unauthorized,
                ServerError::Unauthorized(String::from("missing api key")),
            )),
            _ => Outcome::Failure((
                Status::Unauthorized,
                ServerError::Unauthorized(String::from(
                    "invalid api key/missing permissions for presences",
                )),
            )),
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
        (status = 200, description = "Got Infos", body = Info)
    )
)]
#[get("/info")]
pub async fn info() -> Json<Info> {
    Json::from(Info {
        status: "Up and Running!".into(),
        message: "Welcome to the sndm!".into(),
        source: "https://github.com/NWrenger/sndm".into(),
        developer_team: vec!["Nils Wrenger".into(), "Leonard Böttcher".into()],
    })
}

#[utoipa::path(
    responses(
        (status = 200, description = "Got Stats", body = Stats),
        (status = 401, description = "Unauthorized to view Stats", body = ServerError),
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
        (status = 401, description = "Unauthorized to fetch a User", body = ServerError),
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
        (status = 401, description = "Unauthorized to search all Users", body = ServerError),
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
        (status = 401, description = "Unauthorized to add a User", body = ServerError, example = json!(ServerError::Unauthorized("string".into()))),
        (status = 422, description = "The Json is parsed in a wrong format", body = ServerError, example = json!(ServerError::UnprocessableEntity("string".into()))),
    ),
    security (
        ("server_api_key" = []),
        ("write_api_key" = [])
    )
)]
#[post("/user", format = "json", data = "<user>")]
pub async fn add_user(_api_key: GeneralApiKey, _api_key_write: WriteApiKey, user: Json<User>) -> Json<Result<()>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::user::add(&db, &user))
}

#[utoipa::path(
    request_body = User,
    responses(
        (status = 200, description = "Update a User sended successfully"),
        (status = 401, description = "Unauthorized to update a User", body = ServerError, example = json!(ServerError::Unauthorized("string".into()))),
        (status = 422, description = "The Json is parsed in a wrong format", body = ServerError, example = json!(ServerError::UnprocessableEntity("string".into()))),
    ),
    security (
        ("server_api_key" = []),
        ("write_api_key" = [])
    )
)]
#[put("/user", format = "json", data = "<user>")]
pub async fn update_user(_api_key: GeneralApiKey, _api_key_write: WriteApiKey, user: Json<User>) -> Json<Result<()>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::user::update(&db, &user.account, &user))
}

#[utoipa::path(
    responses(
        (status = 200, description = "User delete sended successfully"),
        (status = 401, description = "Unauthorized to delete Users", body = ServerError),
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
pub async fn delete_user(_api_key: GeneralApiKey, _api_key_write: WriteApiKey, id: &str) -> Json<Result<()>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::user::delete(&db, id))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Got a Presence by a specific account and date", body = Presence),
        (status = 401, description = "Unauthorized to fetch a Presence", body = ServerError),
    ),
    params(
        ("account", description = "The unique user account"),
        ("date", description = "The date")
    ),
    security (
        ("server_api_key" = [])
    )
)]
#[get("/presence/fetch/<account>/<date>")]
pub async fn fetch_presence(
    _api_key: EmploymentApiKey,
    account: &str,
    date: &str,
) -> Json<Result<Presence>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    let date = match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        Ok(_) => NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap(),
        Err(_) => {
            return Json(Err(Error::InvalidDate));
        }
    };
    Json(db::presence::fetch(&db, account, date))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Searched all Presences", body = Vec<Presence>),
        (status = 401, description = "Unauthorized to search all Presences", body = ServerError),
    ),
    security (
        ("server_api_key" = [])
    )
)]
#[get("/presence/search?<text>")]
pub async fn search_presence(
    _api_key: EmploymentApiKey,
    text: Option<&str>,
) -> Json<Result<Vec<Presence>>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::presence::search(&db, text.unwrap_or_default()))
}

#[utoipa::path(
    request_body = Presence,
    responses(
        (status = 200, description = "Add a presence sended successfully"),
        (status = 401, description = "Unauthorized to add a Presence", body = ServerError, example = json!(ServerError::Unauthorized("string".into()))),
        (status = 422, description = "The Json is parsed in a wrong format", body = ServerError, example = json!(ServerError::UnprocessableEntity("string".into()))),
    ),
    security (
        ("server_api_key" = []),
        ("write_api_key" = [])
    )
)]
#[post("/presence", format = "json", data = "<presence>")]
pub async fn add_presence(
    _api_key: EmploymentApiKey,
    _api_key_write: WriteApiKey,
    presence: Json<Presence>,
) -> Json<Result<()>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::presence::add(&db, &presence))
}

#[utoipa::path(
    request_body = Presence,
    responses(
        (status = 200, description = "Update a Presence sended successfully"),
        (status = 401, description = "Unauthorized to update a Presence", body = ServerError, example = json!(ServerError::Unauthorized("string".into()))),
        (status = 422, description = "The Json is parsed in a wrong format", body = ServerError, example = json!(ServerError::UnprocessableEntity("string".into()))),
    ),
    security (
        ("server_api_key" = []),
        ("write_api_key" = [])
    )
)]
#[put("/presence", format = "json", data = "<presence>")]
pub async fn update_presence(
    _api_key: EmploymentApiKey,
    _api_key_write: WriteApiKey,
    presence: Json<Presence>,
) -> Json<Result<()>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::presence::update(
        &db,
        &presence.presenter,
        presence.date,
        &presence,
    ))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Presence delete sended successfully"),
        (status = 401, description = "Unauthorized to delete Presences", body = ServerError),
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
#[delete("/presence/<account>/<date>")]
pub async fn delete_presence(
    _api_key: EmploymentApiKey,
    _api_key_write: WriteApiKey,
    account: &str,
    date: &str,
) -> Json<Result<()>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    let date = match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        Ok(_) => NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap(),
        Err(_) => {
            return Json(Err(Error::InvalidDate));
        }
    };
    Json(db::presence::delete(&db, account, date))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Got a Criminal by a specific account", body = Criminal),
        (status = 401, description = "Unauthorized to fetch a Criminal", body = ServerError),
    ),
    params(
        ("account", description = "The unique user account"),
    ),
    security (
        ("server_api_key" = [])
    )
)]
#[get("/criminal/fetch/<account>")]
pub async fn fetch_criminal(
    _api_key: PoliceApiKey,
    account: &str,
) -> Json<Result<Criminal>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::criminals::fetch(&db, account))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Searched all Criminals", body = Vec<Criminal>),
        (status = 401, description = "Unauthorized to search all Criminals", body = ServerError),
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
    Json(db::criminals::search(&db, text.unwrap_or_default()))
}

#[utoipa::path(
    request_body = Criminal,
    responses(
        (status = 200, description = "Add a criminal sended successfully"),
        (status = 401, description = "Unauthorized to add a Crimials", body = ServerError, example = json!(ServerError::Unauthorized("string".into()))),
        (status = 422, description = "The Json is parsed in a wrong format", body = ServerError, example = json!(ServerError::UnprocessableEntity("string".into()))),
    ),
    security (
        ("server_api_key" = []),
        ("write_api_key" = [])
    )
)]
#[post("/criminal", format = "json", data = "<criminals>")]
pub async fn add_criminal(
    _api_key: PoliceApiKey,
    _api_key_write: WriteApiKey,
    criminals: Json<Criminal>,
) -> Json<Result<()>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::criminals::add(&db, &criminals))
}

#[utoipa::path(
    request_body = Crimials,
    responses(
        (status = 200, description = "Update a Presence sended successfully"),
        (status = 401, description = "Unauthorized to update a Presence", body = ServerError, example = json!(ServerError::Unauthorized("string".into()))),
        (status = 422, description = "The Json is parsed in a wrong format", body = ServerError, example = json!(ServerError::UnprocessableEntity("string".into()))),
    ),
    security (
        ("server_api_key" = []),
        ("write_api_key" = [])
    )
)]
#[put("/criminal", format = "json", data = "<criminals>")]
pub async fn update_criminal(
    _api_key: PoliceApiKey,
    _api_key_write: WriteApiKey,
    criminals: Json<Criminal>,
) -> Json<Result<()>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::criminals::update(
        &db,
        &criminals.criminal,
        &criminals,
    ))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Criminal delete sended successfully"),
        (status = 401, description = "Unauthorized to delete Criminals", body = ServerError),
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
    Json(db::criminals::delete(&db, account))
}
