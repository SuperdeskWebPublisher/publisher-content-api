pub mod pagination;

use chrono::prelude::*;
use crate::schema::{swp_article, swp_route, swp_article_media, swp_image, swp_image_rendition, swp_author, swp_article_author, swp_keyword, swp_article_keyword, swp_article_statistics};
use diesel::deserialize::Queryable;
use diesel::prelude::*;
use juniper_eager_loading::impl_load_from_for_diesel;

#[derive(Identifiable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "swp_article"]
pub struct Article {
    pub id: i32,
    pub slug: String,
    pub status: String,
    pub title: String,
    pub body: String,
    pub lead: String,
    pub route_id: i32,
    pub comments_count: i32,
    pub extra: Option<String>,
    pub metadata: Option<String>,
    pub feature_media: Option<i32>,
}

#[derive(Identifiable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "swp_route"]
pub struct Route {
    pub id: i32,
    pub name: String,
    // pub r#type: String,
}

#[derive(Identifiable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "swp_article_statistics"]
pub struct Statistics {
    pub id: i32,
    pub article_id: i32,
    pub page_views_number: i32,
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

impl juniper_eager_loading::LoadFrom<Article> for ArticleMedia {
    type Error = diesel::result::Error;
    type Connection = PgConnection;
    
    fn load(froms: &[Article], db: &Self::Connection) -> Result<Vec<Self>, Self::Error> {
        use diesel::pg::expression::dsl::any;

        let from_ids = froms.iter().map(|other| other.id).collect::<Vec<_>>();
        println!("{:?}", from_ids);
        swp_article_media::table
            .filter(swp_article_media::article_id.eq(any(from_ids)))
            .load(db)
            .map_err(From::from)
    }
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

        //Article.id -> (swp_article_media.article_id, ArticleMedia),
        ArticleMedia.article_id -> (swp_article.id, Article),
        Image.id -> (swp_image_rendition.image_id, ImageRendition),
        ArticleMedia.id -> (swp_image_rendition.media_id, ImageRendition),
        ImageRendition.media_id -> (swp_article_media.id, ArticleMedia),

        //Article.feature_media -> (swp_article_media.id, ArticleMedia),
        //ArticleMedia.id -> (swp_article.feature_media, Article),

        Statistics.article_id -> (swp_article.id, Article),

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
