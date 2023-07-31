use actix_cors::Cors;
use actix_web::{http, web, HttpResponse};

use async_std::io;
use log::{debug, error};
use std::{env, io::Error};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::entity::link;
use crate::handler;
use crate::{handler::prelude::link_handler, repository};

pub fn get_util_server() -> Result<Server, Error> {
    env_logger::init();
    match (
        Host::try_from(env::var("HOST").unwrap()),
        Port::try_from(env::var("PORT").unwrap()),
        RustLog::try_from(env::var("RUST_LOG").unwrap()),
        DbConnection::try_from(env::var("DATABASE_URL").unwrap()),
        HostAllowed::try_from(env::var("ALLOWED_HOSTS").unwrap()),
    ) {
        (Ok(host), Ok(port), Ok(rust_log), Ok(db_connection), Ok(host_allowed)) => {
            debug!(
                "host: {}, port: {}, rust_log: {}, db_connection: {}, host_allowed: {}",
                env::var("HOST").unwrap(),
                env::var("PORT").unwrap(),
                env::var("RUST_LOG").unwrap(),
                env::var("DATABASE_URL").unwrap(),
                env::var("ALLOWED_HOSTS").unwrap(),
            );
            Ok(Server {
                host: String::from(host),
                port: String::from(port),
                rust_log: String::from(rust_log),
                db_connection: String::from(db_connection),
                allowed_hosts: String::from(host_allowed),
            })
        }
        _ => {
            error!("Failed to parse env variables into server struct type values");
            Err(Error::new(
                io::ErrorKind::Other,
                "Failed to parse env variables",
            ))
        }
    }
}

pub fn get_cross_origin() -> Cors {
    Cors::default()
        .allowed_origin(env::var("ALLOWED_HOSTS").unwrap().as_str())
        .allowed_methods(vec!["GET", "POST", "DELETE"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    #[derive(OpenApi)]
    #[openapi(
        paths(handler::prelude::get_link, handler::prelude::create_link, handler::prelude::delete_link),
        components(
            schemas(link::Model, repository::link::UrlRequest)
        ),
        tags((name = "Link", description = "Link management endpoints."))
    )]
    struct ApiDoc;
    let openapi = ApiDoc::openapi();
    cfg.service(web::scope("/health").service(web::resource("").to(|| HttpResponse::Ok())))
        .service(link_handler())
        .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi));
}

pub fn get_host_and_port(server: Server) -> String {
    let host_str = server.host;
    let port_str = server.port;
    debug!("Start server");
    format!("{}:{}", host_str, port_str)
}

pub struct Server {
    pub host: String,
    pub port: String,
    pub rust_log: String,
    pub db_connection: String,
    pub allowed_hosts: String,
}

pub struct RustLog(String);
impl TryFrom<String> for RustLog {
    type Error = ();
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(());
        } else {
            Ok(Self(value))
        }
    }
}
impl From<RustLog> for String {
    fn from(value: RustLog) -> Self {
        value.0
    }
}

#[derive(Debug)]
pub struct DbConnection(String);
impl TryFrom<String> for DbConnection {
    type Error = ();
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(());
        } else {
            Ok(Self(value))
        }
    }
}
impl From<DbConnection> for String {
    fn from(value: DbConnection) -> Self {
        value.0
    }
}

#[derive(Debug)]
pub struct Host(String);
impl TryFrom<String> for Host {
    type Error = ();
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(());
        } else {
            Ok(Self(value))
        }
    }
}
impl From<Host> for String {
    fn from(value: Host) -> Self {
        value.0
    }
}

#[derive(Debug)]
pub struct Port(String);
impl TryFrom<String> for Port {
    type Error = ();
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(());
        } else {
            Ok(Self(value))
        }
    }
}

impl From<Port> for String {
    fn from(value: Port) -> Self {
        value.0
    }
}

#[derive(Debug)]
pub struct HostAllowed(String);
impl TryFrom<String> for HostAllowed {
    type Error = ();
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(());
        } else {
            Ok(Self(value))
        }
    }
}

impl From<HostAllowed> for String {
    fn from(value: HostAllowed) -> Self {
        value.0
    }
}
