use crate::entity::{link, prelude::*};
use actix_web::web::Json;
use lazy_static::lazy_static;
use log::debug;
use regex::Regex;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{self, NotSet},
    ColumnTrait, DatabaseConnection, DeleteResult, EntityTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

lazy_static! {
    static ref RE_URL: Regex = Regex::new(r"^(https?|ftp)://[^\s/$.?#].[^\s]*$").unwrap();
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct LinkRequest {
    pub url: String,
    pub short_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, ToSchema, Validate)]
pub struct UrlRequest {
    #[validate(regex(path = "RE_URL", message = "Url is not valid"))]
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct LinkRepository {
    pub db_connection: DatabaseConnection,
}

impl LinkRepository {
    pub async fn get(&self) -> Vec<link::Model> {
        Link::find()
            .all(&self.db_connection)
            .await
            .expect("failed to get link")
    }

    pub async fn get_by_id(&self, short: String) -> Option<link::Model> {
        Link::find()
            .filter(link::Column::ShortUrl.contains(&short))
            .one(&self.db_connection)
            .await
            .expect("failed to get link")
    }

    pub async fn create(&self, new_link: Json<LinkRequest>) -> Option<link::Model> {
        let link = link::ActiveModel {
            id: NotSet,
            url: ActiveValue::Set(new_link.url.to_owned()),
            short_url: ActiveValue::Set(new_link.short_url.to_owned()),
        };
        let link: link::Model = link
            .insert(&self.db_connection)
            .await
            .expect("failed to create link");
        debug!("saved: {:?}", link);
        return link.into();
    }

    pub async fn delete(&self, id: i32) -> DeleteResult {
        let result: DeleteResult = Link::delete_by_id(id)
            .exec(&self.db_connection)
            .await
            .expect("failed to delete link");
        debug!("deleted: {:?}", result);
        return result.into();
    }
}
