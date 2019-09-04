pub mod pagination;

use chrono::prelude::*;
use crate::schema::{swp_article, swp_route, swp_article_media, swp_image, swp_image_rendition, swp_author, swp_article_author, swp_keyword, swp_article_keyword};
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
    pub usage_terms: String,
}

#[derive(Identifiable, Queryable, Debug, PartialEq, Clone)]
#[table_name = "swp_image"]
pub struct Image {
    pub id: i32,
    pub asset_id: String,
    pub file_extension: String
}

#[derive(Identifiable, Queryable, Debug, PartialEq, Clone)]
#[table_name = "swp_image_rendition"]
pub struct ImageRendition {
    pub id: i32,
    pub image_id: i32,
    pub media_id: i32,
    pub width: i32,
    pub height: i32,
    pub name: String
}

#[derive(Identifiable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "swp_author"]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub role: String,
    pub job_title: String,
    pub biography: Option<String>,
    pub slug: Option<String>,
    pub twitter: Option<String>,
    pub facebook: Option<String>,
    pub instagram: Option<String>,
}

#[derive(Identifiable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "swp_article_author"]
#[primary_key(article_id)]
pub struct ArticleAuthor {
    pub article_id: i32,
    pub author_id: i32,
}

#[derive(Identifiable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "swp_keyword"]
pub struct Keyword {
    pub id: i32,
    pub name: String,
    pub slug: String
}

#[derive(Identifiable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "swp_article_keyword"]
#[primary_key(article_id)]
pub struct ArticleKeyword {
    pub article_id: i32,
    pub keyword_id: i32,
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
        i32 -> (swp_image_rendition, ImageRendition),
        i32 -> (swp_author, Author),
        i32 -> (swp_keyword, Keyword),

        Article.id -> (swp_article_media.article_id, ArticleMedia),
        ArticleMedia.article_id -> (swp_article.id, Article),
        Image.id -> (swp_image_rendition.image_id, ImageRendition),
        ArticleMedia.id-> (swp_image_rendition.media_id, ImageRendition),
        ImageRendition.media_id -> (swp_article_media.id, ArticleMedia),

        Author.id-> (swp_article_author.author_id, ArticleAuthor),
        Article.id-> (swp_article_author.article_id, ArticleAuthor),

        ArticleAuthor.author_id-> (swp_author.id, Author),
        ArticleAuthor.article_id-> (swp_article.id, Article),

        Keyword.id-> (swp_article_keyword.keyword_id, ArticleKeyword),
        Article.id-> (swp_article_keyword.article_id, ArticleKeyword),

        ArticleKeyword.keyword_id-> (swp_keyword.id, Keyword),
        ArticleKeyword.article_id-> (swp_article.id, Article),
    }
}
