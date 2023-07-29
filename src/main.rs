use actix_web::HttpServer;
use repository::prelude::LinkRepository;
use utils::server::config;

use std::env;

mod entity;
mod handler;
mod repository;
mod utils;

use crate::utils::server;

#[derive(Debug, Clone)]
pub struct AppState {
    pub link_repository: LinkRepository,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    if let Ok(server_util) = server::get_util_server() {
        env::set_var("RUST_LOG", server_util.rust_log.clone());
        let db_connection = sea_orm::Database::connect(server_util.db_connection.clone())
            .await
            .unwrap();

        let link_repository = LinkRepository { db_connection };
        let app_state = AppState { link_repository };

        let server = HttpServer::new(move || {
            actix_web::App::new()
                .app_data(actix_web::web::Data::new(app_state.clone()))
                .wrap(server::get_cross_origin())
                .configure(config)
        })
        .bind(server::get_host_and_port(server_util))?;
        server.run().await?;
    }

    Ok(())
}
