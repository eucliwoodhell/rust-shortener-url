use crate::{
    repository::{link::LinkRequest, prelude::UrlRequest},
    AppState,
};
use actix_web::{
    delete, get, post, web, Error as ActixError, HttpResponse, Responder, Result as ActixResult,
    Scope,
};
use log::debug;
use rand::Rng;
use serde_json::json;
use validator::Validate;

#[utoipa::path(
    get,
    path = "/link",
    responses(
        (status = 200, description = "List current link items", body = [Model])
    )
)]
#[get("")]
pub async fn get_link(state: web::Data<AppState>) -> ActixResult<impl Responder, ActixError> {
    let link = state.link_repository.get().await;
    debug!("got: {:?}", link);
    Ok(web::Json(link))
}

#[utoipa::path(
    get,
    path = "/link/{id}",
    params(
        ("id", description = "Unique storage id of Link")
    ),
    responses(
        (status = 200, description = "Link items", body = [Model])
    )
)]
#[get("/{id}")]
pub async fn get_link_by_id(
    state: web::Data<AppState>,
    short: web::Path<String>,
) -> ActixResult<impl Responder, ActixError> {
    debug!("short: {:?}", short);
    let link = state.link_repository.get_by_id(short.into_inner()).await;
    debug!("got: {:?}", link);
    Ok(web::Json(link))
}

#[utoipa::path(
    request_body = UrlRequest,
    path = "/link",
    responses(
        (status = 200, description = "Created new link", body = [Model])
    )
)]
#[post("")]
async fn create_link(state: web::Data<AppState>, new_url: web::Json<UrlRequest>) -> impl Responder {
    let is_valid = new_url.validate();
    match is_valid {
        Ok(_) => {
            let new_link = LinkRequest {
                url: new_url.url.clone(),
                short_url: generate_random_characters(5),
            };
            let link = state.link_repository.create(web::Json(new_link)).await;
            debug!("created: {:?}", link);
            HttpResponse::Ok().json(link)
        }
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}

fn generate_random_characters(lenght: usize) -> String {
    let rng = rand::thread_rng();
    rng.sample_iter(rand::distributions::Alphanumeric)
        .take(lenght)
        .map(char::from)
        .collect()
}

#[utoipa::path(
    delete,
    path = "/link/{id}",
    params(
        ("id", description = "Unique storage id of Link")
    ),
    responses(
        (status = 200, description = "Deleted link")
    )
)]
#[delete("/{id}")]
async fn delete_link(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> ActixResult<impl Responder, ActixError> {
    let link = state.link_repository.delete(id.into_inner()).await;
    debug!("deleted: {:?}", link.rows_affected);
    Ok(web::Json(json!({
        "message": "deleted",
        "deleted": link.rows_affected,
    })))
}

// Scope link
pub fn link_handler() -> Scope {
    web::scope("/link")
        .service(get_link)
        .service(get_link_by_id)
        .service(create_link)
        .service(delete_link)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{repository::prelude::LinkRepository, utils::server::get_cross_origin};
    use actix_web::{
        dev::Service,
        http,
        rt::spawn,
        test,
        web::{self, Data},
        App,
    };

    use std::{
        env,
        sync::atomic::{AtomicUsize, Ordering},
        sync::Arc,
    };

    #[actix_web::test]
    async fn test_get_link() {
        dotenv::dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_connection = sea_orm::Database::connect(db_url).await.unwrap();
        let app = test::init_service(
            App::new()
                .app_data(Data::new(AppState {
                    link_repository: LinkRepository { db_connection },
                }))
                .service(web::scope("/link").service(get_link)),
        )
        .await;

        let req = test::TestRequest::get().uri("/link").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_link_by_id() {
        dotenv::dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_connection = sea_orm::Database::connect(db_url).await.unwrap();
        let app = test::init_service(
            App::new()
                .app_data(Data::new(AppState {
                    link_repository: LinkRepository { db_connection },
                }))
                .service(web::scope("/link").service(get_link_by_id)),
        )
        .await;

        let req = test::TestRequest::get().uri("/link/1").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_create_link() {
        dotenv::dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_connection = sea_orm::Database::connect(db_url).await.unwrap();

        let app = test::init_service(
            App::new()
                .app_data(Data::new(AppState {
                    link_repository: LinkRepository { db_connection },
                }))
                .wrap(get_cross_origin())
                .service(web::scope("/link").service(create_link)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/link")
            .set_json(&UrlRequest {
                url: "https://google.com".to_string(),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_create_link_generate_short() {
        let gr = generate_random_characters(5);
        assert_eq!(gr.len(), 5);
    }

    #[actix_web::test]
    async fn test_delete_link() {
        dotenv::dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_connection = sea_orm::Database::connect(db_url).await.unwrap();

        let app = test::init_service(
            App::new()
                .app_data(Data::new(AppState {
                    link_repository: LinkRepository { db_connection },
                }))
                .wrap(get_cross_origin())
                .service(web::scope("/link").service(delete_link)),
        )
        .await;

        let req = test::TestRequest::delete().uri("/link/1").to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_generate_short_url() {
        let short_url = generate_random_characters(5);
        assert_eq!(short_url.len(), 5);
    }

    // TODO test 5000 request per second
    #[actix_web::test]
    async fn test_get_link_5000() {
        dotenv::dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_connection = sea_orm::Database::connect(db_url).await.unwrap();
        let server = test::init_service(
            App::new()
                .app_data(Data::new(AppState {
                    link_repository: LinkRepository { db_connection },
                }))
                .service(web::scope("/link").service(get_link)),
        )
        .await;

        // Create a shared counter to keep track of the completed requests
        let counter = Arc::new(AtomicUsize::new(0));
        let num_requests = 5000;

        // Use multiple async tasks to send requests concurrently
        let mut tasks = vec![];
        let server = Arc::new(server);

        for _ in 0..num_requests {
            let server = server.clone();
            let counter = counter.clone();

            let task = spawn(async move {
                let server = server.clone();
                let request = test::TestRequest::get().uri("/link").to_request();

                let response = server.call(request).await.unwrap();
                if response.status().is_success() {
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            });

            tasks.push(task);
        }

        for task in tasks {
            task.await.unwrap();
        }

        assert_eq!(counter.load(Ordering::Relaxed), num_requests);
    }
}
