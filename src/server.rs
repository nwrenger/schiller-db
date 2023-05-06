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

use db::project::{Database, Error, Presence, User};
use db::stats::Stats;

/// Todo operation error.
#[derive(Serialize, ToSchema, Responder, Debug)]
pub enum ServerError {
    /// When unauthorized to complete operation
    #[response(status = 401)]
    Unauthorized(String),
}

pub struct ServerApiKey;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ServerApiKey {
    type Error = ServerError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request.headers().get("server_api_key").next() {
            Some(key) if key == "test" => Outcome::Success(ServerApiKey),
            None => Outcome::Failure((
                Status::Unauthorized,
                ServerError::Unauthorized(String::from("missing api key")),
            )),
            _ => Outcome::Failure((
                Status::Unauthorized,
                ServerError::Unauthorized(String::from("invalid api key")),
            )),
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Info {
    status: String,
    message: String,
    source: String,
    developer_team: Vec<String>,
}

#[utoipa::path(
    context_path = "",
    responses(
        (status = 200, description = "Got Infos", body = Info)
    )
)]
#[get("/info")]
pub async fn info() -> Json<Result<Info, Error>> {
    Json::from(Ok(Info {
        status: "Up and Running!".into(),
        message: "Welcome to the PDM!".into(),
        source: "https://github.com/NWrenger/pdm".into(),
        developer_team: vec!["Leonard Böttcher".into(), "Nils Wrenger".into()],
    }))
}

#[utoipa::path(
    context_path = "",
    responses(
        (status = 200, description = "Got Stats", body = Stats),
        (status = 401, description = "Unauthorized to view Stats", body = ServerError, example = json!(ServerError::Unauthorized(String::from("id = 1")))),
    ),
    security (
        ("api_key" = [])
    )
)]
#[get("/stats")]
pub async fn stats(_api_key: ServerApiKey) -> Json<Result<Stats, Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::stats::fetch(&db))
}

#[utoipa::path(
    context_path = "",
    responses(
        (status = 200, description = "Got all Users", body = User),
        (status = 401, description = "Unauthorized to view all Users", body = ServerError, example = json!(ServerError::Unauthorized(String::from("id = 1")))),
    ),
    security (
        ("api_key" = [])
    )
)]
#[get("/user/all")]
pub async fn all_users(_api_key: ServerApiKey) -> Json<Result<Vec<User>, Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::user::search(&db, ""))
}

#[utoipa::path(
    context_path = "",
    responses(
        (status = 200, description = "Got a User by a specific id", body = User),
        (status = 401, description = "Unauthorized to fetch a User", body = ServerError, example = json!(ServerError::Unauthorized(String::from("id = 1")))),
    ),
    params(
        ("id", description = "The unique user id")
    ),
    security (
        ("api_key" = [])
    )
)]
#[get("/user/fetch/<id>")]
pub async fn fetch_user(_api_key: ServerApiKey, id: &str) -> Json<Result<User, Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::user::fetch(&db, id))
}

#[utoipa::path(
    context_path = "",
    responses(
        (status = 200, description = "Searched all Users", body = User),
        (status = 401, description = "Unauthorized to search all Users", body = ServerError, example = json!(ServerError::Unauthorized(String::from("id = 1")))),
    ),
    params(
        ("text", description = "The search text")
    ),
    security (
        ("api_key" = [])
    )
)]
#[get("/user/search/<text>")]
pub async fn search_user(_api_key: ServerApiKey, text: &str) -> Json<Result<Vec<User>, Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::user::search(&db, text))
}

#[utoipa::path(
    context_path = "",
    request_body = User,
    responses(
        (status = 200, description = "Add a User sended successfully", body = User),
        (status = 401, description = "Unauthorized to add a User", body = ServerError, example = json!(ServerError::Unauthorized(String::from("id = 1")))),
    ),
    security (
        ("api_key" = [])
    )
)]
#[post("/user", format = "json", data = "<user>")]
pub async fn add_user(_api_key: ServerApiKey, user: Json<User>) -> Json<Result<(), Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::user::add(&db, &user))
}

#[utoipa::path(
    context_path = "",
    request_body = User,
    responses(
        (status = 200, description = "Update a User sended successfully", body = User),
        (status = 401, description = "Unauthorized to update a User", body = ServerError, example = json!(ServerError::Unauthorized(String::from("id = 1")))),
    ),
    security (
        ("api_key" = [])
    )
)]
#[put("/user", format = "json", data = "<user>")]
pub async fn update_user(_api_key: ServerApiKey, user: Json<User>) -> Json<Result<(), Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::user::update(&db, &user.account, &user))
}

#[utoipa::path(
    context_path = "",
    responses(
        (status = 200, description = "User delete sended successfully"),
        (status = 401, description = "Unauthorized to delete Users", body = ServerError, example = json!(ServerError::Unauthorized(String::from("id = 1")))),
    ),
    params(
        ("id", description = "The unique user id")
    ),
    security(
        ("api_key" = [])
    )
)]
#[delete("/user/<id>")]
pub async fn delete_user(_api_key: ServerApiKey, id: &str) -> Json<Result<(), Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::user::delete(&db, id))
}

#[utoipa::path(
    context_path = "",
    responses(
        (status = 200, description = "Got all Presences", body = Presence),
        (status = 401, description = "Unauthorized to view all Presences", body = ServerError, example = json!(ServerError::Unauthorized(String::from("id = 1")))),
    ),
    security (
        ("api_key" = [])
    )
)]
#[get("/presence/all")]
pub async fn all_presences(_api_key: ServerApiKey) -> Json<Result<Vec<Presence>, Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::presence::search(&db, ""))
}

#[utoipa::path(
    context_path = "",
    responses(
        (status = 200, description = "Got a Presence by a specific account and date", body = Presence),
        (status = 401, description = "Unauthorized to fetch a Presence", body = ServerError, example = json!(ServerError::Unauthorized(String::from("id = 1")))),
    ),
    params(
        ("account", description = "The unique user account"),
        ("date", description = "The date")
    ),
    security (
        ("api_key" = [])
    )
)]
#[get("/presence/fetch/<account>/<date>")]
pub async fn fetch_presence(
    _api_key: ServerApiKey,
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

#[utoipa::path(
    context_path = "",
    responses(
        (status = 200, description = "Searched all Presences", body = Presence),
        (status = 401, description = "Unauthorized to search all Presences", body = ServerError, example = json!(ServerError::Unauthorized(String::from("id = 1")))),
    ),
    params(
        ("text", description = "The search text")
    ),
    security (
        ("api_key" = [])
    )
)]
#[get("/presence/search/<text>")]
pub async fn search_presence(
    _api_key: ServerApiKey,
    text: &str,
) -> Json<Result<Vec<Presence>, Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::presence::search(&db, text))
}

#[utoipa::path(
    context_path = "",
    request_body = Presence,
    responses(
        (status = 200, description = "Add a presence sended successfully", body = Presence),
        (status = 401, description = "Unauthorized to add a Presence", body = ServerError, example = json!(ServerError::Unauthorized(String::from("id = 1")))),
    ),
    security (
        ("api_key" = [])
    )
)]
#[post("/presence", format = "json", data = "<presence>")]
pub async fn add_presence(
    _api_key: ServerApiKey,
    presence: Json<Presence>,
) -> Json<Result<(), Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::presence::add(&db, &presence))
}

#[utoipa::path(
    context_path = "",
    request_body = Presence,
    responses(
        (status = 200, description = "Update a Presence sended successfully", body = Presence),
        (status = 401, description = "Unauthorized to update a Presence", body = ServerError, example = json!(ServerError::Unauthorized(String::from("id = 1")))),
    ),
    security (
        ("api_key" = [])
    )
)]
#[put("/presence", format = "json", data = "<presence>")]
pub async fn update_presence(
    _api_key: ServerApiKey,
    presence: Json<Presence>,
) -> Json<Result<(), Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    Json(db::presence::update(
        &db,
        &presence.presenter,
        presence.date,
        &presence,
    ))
}

#[utoipa::path(
    context_path = "",
    responses(
        (status = 200, description = "Presence delete sended successfully"),
        (status = 401, description = "Unauthorized to delete Presences", body = ServerError, example = json!(ServerError::Unauthorized(String::from("id = 1")))),
    ),
    params(
        ("account", description = "The unique user account"),
        ("date", description = "The date")
    ),
    security(
        ("api_key" = [])
    )
)]
#[delete("/presence/<account>/<date>")]
pub async fn delete_presence(
    _api_key: ServerApiKey,
    account: &str,
    date: &str,
) -> Json<Result<(), Error>> {
    let db = Database::open(Cow::from(Path::new("./pdm.db"))).unwrap().0;
    let date = match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        Ok(_) => NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap(),
        Err(_) => {
            return Json(Err(Error::InvalidDate));
        }
    };
    Json(db::presence::delete(&db, account, date))
}
