use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use log::warn;
use rocket::{
    delete, get,
    http::Status,
    outcome::Outcome,
    post, put,
    request::{self, FromRequest},
    serde::json::Json,
    Request, fs::NamedFile,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use std::{borrow::Cow, path::{Path, PathBuf}};
use std::{env, marker::PhantomData};

use crate::db::{self, user::UserSearch};
use chrono::NaiveDate;

use db::absence::Absence;
use db::criminal::Criminal;
use db::login::{Login, Permission};
use db::project::{Database, Error, Result};
use db::stats::Stats;
use db::user::User;

pub trait Access {
    fn check(l: Login) -> bool;
}
pub struct UserReadOnly {}
impl Access for UserReadOnly {
    fn check(l: Login) -> bool {
        matches!(l.access_user, Permission::ReadOnly | Permission::Write)
    }
}
pub struct UserWrite {}
impl Access for UserWrite {
    fn check(l: Login) -> bool {
        matches!(l.access_user, Permission::Write)
    }
}
pub struct AbsenceReadOnly {}
impl Access for AbsenceReadOnly {
    fn check(l: Login) -> bool {
        matches!(l.access_absence, Permission::ReadOnly | Permission::Write)
    }
}
pub struct AbsenceWrite {}
impl Access for AbsenceWrite {
    fn check(l: Login) -> bool {
        matches!(l.access_absence, Permission::Write)
    }
}
pub struct CriminalReadOnly {}
impl Access for CriminalReadOnly {
    fn check(l: Login) -> bool {
        matches!(l.access_criminal, Permission::ReadOnly | Permission::Write)
    }
}
pub struct CriminalWrite {}
impl Access for CriminalWrite {
    fn check(l: Login) -> bool {
        matches!(l.access_criminal, Permission::Write)
    }
}

pub struct Auth<P: Access> {
    pub user: String,
    pub _phantom: PhantomData<P>,
}

#[rocket::async_trait]
impl<'r, P: Access> FromRequest<'r> for Auth<P> {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // Parsing authentication header
        // e.g. "Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==" -> Aladdin:open sesame

        let header = request.headers().get("authorization").next();

        let Some(user_pass) = header
                .and_then(|v| v.strip_prefix("Basic "))
                .and_then(|v| BASE64.decode(v).ok())
                .and_then(|v| String::from_utf8(v).ok()) else {
            warn!("missing auth {header:?} from {:?}", request.client_ip());
            return Outcome::Failure((Status::Unauthorized, Error::Unauthorized));
        };
        let Some((user, password)) = user_pass.split_once(':') else {
            warn!("wrong auth header '{user_pass}' from {:?}", request.client_ip());
            return Outcome::Failure((Status::Unauthorized, Error::Unauthorized));
        };

        // lookup in database

        let Ok((db, _)) = Database::open(Cow::from(Path::new("./sndm.db"))) else {
            return Outcome::Failure((Status::Unauthorized, Error::Unauthorized));
        };
        let Ok(login) = db::login::fetch(&db, user, password) else {
            warn!("missing auth credentials '{user}:{password}' from {:?}", request.client_ip());
            return Outcome::Failure((Status::Unauthorized, Error::Unauthorized));
        };

        // checking permissions

        if P::check(login) {
            Outcome::Success(Self {
                user: user.into(),
                _phantom: PhantomData,
            })
        } else {
            warn!(
                "missing auth permissions '{user}:{password}' from {:?}",
                request.client_ip()
            );
            Outcome::Failure((Status::Unauthorized, Error::Unauthorized))
        }
    }
}

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    let path = Path::new("static").join("index.html");
    NamedFile::open(path).await.ok()
}

#[get("/<path..>")]
pub async fn static_files(path: PathBuf) -> Option<NamedFile> {
    let path = Path::new("static").join(path);
    NamedFile::open(path).await.ok()
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
    ),
    security (
        ("authorization" = []),
    )
)]
#[get("/stats")]
pub async fn stats(auth: Auth<CriminalReadOnly>) -> Json<Result<Stats>> {
    warn!("GET /stats: {}", auth.user);
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::stats::fetch(&db))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Got a User by a specific id", body = User),
        (status = 401, description = "Unauthorized to fetch a User", body = Error, example = json!({"Err": Error::Unauthorized})),
    ),
    params(
        ("id", description = "The unique user id")
    ),
    security (
        ("authorization" = []),
    )
)]
#[get("/user/fetch/<id>")]
pub async fn fetch_user(auth: Auth<UserReadOnly>, id: &str) -> Json<Result<User>> {
    warn!("GET /user/fetch/{id}: {}", auth.user);
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::user::fetch(&db, id))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Got all Roles", body = Vec<String>),
        (status = 401, description = "Unauthorized to get all Roles", body = Error, example = json!({"Err": Error::Unauthorized})),
    ),
    security (
        ("authorization" = []),
    )
)]
#[get("/user/all_roles")]
pub async fn all_roles(auth: Auth<UserReadOnly>) -> Json<Result<Vec<String>>> {
    warn!("GET /user/all_roles: {}", auth.user);
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::user::all_roles(&db))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Searched all Users", body = Vec<User>),
        (status = 401, description = "Unauthorized to search all Users", body = Error, example = json!({"Err": Error::Unauthorized})),
    ),
    security (
        ("authorization" = []),
    )
)]
#[get("/user/search?<name>&<role>&<offset>")]
pub async fn search_user(
    auth: Auth<UserReadOnly>,
    name: Option<&str>,
    role: Option<&str>,
    offset: Option<usize>,
) -> Json<Result<Vec<User>>> {
    warn!(
        "GET /user/search?{name:?}&{role:?}&{offset:?}: {}",
        auth.user
    );
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::user::search(
        &db,
        UserSearch::new(name.unwrap_or_default(), role.unwrap_or("%")),
        offset.unwrap_or_default(),
    ))
}

#[utoipa::path(
    request_body = User,
    responses(
        (status = 200, description = "Add a User sended successfully"),
        (status = 401, description = "Unauthorized to add a User", body = Error, example = json!({"Err": Error::Unauthorized})),
        (status = 422, description = "The Json is parsed in a wrong format", body = Error, example = json!({"Err": Error::UnprocessableEntity})),
    ),
    security (
        ("authorization" = []),
    )
)]
#[post("/user", format = "json", data = "<user>")]
pub async fn add_user(auth: Auth<UserWrite>, user: Json<User>) -> Json<Result<()>> {
    warn!("POST /user with data {user:?}: {}", auth.user);
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::user::add(&db, &user))
}

#[utoipa::path(
    request_body = User,
    responses(
        (status = 200, description = "Update a User sended successfully"),
        (status = 401, description = "Unauthorized to update a User", body = Error, example = json!({"Err": Error::Unauthorized})),
        (status = 422, description = "The Json is parsed in a wrong format", body = Error, example = json!({"Err": Error::UnprocessableEntity})),
    ),
    security (
        ("authorization" = []),
    )
)]
#[put("/user", format = "json", data = "<user>")]
pub async fn update_user(auth: Auth<UserWrite>, user: Json<User>) -> Json<Result<()>> {
    warn!("PUT /user with data {user:?}: {}", auth.user);
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::user::update(&db, &user.account, &user))
}

#[utoipa::path(
    responses(
        (status = 200, description = "User delete sended successfully"),
        (status = 401, description = "Unauthorized to delete Users", body = Error, example = json!({"Err": Error::Unauthorized})),
    ),
    params(
        ("id", description = "The unique user id")
    ),
    security(
        ("authorization" = []),
    )
)]
#[delete("/user/<id>")]
pub async fn delete_user(auth: Auth<UserWrite>, id: &str) -> Json<Result<()>> {
    warn!("DELETE /user/{id}: {}", auth.user);
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::user::delete(&db, id))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Got an Absence by a specific account and date", body = Absence),
        (status = 401, description = "Unauthorized to fetch an Absence", body = Error, example = json!({"Err": Error::Unauthorized})),
    ),
    params(
        ("account", description = "The unique user account"),
        ("date", description = "The date")
    ),
    security (
        ("authorization" = []),
    )
)]
#[get("/absence/fetch/<account>/<date>")]
pub async fn fetch_absence(
    auth: Auth<AbsenceReadOnly>,
    account: &str,
    date: &str,
) -> Json<Result<Absence>> {
    warn!("GET /absence/fetch/{account}/{date}: {}", auth.user);
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
    ),
    security (
        ("authorization" = []),
    )
)]
#[get("/absence/search?<text>")]
pub async fn search_absence(
    auth: Auth<AbsenceReadOnly>,
    text: Option<&str>,
) -> Json<Result<Vec<Absence>>> {
    warn!("GET /absence/search?{text:?}: {}", auth.user);
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::absence::search(&db, text.unwrap_or_default()))
}

#[utoipa::path(
    request_body = Absence,
    responses(
        (status = 200, description = "Add an Absence sended successfully"),
        (status = 401, description = "Unauthorized to add a Absence", body = Error, example = json!({"Err": Error::Unauthorized})),
        (status = 422, description = "The Json is parsed in a wrong format", body = Error, example = json!({"Err": Error::UnprocessableEntity})),
    ),
    security (
        ("authorization" = []),
    )
)]
#[post("/absence", format = "json", data = "<absence>")]
pub async fn add_absence(auth: Auth<AbsenceWrite>, absence: Json<Absence>) -> Json<Result<()>> {
    warn!("POST /absence with data {absence:?}: {}", auth.user);
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::absence::add(&db, &absence))
}

#[utoipa::path(
    request_body = Absence,
    responses(
        (status = 200, description = "Update an Absence sended successfully"),
        (status = 401, description = "Unauthorized to update an Absence", body = Error, example = json!({"Err": Error::Unauthorized})),
        (status = 422, description = "The Json is parsed in a wrong format", body = Error, example = json!({"Err": Error::UnprocessableEntity})),
    ),
    security (
        ("authorization" = []),
    )
)]
#[put("/absence", format = "json", data = "<absence>")]
pub async fn update_absence(auth: Auth<AbsenceWrite>, absence: Json<Absence>) -> Json<Result<()>> {
    warn!("PUT /absence with data {absence:?}: {}", auth.user);
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
    ),
    params(
        ("account", description = "The unique user account"),
        ("date", description = "The date")
    ),
    security(
        ("authorization" = []),
    )
)]
#[delete("/absence/<account>/<date>")]
pub async fn delete_absence(
    auth: Auth<AbsenceWrite>,
    account: &str,
    date: &str,
) -> Json<Result<()>> {
    warn!("DELETE /absence/{account}/{date}: {}", auth.user);
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
    ),
    params(
        ("account", description = "The unique user account"),
    ),
    security (
        ("authorization" = []),
    )
)]
#[get("/criminal/fetch/<account>")]
pub async fn fetch_criminal(auth: Auth<CriminalReadOnly>, account: &str) -> Json<Result<Criminal>> {
    warn!("GET /criminal/fetch/{account}: {}", auth.user);
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::criminal::fetch(&db, account))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Searched all criminals", body = Vec<Criminal>),
        (status = 401, description = "Unauthorized to search all criminals", body = Error, example = json!({"Err": Error::Unauthorized})),
    ),
    security (
        ("authorization" = []),
    )
)]
#[get("/criminal/search?<text>")]
pub async fn search_criminal(
    auth: Auth<CriminalReadOnly>,
    text: Option<&str>,
) -> Json<Result<Vec<Criminal>>> {
    warn!("GET /criminal/search?{text:?}: {}", auth.user);
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::criminal::search(&db, text.unwrap_or_default()))
}

#[utoipa::path(
    request_body = Criminal,
    responses(
        (status = 200, description = "Add a criminal sended successfully"),
        (status = 401, description = "Unauthorized to add a criminals", body = Error, example = json!({"Err": Error::Unauthorized})),
        (status = 422, description = "The Json is parsed in a wrong format", body = Error, example = json!({"Err": Error::UnprocessableEntity})),
    ),
    security (
        ("authorization" = []),
    )
)]
#[post("/criminal", format = "json", data = "<criminal>")]
pub async fn add_criminal(auth: Auth<CriminalWrite>, criminal: Json<Criminal>) -> Json<Result<()>> {
    warn!("POST /criminal with data {criminal:?}: {}", auth.user);
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::criminal::add(&db, &criminal))
}

#[utoipa::path(
    request_body = Criminal,
    responses(
        (status = 200, description = "Update a absence sended successfully"),
        (status = 401, description = "Unauthorized to update a absence", body = Error, example = json!({"Err": Error::Unauthorized})),
        (status = 422, description = "The Json is parsed in a wrong format", body = Error, example = json!({"Err": Error::UnprocessableEntity})),
    ),
    security (
        ("authorization" = []),
    )
)]
#[put("/criminal", format = "json", data = "<criminal>")]
pub async fn update_criminal(
    auth: Auth<CriminalWrite>,
    criminal: Json<Criminal>,
) -> Json<Result<()>> {
    warn!("PUT /criminal with data {criminal:?}: {}", auth.user);
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::criminal::update(&db, &criminal.account, &criminal))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Criminal delete sended successfully"),
        (status = 401, description = "Unauthorized to delete Criminal", body = Error, example = json!({"Err": Error::Unauthorized})),
    ),
    params(
        ("account", description = "The unique user account"),
    ),
    security(
        ("authorization" = []),
    )
)]
#[delete("/criminal/<account>")]
pub async fn delete_criminal(auth: Auth<UserWrite>, account: &str) -> Json<Result<()>> {
    warn!("DELETE /criminal/{account}: {}", auth.user);
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::criminal::delete(&db, account))
}

#[utoipa::path(
    request_body = Login,
    responses(
        (status = 200, description = "Add a Login sended successfully"),
        (status = 401, description = "Unauthorized to add a Logins", body = Error, example = json!({"Err": Error::Unauthorized})),
        (status = 422, description = "The Json is parsed in a wrong format", body = Error, example = json!({"Err": Error::UnprocessableEntity})),
    ),
    security (
        ("authorization" = []),
    )
)]
#[post("/login", format = "json", data = "<login>")]
pub async fn add_login(auth: Auth<UserWrite>, login: Json<Login>) -> Json<Result<()>> {
    warn!("POST /login with data {login:?}: {}", auth.user);
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::login::add(&db, &login))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Login delete sended successfully"),
        (status = 401, description = "Unauthorized to delete Login", body = Error, example = json!({"Err": Error::Unauthorized})),
    ),
    params(
        ("user", description = "The unique user"),
    ),
    security(
        ("authorization" = []),
    )
)]
#[delete("/login/<user>")]
pub async fn delete_login(auth: Auth<UserWrite>, user: &str) -> Json<Result<()>> {
    warn!("DELETE /login/{user}: {}", auth.user);
    let user = user.trim();

    if user == env::var("SNDM_USER").unwrap() {
        warn!("unable to delete admin '{user}'");
        return Json(Err(Error::InvalidUser));
    }

    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::login::delete(&db, user))
}
