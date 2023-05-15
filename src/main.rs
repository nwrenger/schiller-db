#![feature(const_option_ext)]
pub mod db;
mod server;

use std::{borrow::Cow, path::Path};

use db::project::{fetch_user_data, Database};

use rocket::fairing::{Fairing, Info, Kind};
use rocket::{catch, catchers, routes, Build, Request, Response, Rocket};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;

use crate::db::project::Error;

struct SuccessLogger;

#[rocket::async_trait]
impl Fairing for SuccessLogger {
    fn info(&self) -> Info {
        Info {
            name: "Success Logger",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if response.status().code == 200
            && !request.uri().to_string().starts_with("/swagger-ui")
            && !request.uri().to_string().starts_with("/api-docs")
        {
            let now = Local::now();
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("log.txt")
                .unwrap();
            writeln!(
                file,
                "{} Successful request made to {} route {} from IP {}",
                now.format("[%d-%m-%Y|%H:%M:%S%.3f]"),
                request.method(),
                request.uri(),
                request.remote().unwrap()
            )
            .unwrap();
        }
    }
}

#[rocket::launch]
fn rocket() -> Rocket<Build> {
    let path = Path::new("./sndm.db");
    match Database::open(Cow::from(path)) {
        Ok(db) => db.0,
        Err(_) => {
            let db = Database::create(Cow::from(path)).unwrap();
            db::project::create(&db).unwrap();
            fetch_user_data(&db, Cow::from(Path::new("./benutzer.txt")), "|").unwrap();
            db
        }
    };

    #[derive(OpenApi)]
    #[openapi(
        paths(
            server::info,
            server::stats,
            server::fetch_user,
            server::search_user,
            server::add_user,
            server::update_user,
            server::delete_user,
            server::fetch_absence,
            server::search_absence,
            server::add_absence,
            server::update_absence,
            server::delete_absence,
            server::fetch_criminal,
            server::search_criminal,
            server::add_criminal,
            server::update_criminal,
            server::delete_criminal,
        ),
        components(
            schemas(db::project::User, db::project::Absence, db::project::Criminal, db::stats::Stats, db::project::Error, server::Info)
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
                "server_api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("server_api_key"))),
            );
            components.add_security_scheme(
                "write_api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("write_api_key"))),
            )
        }
    }

    let figment = rocket::Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("port", 80));

    rocket::custom(figment)
        .register(
            "/",
            catchers![
                unauthorized,
                not_found,
                unprocessable_entity,
                internal_error
            ],
        )
        .attach(SuccessLogger)
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .mount(
            "/",
            routes![
                server::info,
                server::stats,
                server::fetch_user,
                server::search_user,
                server::add_user,
                server::update_user,
                server::delete_user,
                server::fetch_absence,
                server::search_absence,
                server::add_absence,
                server::update_absence,
                server::delete_absence,
                server::fetch_criminal,
                server::search_criminal,
                server::add_criminal,
                server::update_criminal,
                server::delete_criminal,
            ],
        )
}

#[catch(401)]
async fn unauthorized(_req: &Request<'_>) -> serde_json::Value {
    Error::Unauthorized.into()
}

#[catch(404)]
async fn not_found(_req: &Request<'_>) -> serde_json::Value {
    Error::NotFound.into()
}

#[catch(422)]
async fn unprocessable_entity(_req: &Request<'_>) -> serde_json::Value {
    Error::UnprocessableEntity.into()
}

#[catch(500)]
async fn internal_error(_req: &Request<'_>) -> serde_json::Value {
    Error::InternalError.into()
}
