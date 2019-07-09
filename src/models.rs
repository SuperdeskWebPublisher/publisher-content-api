use chrono::prelude::*;
use crate::schema::{swp_article, swp_route, swp_article_media, swp_image};
use diesel::deserialize::Queryable;
use diesel::prelude::*;
use juniper_eager_loading::impl_load_from_for_diesel;

#[derive(Identifiable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "swp_article"]
pub struct Article {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub body: String,
    pub lead: String,
    pub route_id: i32,
}

#[derive(Identifiable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "swp_route"]
pub struct Route {
    pub id: i32,
    pub name: String,
    // pub r#type: String,
}

#[derive(Identifiable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "swp_article_media"]
pub struct ArticleMedia {
    pub id: i32,
    pub article_id: i32,
    pub image_id: i32,
    pub key: String,
    pub body: String,
    pub description: String,
    pub located: String,
    pub by_line: String,
    pub mimetype: String,
    pub usage_terms: String
}

#[derive(Identifiable, Queryable, Debug, PartialEq, Clone)]
#[table_name = "swp_image"]
pub struct Image {
    pub id: i32,
    pub asset_id: String,
    pub file_extension: String
}

impl_load_from_for_diesel! {
    (
        error = diesel::result::Error,
        connection = PgConnection,
    ) => {
        i32 -> (swp_article, Article),
        i32 -> (swp_route, Route),
        i32 -> (swp_article_media, ArticleMedia),
        i32 -> (swp_image, Image),

        Article.id -> (swp_article_media.article_id, ArticleMedia),
        ArticleMedia.article_id -> (swp_article.id, Article),
    }
}
