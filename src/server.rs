use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use log::warn;
use rocket::{
    delete,
    fs::NamedFile,
    get,
    http::Status,
    outcome::Outcome,
    post, put,
    request::{self, FromRequest},
    serde::json::Json,
    Request,
};

use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};
use std::{env, marker::PhantomData};

use crate::db::{
    self,
    login::{NewLogin, Permissions},
    user::UserSearch,
};
use chrono::NaiveDate;

use db::criminal::{Criminal, CriminalSearch};
use db::login::{Login, Permission};
use db::project::{Database, Error, Result};
use db::stats::Stats;
use db::user::User;
use db::workless::{Workless, WorklessSearch};

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
pub struct WorklessReadOnly {}
impl Access for WorklessReadOnly {
    fn check(l: Login) -> bool {
        matches!(l.access_workless, Permission::ReadOnly | Permission::Write)
    }
}
pub struct WorklessWrite {}
impl Access for WorklessWrite {
    fn check(l: Login) -> bool {
        matches!(l.access_workless, Permission::Write)
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
            .and_then(|v| String::from_utf8(v).ok())
        else {
            warn!("missing auth {header:?} from {:?}", request.client_ip());
            return Outcome::Failure((Status::Unauthorized, Error::Unauthorized));
        };
        let Some((user, password)) = user_pass.split_once(':') else {
            warn!(
                "wrong auth header '{user_pass}' from {:?}",
                request.client_ip()
            );
            return Outcome::Failure((Status::Unauthorized, Error::Unauthorized));
        };

        // lookup in database

        let Ok((db, _)) = Database::open(Cow::from(Path::new("./sndm.db"))) else {
            warn!("could not open Database");
            return Outcome::Failure((Status::Unauthorized, Error::Unauthorized));
        };
        let Ok(login) = db::login::fetch(&db, user) else {
            warn!(
                "missing auth credentials '{user}:{password}' from {:?}",
                request.client_ip()
            );
            return Outcome::Failure((Status::Unauthorized, Error::Unauthorized));
        };

        // checking password

        if !login.check_password(password) {
            return Outcome::Failure((Status::Unauthorized, Error::Unauthorized));
        }

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

#[get("/login")]
pub async fn login() -> Option<NamedFile> {
    let path = Path::new("static").join("login.html");
    NamedFile::open(path).await.ok()
}

#[get("/<path..>")]
pub async fn static_files(path: PathBuf) -> Option<NamedFile> {
    let path = Path::new("static").join(path);
    NamedFile::open(path).await.ok()
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
pub async fn stats(_auth: Auth<UserReadOnly>) -> Json<Result<Stats>> {
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
pub async fn fetch_user(_auth: Auth<UserReadOnly>, id: &str) -> Json<Result<User>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::user::fetch(&db, id))
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
#[get("/user/search?<name>&<role>&<limit>")]
pub async fn search_user(
    _auth: Auth<UserReadOnly>,
    name: Option<&str>,
    role: Option<&str>,
    limit: Option<usize>,
) -> Json<Result<Vec<User>>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::user::search(
        &db,
        UserSearch::new(name.unwrap_or_default(), role.unwrap_or("%")),
        limit.unwrap_or(200),
    ))
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
#[get("/user/all_roles?<name>")]
pub async fn all_roles(_auth: Auth<UserReadOnly>, name: Option<&str>) -> Json<Result<Vec<String>>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::user::all_roles(&db, name.unwrap_or("")))
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
    params(
        ("id", description = "The unique user id")
    ),
    security (
        ("authorization" = []),
    )
)]
#[put("/user/<id>", format = "json", data = "<user>")]
pub async fn update_user(auth: Auth<UserWrite>, user: Json<User>, id: &str) -> Json<Result<()>> {
    warn!("PUT /user/{id} with data {user:?}: {}", auth.user);
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::user::update(&db, id, &user))
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
        (status = 200, description = "Got a Workless by a specific account, old company and date", body = Workless),
        (status = 401, description = "Unauthorized to fetch a Workless", body = Error, example = json!({"Err": Error::Unauthorized})),
    ),
    params(
        ("account", description = "The unique user account"),
        ("old_company", description = "The old company"),
        ("date", description = "The date")
    ),
    security (
        ("authorization" = []),
    )
)]
#[get("/workless/fetch/<account>/<old_company>/<date>")]
pub async fn fetch_workless(
    _auth: Auth<WorklessReadOnly>,
    account: &str,
    old_company: &str,
    date: &str,
) -> Json<Result<Workless>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    let date = match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            return Json(Err(Error::InvalidDate));
        }
    };
    Json(db::workless::fetch(&db, account, old_company, date))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Searched all Workless", body = Vec<Workless>),
        (status = 401, description = "Unauthorized to search all Workless", body = Error, example = json!({"Err": Error::Unauthorized})),
    ),
    security (
        ("authorization" = []),
    )
)]
#[get("/workless/search?<name>&<old_company>&<date>&<limit>")]
pub async fn search_workless(
    _auth: Auth<WorklessReadOnly>,
    name: Option<&str>,
    old_company: Option<&str>,
    date: Option<&str>,
    limit: Option<usize>,
) -> Json<Result<Vec<Workless>>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::workless::search(
        &db,
        WorklessSearch::new(
            name.unwrap_or_default(),
            old_company.unwrap_or("%"),
            date.unwrap_or("%"),
        ),
        limit.unwrap_or(200),
    ))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Searched all Workless by roles", body = Vec<Workless>),
        (status = 401, description = "Unauthorized to search all Workless by roles", body = Error, example = json!({"Err": Error::Unauthorized})),
    ),
    security (
        ("authorization" = []),
    )
)]
#[get("/workless/search_role?<name>&<date>&<role>&<limit>")]
pub async fn search_workless_roles(
    _auth: Auth<WorklessReadOnly>,
    name: Option<&str>,
    date: Option<&str>,
    role: Option<&str>,
    limit: Option<usize>,
) -> Json<Result<Vec<Workless>>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::workless::search_role(
        &db,
        name.unwrap_or(""),
        role.unwrap_or("%"),
        date.unwrap_or("%"),
        limit.unwrap_or(9999),
    ))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Got all Dates", body = Vec<String>),
        (status = 401, description = "Unauthorized to get all Dates", body = Error, example = json!({"Err": Error::Unauthorized})),
    ),
    security (
        ("authorization" = []),
    )
)]
#[get("/workless/all_dates")]
pub async fn all_dates(_auth: Auth<WorklessReadOnly>) -> Json<Result<Vec<String>>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::workless::all_dates(&db))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Got all Roles by Date", body = Vec<String>),
        (status = 401, description = "Unauthorized to get all Roles by Date", body = Error, example = json!({"Err": Error::Unauthorized})),
    ),
    security (
        ("authorization" = []),
    )
)]
#[get("/workless/all_roles?<date>&<name>")]
pub async fn all_roles_workless(
    _auth: Auth<WorklessReadOnly>,
    date: Option<&str>,
    name: Option<&str>,
) -> Json<Result<Vec<String>>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::workless::all_roles(
        &db,
        date.unwrap_or("%"),
        name.unwrap_or(""),
    ))
}

#[utoipa::path(
    request_body = Workless,
    responses(
        (status = 200, description = "Add an Workless sended successfully"),
        (status = 401, description = "Unauthorized to add a Workless", body = Error, example = json!({"Err": Error::Unauthorized})),
        (status = 422, description = "The Json is parsed in a wrong format", body = Error, example = json!({"Err": Error::UnprocessableEntity})),
    ),
    security (
        ("authorization" = []),
    )
)]
#[post("/workless", format = "json", data = "<workless>")]
pub async fn add_workless(auth: Auth<WorklessWrite>, workless: Json<Workless>) -> Json<Result<()>> {
    warn!("POST /workless with data {workless:?}: {}", auth.user);
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::workless::add(&db, &workless))
}

#[utoipa::path(
    request_body = Workless,
    responses (
        (status = 200, description = "Update an Workless sended successfully"),
        (status = 401, description = "Unauthorized to update an Workless", body = Error, example = json!({"Err": Error::Unauthorized})),
        (status = 422, description = "The Json is parsed in a wrong format", body = Error, example = json!({"Err": Error::UnprocessableEntity})),
    ),
    params(
        ("previous_account", description = "The unique user account"),
        ("previous_old_company", description = "The old company"),
        ("previous_date", description = "The date")
    ),
    security (
        ("authorization" = []),
    )
)]
#[put(
    "/workless/<previous_account>/<previous_old_company>/<previous_date>",
    format = "json",
    data = "<workless>"
)]
pub async fn update_workless(
    auth: Auth<WorklessWrite>,
    workless: Json<Workless>,
    previous_account: &str,
    previous_old_company: &str,
    previous_date: &str,
) -> Json<Result<()>> {
    warn!(
        "PUT /workless/{previous_account}/{previous_old_company}/{previous_date} with data {workless:?}: {}",
        auth.user
    );
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    let previous_date = match NaiveDate::parse_from_str(previous_date, "%Y-%m-%d") {
        Ok(previous_date) => previous_date,
        Err(_) => {
            return Json(Err(Error::InvalidDate));
        }
    };
    Json(db::workless::update(
        &db,
        previous_account,
        previous_old_company,
        previous_date,
        &workless,
    ))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Workless delete sended successfully"),
        (status = 401, description = "Unauthorized to delete Workless", body = Error, example = json!({"Err": Error::Unauthorized})),
    ),
    params(
        ("account", description = "The unique user account"),
        ("old_company", description = "The old company"),
        ("date", description = "The date")
    ),
    security(
        ("authorization" = []),
    )
)]
#[delete("/workless/<account>/<old_company>/<date>")]
pub async fn delete_workless(
    auth: Auth<WorklessWrite>,
    account: &str,
    old_company: &str,
    date: &str,
) -> Json<Result<()>> {
    warn!("DELETE /workless/{account}/{date}: {}", auth.user);
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    let date = match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            return Json(Err(Error::InvalidDate));
        }
    };
    Json(db::workless::delete(&db, account, old_company, date))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Got a Criminal by a specific account and kind", body = Criminal),
        (status = 401, description = "Unauthorized to fetch a Criminal", body = Error, example = json!({"Err": Error::Unauthorized})),
    ),
    params(
        ("account", description = "The unique user account"),
        ("kind", description = "The kind of the crime"),
    ),
    security (
        ("authorization" = []),
    )
)]
#[get("/criminal/fetch/<account>/<kind>")]
pub async fn fetch_criminal(
    _auth: Auth<CriminalReadOnly>,
    account: &str,
    kind: &str,
) -> Json<Result<Criminal>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::criminal::fetch(&db, account, kind))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Got all Kinds", body = Vec<String>),
        (status = 401, description = "Unauthorized to get all Kinds", body = Error, example = json!({"Err": Error::Unauthorized})),
    ),
    security (
        ("authorization" = []),
    )
)]
#[get("/criminal/all_accounts")]
pub async fn all_accounts(_auth: Auth<CriminalReadOnly>) -> Json<Result<Vec<String>>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::criminal::all_accounts(&db))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Got all Roles by Criminal", body = Vec<String>),
        (status = 401, description = "Unauthorized to get all Roles by Criminal", body = Error, example = json!({"Err": Error::Unauthorized})),
    ),
    security (
        ("authorization" = []),
    )
)]
#[get("/criminal/all_roles?<name>")]
pub async fn all_roles_criminal(
    _auth: Auth<CriminalReadOnly>,
    name: Option<&str>,
) -> Json<Result<Vec<String>>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::criminal::all_roles(&db, name.unwrap_or("")))
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
#[get("/criminal/search?<name>&<kind>&<limit>")]
pub async fn search_criminal(
    _auth: Auth<CriminalReadOnly>,
    name: Option<&str>,
    kind: Option<&str>,
    limit: Option<usize>,
) -> Json<Result<Vec<Criminal>>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::criminal::search(
        &db,
        CriminalSearch::new(name.unwrap_or_default(), kind.unwrap_or("%")),
        limit.unwrap_or(200),
    ))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Searched all Criminals by roles", body = Vec<Criminal>),
        (status = 401, description = "Unauthorized to search all Criminals by roles", body = Error, example = json!({"Err": Error::Unauthorized})),
    ),
    security (
        ("authorization" = []),
    )
)]
#[get("/criminal/search_role?<name>&<role>&<limit>")]
pub async fn search_criminal_roles(
    _auth: Auth<CriminalReadOnly>,
    name: Option<&str>,
    role: Option<&str>,
    limit: Option<usize>,
) -> Json<Result<Vec<Criminal>>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::criminal::search_role(
        &db,
        name.unwrap_or(""),
        role.unwrap_or("%"),
        limit.unwrap_or(9999),
    ))
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
        (status = 200, description = "Update a workless sended successfully"),
        (status = 401, description = "Unauthorized to update a workless", body = Error, example = json!({"Err": Error::Unauthorized})),
        (status = 422, description = "The Json is parsed in a wrong format", body = Error, example = json!({"Err": Error::UnprocessableEntity})),
    ),
    params(
        ("previous_account", description = "The unique user account"),
        ("previous_kind", description = "The kind of the crime"),
    ),
    security (
        ("authorization" = []),
    )
)]
#[put(
    "/criminal/<previous_account>/<previous_kind>",
    format = "json",
    data = "<criminal>"
)]
pub async fn update_criminal(
    auth: Auth<CriminalWrite>,
    previous_account: &str,
    previous_kind: &str,
    criminal: Json<Criminal>,
) -> Json<Result<()>> {
    warn!(
        "PUT /criminal/{previous_account}/{previous_kind} with data {criminal:?}: {}",
        auth.user
    );
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::criminal::update(
        &db,
        previous_account,
        previous_kind,
        &criminal,
    ))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Criminal delete sended successfully"),
        (status = 401, description = "Unauthorized to delete Criminal", body = Error, example = json!({"Err": Error::Unauthorized})),
    ),
    params(
        ("account", description = "The unique user account"),
        ("kind", description = "The kind of the crime"),
    ),
    security(
        ("authorization" = []),
    )
)]
#[delete("/criminal/<account>/<kind>")]
pub async fn delete_criminal(
    auth: Auth<CriminalWrite>,
    account: &str,
    kind: &str,
) -> Json<Result<()>> {
    warn!("DELETE /criminal/{account}/{kind}: {}", auth.user);
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::criminal::delete(&db, account, kind))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Got a Permissions by a specific user", body = Permissions),
        (status = 401, description = "Unauthorized to fetch Permissions", body = Error, example = json!({"Err": Error::Unauthorized})),
    ),
    params(
        ("user", description = "The unique user"),
    ),
    security (
        ("authorization" = []),
    )
)]
#[get("/login/fetch/<user>")]
pub async fn fetch_permission(_auth: Auth<UserReadOnly>, user: &str) -> Json<Result<Permissions>> {
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::login::fetch_permission(&db, user))
}

#[utoipa::path(
    request_body = NewLogin,
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
pub async fn add_login(auth: Auth<UserWrite>, login: Json<NewLogin>) -> Json<Result<()>> {
    warn!("POST /login with data {login:?}: {}", auth.user);
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;
    Json(db::login::add(&db, login.into_inner()))
}

#[utoipa::path(
    request_body = NewLogin,
    responses(
        (status = 200, description = "Update a login sended successfully"),
        (status = 401, description = "Unauthorized to update a login", body = Error, example = json!({"Err": Error::Unauthorized})),
        (status = 422, description = "The Json is parsed in a wrong format", body = Error, example = json!({"Err": Error::UnprocessableEntity})),
    ),
    security (
        ("authorization" = []),
    )
)]
#[put("/login", format = "json", data = "<login>")]
pub async fn update_login(auth: Auth<UserReadOnly>, login: Json<NewLogin>) -> Json<Result<()>> {
    warn!("PUT /login with data {login:?}: {}", auth.user);
    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;

    if db::login::fetch(&db, &login.user).is_err() {
        warn!("invalid user of login json");
        return Json(Err(Error::InvalidLogin));
    }

    if auth.user != login.user {
        warn!("missing permissions to change user password: {}", auth.user);
        return Json(Err(Error::Unauthorized));
    }

    Json(db::login::update(
        &db,
        &login.clone().into_inner().user,
        &login.into_inner().password,
    ))
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

#[utoipa::path(
    responses(
        (status = 200, description = "All Logins delete sended successfully"),
        (status = 401, description = "Unauthorized to delete All Logins", body = Error, example = json!({"Err": Error::Unauthorized})),
    ),
    security(
        ("authorization" = []),
    )
)]
#[delete("/all_logins")]
pub async fn delete_all_logins(auth: Auth<UserWrite>) -> Json<Result<()>> {
    warn!("DELETE /all_logins: {}", auth.user);

    let db = Database::open(Cow::from(Path::new("./sndm.db"))).unwrap().0;

    let users = db::login::all_logins(&db).unwrap();

    for user in &users {
        if *user == env::var("SNDM_USER").unwrap() {
            warn!("unable to delete admin '{}'", user);
        } else {
            db::login::delete(&db, user).unwrap();
        }
    }
    Json(Ok(()))
}
