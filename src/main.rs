pub mod db;
mod server;

use std::env;
use std::{borrow::Cow, path::Path};

use db::project::{fetch_user_data, Database, Result};

use log::{warn, Level, LevelFilter};
use rocket::serde::json::Json;
use rocket::{catch, catchers, response::Responder, routes, Build, Request, Response, Rocket};
use serde::Serialize;
use simplelog::{ConfigBuilder, WriteLogger};
use utoipa::openapi::security::{Http, HttpAuthScheme};
use utoipa::{openapi::security::SecurityScheme, Modify, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

use std::fs::OpenOptions;

use crate::db::login::{Login, Permission};
use crate::db::project::Error;

#[rocket::launch]
fn rocket() -> Rocket<Build> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.txt")
        .unwrap();

    WriteLogger::init(
        LevelFilter::Warn,
        ConfigBuilder::new()
            .set_time_format_rfc3339()
            .set_time_offset_to_local()
            .unwrap_or(&mut ConfigBuilder::default())
            .set_level_color(Level::Trace, None)
            .set_level_color(Level::Debug, None)
            .set_level_color(Level::Info, None)
            .set_level_color(Level::Warn, None)
            .set_level_color(Level::Error, None)
            .add_filter_ignore_str("rocket")
            .build(),
        file,
    )
    .unwrap();

    warn!("Started Logging");

    dotenv::from_filename("admin.env").ok();

    let path = Path::new("./sndm.db");
    match Database::open(Cow::from(path)) {
        Ok(db) => db.0,
        Err(_) => {
            let db = Database::create(Cow::from(path)).unwrap();
            db::project::create(&db).unwrap();
            fetch_user_data(&db, Cow::from(Path::new("./benutzer.txt")), "|").unwrap();
            // Admin user
            db::login::add(
                &db,
                &Login {
                    user: env::var("SNDM_USER").unwrap(),
                    password: env::var("SNDM_PASSWORD").unwrap(),
                    access_user: Permission::Write,
                    access_absence: Permission::Write,
                    access_criminal: Permission::Write,
                },
            )
            .unwrap();
            db
        }
    };

    #[derive(OpenApi)]
    #[openapi(
        paths(
            server::stats,
            server::fetch_user,
            server::search_user,
            server::all_roles,
            server::add_user,
            server::update_user,
            server::delete_user,
            server::fetch_absence,
            server::search_absence,
            server::all_dates,
            server::add_absence,
            server::update_absence,
            server::delete_absence,
            server::fetch_criminal,
            server::all_kinds,
            server::search_criminal,
            server::add_criminal,
            server::update_criminal,
            server::delete_criminal,
            server::fetch_permission,
            server::add_login,
            server::delete_login,
        ),
        components(
            schemas(db::user::User, db::absence::Absence, db::criminal::Criminal, db::login::Login, db::login::Permission, db::login::Permissions, db::stats::Stats, db::project::Error)
        ),
        tags(
            (name = "server", description = "Server management endpoints.")
        ),
        modifiers(&SecurityAddon)
    )]
    struct ApiDoc;

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
            components.add_security_scheme(
                "authorization",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Basic)),
            );
        }
    }

    let figment = rocket::Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("port", 80))
        .merge(("limits.json", 32768));

    rocket::custom(figment)
        .register(
            "/",
            catchers![
                unauthorized,
                not_found,
                unprocessable_entity,
                internal_error,
                exceeded_limit_413,
                exceeded_limit_414
            ],
        )
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .mount(
            "/",
            routes![
                server::index,
                server::login,
                server::static_files,
                server::stats,
                server::fetch_user,
                server::search_user,
                server::all_roles,
                server::add_user,
                server::update_user,
                server::delete_user,
                server::fetch_absence,
                server::search_absence,
                server::all_dates,
                server::add_absence,
                server::update_absence,
                server::delete_absence,
                server::fetch_criminal,
                server::all_kinds,
                server::search_criminal,
                server::add_criminal,
                server::update_criminal,
                server::delete_criminal,
                server::fetch_permission,
                server::add_login,
                server::delete_login,
            ],
        )
}

struct JsonWithHeaders<T: Serialize> {
    headers: Vec<(&'static str, &'static str)>,
    json: Json<T>,
}

impl<'r, T: Serialize> Responder<'r, 'static> for JsonWithHeaders<T> {
    fn respond_to(self, request: &'r Request<'_>) -> rocket::response::Result<'static> {
        let mut builder = Response::build_from(Responder::respond_to(self.json, request)?);
        for &(key, value) in &self.headers {
            builder.raw_header(key, value);
        }
        builder.ok()
    }
}

#[catch(401)]
async fn unauthorized<'r>(_req: &Request<'_>) -> JsonWithHeaders<Result<()>> {
    let json = Json(Err(Error::Unauthorized));
    let headers = vec![("WWW-Authenticate", "Basic realm=\"User Visible Realm\"")];
    JsonWithHeaders { headers, json }
}

#[catch(404)]
async fn not_found(_req: &Request<'_>) -> Json<Result<()>> {
    Json(Err(Error::PageNotFound))
}

#[catch(422)]
async fn unprocessable_entity(_req: &Request<'_>) -> Json<Result<()>> {
    Json(Err(Error::UnprocessableEntity))
}

#[catch(500)]
async fn internal_error(_req: &Request<'_>) -> Json<Result<()>> {
    Json(Err(Error::InternalError))
}

#[catch(413)]
async fn exceeded_limit_413(_req: &Request<'_>) -> Json<Result<()>> {
    Json(Err(Error::ExceededLimit))
}

#[catch(414)]
async fn exceeded_limit_414(_req: &Request<'_>) -> Json<Result<()>> {
    Json(Err(Error::ExceededLimit))
}
