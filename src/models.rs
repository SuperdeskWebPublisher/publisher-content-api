pub mod pagination;

use chrono::prelude::*;
use crate::schema::{
    swp_article, 
    swp_route, 
    swp_article_media, 
    swp_image, 
    swp_image_rendition, 
    swp_author, 
    swp_author_media, 
    swp_article_author, 
    swp_keyword, 
    swp_article_keyword, 
    swp_article_statistics,
    swp_article_seo_metadata,
    swp_article_seo_media,
    swp_article_related,
    swp_article_source,
    swp_article_sources,
    swp_slideshow,
    swp_slideshow_item
};
use diesel::deserialize::Queryable;
use diesel::prelude::*;
use juniper_eager_loading::impl_load_from_for_diesel_pg;

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
    pub seo_metadata_id: Option<i32>,
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
    pub media_id: i32,
    pub image_id: i32,
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
    pub author_media_id: Option<i32>
}

#[derive(Identifiable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "swp_article_author"]
#[primary_key(article_id)]
pub struct ArticleAuthor {
    pub article_id: i32,
    pub author_id: i32,
}

#[derive(Identifiable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "swp_author_media"]
pub struct AuthorAvatar {
    pub id: i32,
    //pub author_id: i32,
    pub image_id: i32,
    pub key: String
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

#[derive(Identifiable, Queryable, Debug, PartialEq, Clone)]
#[table_name = "swp_article_seo_metadata"]
pub struct ArticleSeoMetadata {
    pub id: i32,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub twitter_title: Option<String>,
    pub twitter_description: Option<String>,
    pub seo_meta_media_id: Option<i32>,
    pub seo_og_media_id: Option<i32>,
    pub seo_twitter_media_id: Option<i32>,
}

#[derive(Identifiable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "swp_article_seo_media"]
pub struct ArticleSeoMedia {
    pub id: i32,
    pub image_id: i32,
    pub key: String
}

#[derive(Identifiable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "swp_article_related"]
pub struct RelatedArticle {
    pub id: i32,
    pub article_id: i32,
    pub relates_to_id: i32,
}

#[derive(Identifiable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "swp_article_source"]
pub struct Source {
    pub id: i32,
    pub name: String,
}

#[derive(Identifiable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "swp_article_sources"]
#[primary_key(article_id)]
pub struct ArticleSource {
    pub article_id: i32,
    pub source_id: i32,
}

#[derive(Identifiable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "swp_slideshow"]
pub struct Slideshow {
    pub id: i32,
    pub article_id: i32,
    pub code: String,
}

#[derive(Identifiable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "swp_slideshow_item"]
pub struct SlideshowItem {
    pub id: i32,
    pub article_media_id: i32,
    pub slideshow_id: i32,
    pub position: Option<i32>,
}

impl_load_from_for_diesel_pg! {
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
        i32 -> (swp_author_media, AuthorAvatar),
        i32 -> (swp_article_seo_metadata, ArticleSeoMetadata),
        i32 -> (swp_article_seo_media, ArticleSeoMedia),
        i32 -> (swp_article_related, RelatedArticle),
        i32 -> (swp_slideshow, Slideshow),

        Article.id -> (swp_article_media.article_id, ArticleMedia),
        ArticleMedia.article_id -> (swp_article.id, Article),

        // Image.id -> (swp_article_seo_media.image_id, Image),

        //Image.id -> (swp_image_rendition.image_id, ImageRendition),
        ArticleMedia.id-> (swp_image_rendition.media_id, ImageRendition),
        // ImageRendition.media_id -> (swp_article_media.id, ArticleMedia),

        Statistics.article_id -> (swp_article.id, Article),

        Author.id-> (swp_article_author.author_id, ArticleAuthor),
        Article.id-> (swp_article_author.article_id, ArticleAuthor),

        ArticleAuthor.author_id-> (swp_author.id, Author),
        ArticleAuthor.article_id-> (swp_article.id, Article),

        Keyword.id-> (swp_article_keyword.keyword_id, ArticleKeyword),
        Article.id-> (swp_article_keyword.article_id, ArticleKeyword),

        //Article.id-> (swp_article_statistics.article_id, Article),

        ArticleKeyword.keyword_id-> (swp_keyword.id, Keyword),
        ArticleKeyword.article_id-> (swp_article.id, Article),

        Article.id -> (swp_article_related.article_id, RelatedArticle),

        Source.id-> (swp_article_sources.source_id, ArticleSource),
        Article.id-> (swp_article_sources.article_id, ArticleSource),
        ArticleSource.source_id-> (swp_article_source.id, Source),
        ArticleSource.article_id-> (swp_article.id, Article),

        Article.id -> (swp_slideshow.article_id, Slideshow),

        Slideshow.id -> (swp_slideshow_item.slideshow_id, SlideshowItem),
        //SlideshowItem.slideshow_id -> (swp_slideshow.id, Slideshow),
        //ArticleMedia.id -> (swp_slideshow_item.article_media_id, SlideshowItem),
        
        // Slideshow = ArticleMedia
        // SlideshowItem = ImageRendition

        //ArticleMedia.id -> (swp_image_rendition.media_id, ImageRendition),
        //ImageRendition.media_id -> (swp_article_media.id, ArticleMedia),
        //ArticleMedia.id -> (swp_slideshow_item.article_media_id, SlideshowItem),

        // ArticleMedia.id-> (swp_image_rendition.media_id, ImageRendition),
        // ImageRendition.media_id -> (swp_article_media.id, ArticleMedia),
        
    }
}
