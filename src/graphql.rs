use juniper::{Executor, Context as JuniperContext, FieldResult, FieldError, ID};
use super::models::Image as ImageModel;
use super::models::Article as ArticleModel;
use super::models::Author as AuthorModel;
use super::models::AuthorAvatar as AuthorAvatarModel;
use super::models::Keyword as KeywordModel;
use super::models::Statistics as StatisticsModel;
use super::models::ArticleAuthor as ArticleAuthorModel;
use super::models::ArticleKeyword as ArticleKeywordModel;
use super::models::Route as RouteModel;
use super::models::ArticleMedia as ArticleMediaModel;
use super::models::ImageRendition as ImageRenditionModel;
use super::models::ArticleSeoMetadata as ArticleSeoMetadataModel;
use super::models::ArticleSeoMedia as ArticleSeoMediaModel;
use super::models::RelatedArticle as RelatedArticleModel;
use super::models::ArticleSource as ArticleSourceModel;
use super::models::Source as SourceModel;
use super::models::Slideshow as SlideshowModel;
use super::models::SlideshowItem as SlideshowItemModel;
use juniper_eager_loading::{prelude::*, *};
use juniper_from_schema::graphql_schema_from_file;
use crate::db::{DbConn, DbConnPool};
use diesel::prelude::*;
use diesel::debug_query;
use chrono::prelude::*;
use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
    Outcome, State,
};
use std::fmt;
use std::marker::PhantomData;
use serde::de::{Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};
use base64::{encode, decode};
pub mod generator;

graphql_schema_from_file!("schema.graphql");

pub struct Context {
    pub db_con: DbConn
}

impl JuniperContext for Context {}

impl<'a, 'r> FromRequest<'a, 'r> for Context {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Context, ()> {
        let db_pool = request.guard::<State<DbConnPool>>()?;

        match db_pool.get() {
            Ok(db_con) => Outcome::Success(Context { db_con }),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

pub struct Query;

pub struct Mutation;

impl MutationFields for Mutation {
    fn field_noop(&self, _executor: &Executor<'_, Context>) -> FieldResult<&bool> {
        Ok(&true)
    }
}

#[derive(Clone, Debug, PartialEq, EagerLoading)]
#[eager_loading(
    model = "ArticleModel",
    error = "diesel::result::Error",
    connection = "PgConnection"
)]
pub struct Article {
    article: ArticleModel,
    #[has_one(default)]
    route: HasOne<Route>,
    #[has_many(
        root_model_field = "article_media",
        foreign_key_field = "article_id",
    )]
    media: HasMany<ArticleMedia>,
    #[has_many_through(join_model = "ArticleAuthorModel")]
    authors: HasManyThrough<Author>,
    #[has_many_through(join_model = "ArticleKeywordModel")]
    keywords: HasManyThrough<Keyword>,
    #[option_has_one(
        foreign_key_field = "feature_media",
        root_model_field = "article_media",
    )]
    feature_media: OptionHasOne<Box<ArticleMedia>>,
    #[option_has_one(
        root_model_field = "article_seo_metadata",
    )]
    seo_metadata: OptionHasOne<ArticleSeoMetadata>,
    #[has_many(
        root_model_field = "related_article",
        foreign_key_field = "relates_to_id",
    )]
    related_articles: HasMany<RelatedArticle>,
    #[has_many_through(join_model = "ArticleSourceModel")]
    sources: HasManyThrough<Source>,
    #[has_many(
        root_model_field = "slideshow",
        foreign_key_field = "article_id",
    )]
    slideshows: HasMany<Slideshow>,
}

#[derive(Clone, Debug, PartialEq, EagerLoading)]
#[eager_loading(
    model = "StatisticsModel",
    error = "diesel::result::Error",
    connection = "PgConnection"
)]
pub struct Statistics {
    statistics: StatisticsModel,
}

#[derive(Clone, Debug, PartialEq, EagerLoading)]
#[eager_loading(
    model = "RouteModel",
    error = "diesel::result::Error",
    connection = "PgConnection"
)]
pub struct Route {
    route: RouteModel,
}

#[derive(Clone, Debug, PartialEq, EagerLoading)]
#[eager_loading(
    model = "ArticleMediaModel",
    error = "diesel::result::Error",
    connection = "PgConnection"
)]
pub struct ArticleMedia {
    article_media: ArticleMediaModel,
    #[has_one(default)]
    article: HasOne<Article>,
    #[has_one(default)]
    image: HasOne<Image>,
    feature_media: ArticleMediaModel,
    #[has_many(
        root_model_field = "image_rendition",
        foreign_key_field = "media_id",
    )]
    renditions: HasMany<ImageRendition>,
}

#[derive(Clone, Debug, PartialEq, EagerLoading)]
#[eager_loading(
    model = "ImageModel",
    error = "diesel::result::Error",
    connection = "PgConnection"
)]
pub struct Image {
    image: ImageModel,
}

#[derive(Clone, Debug, PartialEq, EagerLoading)]
#[eager_loading(
    model = "ImageRenditionModel",
    error = "diesel::result::Error",
    connection = "PgConnection"
)]
pub struct ImageRendition {
    #[has_one(
        root_model_field = "article_media"
    )]
    media: HasOne<ArticleMedia>,
    image_rendition: ImageRenditionModel,
    #[has_one(default)]
    image: HasOne<Image>,
}

#[derive(Clone, Debug, PartialEq, EagerLoading)]
#[eager_loading(
    model = "AuthorModel",
    error = "diesel::result::Error",
    connection = "PgConnection"
)]
pub struct Author {
    author: AuthorModel,
    #[option_has_one(
        foreign_key_field = "author_media_id",
        root_model_field = "author_avatar",
    )]
    avatar: OptionHasOne<Box<AuthorAvatar>>,
}

#[derive(Clone, Debug, PartialEq, EagerLoading)]
#[eager_loading(
    model = "AuthorAvatarModel",
    error = "diesel::result::Error",
    connection = "PgConnection"
)]
pub struct AuthorAvatar {
    author_avatar: AuthorAvatarModel,
    #[has_one(default)]
    image: HasOne<Image>
}

#[derive(Clone, Debug, PartialEq, EagerLoading)]
#[eager_loading(
    model = "KeywordModel",
    error = "diesel::result::Error",
    connection = "PgConnection"
)]
pub struct Keyword {
    keyword: KeywordModel
}

#[derive(Clone, Debug, PartialEq, EagerLoading)]
#[eager_loading(
    model = "ArticleSeoMetadataModel",
    error = "diesel::result::Error",
    connection = "PgConnection"
)]
pub struct ArticleSeoMetadata {
    article_seo_metadata: ArticleSeoMetadataModel,
    #[option_has_one(
        foreign_key_field = "seo_meta_media_id",
        root_model_field = "article_seo_media",
    )]
    seo_meta_media: OptionHasOne<Box<ArticleSeoMedia>>,
    #[option_has_one(
        foreign_key_field = "seo_og_media_id",
        root_model_field = "article_seo_media",
    )]
    seo_og_media: OptionHasOne<Box<ArticleSeoMedia>>,
    #[option_has_one(
        foreign_key_field = "seo_twitter_media_id",
        root_model_field = "article_seo_media",
    )]
    seo_twitter_media: OptionHasOne<Box<ArticleSeoMedia>>,
}

#[derive(Clone, Debug, PartialEq, EagerLoading)]
#[eager_loading(
    model = "ArticleSeoMediaModel",
    error = "diesel::result::Error",
    connection = "PgConnection"
)]
pub struct ArticleSeoMedia {
    article_seo_media: ArticleSeoMediaModel,
    #[has_one(default)]
    image: HasOne<Image>
}

#[derive(Clone, Debug, PartialEq, EagerLoading)]
#[eager_loading(
    model = "RelatedArticleModel",
    error = "diesel::result::Error",
    connection = "PgConnection"
)]
pub struct RelatedArticle {
    related_article: RelatedArticleModel,
    #[has_one(default)]
    article: HasOne<Article>,
}

#[derive(Clone, Debug, PartialEq, EagerLoading)]
#[eager_loading(
    model = "SourceModel",
    error = "diesel::result::Error",
    connection = "PgConnection"
)]
pub struct Source {
    source: SourceModel
}

#[derive(Clone, Debug, PartialEq, EagerLoading)]
#[eager_loading(
    model = "SlideshowModel",
    error = "diesel::result::Error",
    connection = "PgConnection"
)]
pub struct Slideshow {
    slideshow: SlideshowModel,
    #[has_one(default)]
    article: HasOne<Article>,
    #[has_many(
        root_model_field = "slideshow_item",
        foreign_key_field = "slideshow_id",
    )]
    items: HasMany<SlideshowItem>,
}

#[derive(Clone, Debug, PartialEq, EagerLoading)]
#[eager_loading(
    model = "SlideshowItemModel",
    error = "diesel::result::Error",
    connection = "PgConnection"
)]
pub struct SlideshowItem {
    slideshow_item: SlideshowItemModel,
    #[has_one(default)]
    slideshow: HasOne<Slideshow>,
    #[has_one(
        foreign_key_field = "article_media_id",
        root_model_field = "article_media"
    )]
    media: HasOne<ArticleMedia>,
}

impl ArticleFields for Article {
    fn field_id(&self, _: &Executor<'_, Context>) -> FieldResult<ID> {
        Ok(ID::new(self.article.id.to_string()))
    }

    fn field_title(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.article.title)
    }

    fn field_status(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.article.status)
    }

    fn field_slug(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.article.slug)
    }

    fn field_lead(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.article.lead)
    }

    fn field_body(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.article.body)
    }

    fn field_comments_count(&self, _: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.article.comments_count)
    }

    fn field_statistics(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Statistics, Walked>,
    ) -> FieldResult<Option<Statistics>> {
        use crate::schema::swp_article_statistics::dsl;
        use crate::schema::swp_article_statistics::columns::article_id;

        let conn = &_executor.context().db_con;
        let statistics = dsl::swp_article_statistics
            .filter(article_id.eq(self.article.id))
            .first::<StatisticsModel>(conn)?;

        Ok(Some(Statistics {
            statistics
        }))
    }

    // fn field_published_at(&self, _: &Executor<'_, Context>) -> FieldResult<&Option<DateTime<Utc>>> {
    //     Ok(&self.article.published_at)
    // }

    fn field_route(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Route, Walked>,
    ) -> FieldResult<&Route> {
        Ok(self.route.try_unwrap()?)
    }

    fn field_media(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, ArticleMedia, Walked>,
    ) -> FieldResult<&Vec<ArticleMedia>> {
        Ok(self.media.try_unwrap()?)
    }

    fn field_authors(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Author, Walked>,
    ) -> FieldResult<&Vec<Author>> {
        Ok(self.authors.try_unwrap()?)
    }

    fn field_keywords(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Keyword, Walked>,
    ) -> FieldResult<&Vec<Keyword>> {
        Ok(self.keywords.try_unwrap()?)
    }

    fn field_extra(&self, _: &Executor<'_, Context>) -> FieldResult<&Option<String>> {
        Ok(&self.article.extra)
    }

    fn field_metadata(&self, _: &Executor<'_, Context>) -> FieldResult<&Option<String>> {
        Ok(&self.article.metadata)
    }

    fn field_feature_media<'a>(
        &self,
        _executor: &Executor<'a, Context>,
        _trail: &QueryTrail<'a, ArticleMedia, Walked>,
    ) -> FieldResult<Option<&ArticleMedia>> {
        let feature_media = self
            .feature_media
            .try_unwrap()?
            .as_ref()
            .map(|boxed| &**boxed);

        Ok(feature_media)
    }

    fn field_seo_metadata(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, ArticleSeoMetadata, Walked>,
    ) -> FieldResult<&Option<ArticleSeoMetadata>> {
        self.seo_metadata.try_unwrap().map_err(From::from)
    }

    fn field_related_articles(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, RelatedArticle, Walked>,
    ) -> FieldResult<&Vec<RelatedArticle>> {
        Ok(self.related_articles.try_unwrap()?)
    }

    fn field_sources(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Source, Walked>,
    ) -> FieldResult<&Vec<Source>> {
        Ok(self.sources.try_unwrap()?)
    }

    fn field_slideshows(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Slideshow, Walked>,
    ) -> FieldResult<&Vec<Slideshow>> {
        Ok(self.slideshows.try_unwrap()?)
    }
}

impl AuthorFields for Author {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.author.id)
    }

    fn field_name(&self, _executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.author.name)
    }

    fn field_role(&self, _executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.author.role)
    }

    fn field_job_title(&self, _executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.author.job_title)
    }

    fn field_biography(&self, _executor: &Executor<'_, Context>) -> FieldResult<&Option<String>> {
        Ok(&self.author.biography)
    }

    fn field_slug(&self, _executor: &Executor<'_, Context>) -> FieldResult<&Option<String>> {
        Ok(&self.author.slug)
    }

    fn field_twitter(&self, _executor: &Executor<'_, Context>) -> FieldResult<&Option<String>> {
        Ok(&self.author.twitter)
    }
    
    fn field_facebook(&self, _executor: &Executor<'_, Context>) -> FieldResult<&Option<String>> {
        Ok(&self.author.facebook)
    }

    fn field_instagram(&self, _executor: &Executor<'_, Context>) -> FieldResult<&Option<String>> {
        Ok(&self.author.instagram)
    }

    fn field_avatar<'a>(
        &self,
        _executor: &Executor<'a, Context>,
        _trail: &QueryTrail<'a, AuthorAvatar, Walked>,
    ) -> FieldResult<Option<&AuthorAvatar>> {
        let avatar = self
            .avatar
            .try_unwrap()?
            .as_ref()
            .map(|boxed| &**boxed);

        Ok(avatar)
    }  
}

impl AuthorAvatarFields for AuthorAvatar {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.author_avatar.id)
    }

    fn field_key(&self, _executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.author_avatar.key)
    }

    fn field_image(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Image, Walked>,
    ) -> FieldResult<&Image> {
        Ok(self.image.try_unwrap()?)
    }
}

impl KeywordFields for Keyword {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.keyword.id)
    }

    fn field_name(&self, _executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.keyword.name)
    }

    fn field_slug(&self, _executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.keyword.slug)
    }
}

impl StatisticsFields for Statistics {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.statistics.id)
    }

    fn field_page_views_number(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.statistics.page_views_number)
    }
}

impl RouteFields for Route {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.route.id)
    }

    fn field_name(&self, _executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.route.name)
    }
}

impl ArticleMediaFields for ArticleMedia {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.article_media.id)
    }

    fn field_key(&self, _executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.article_media.key)
    }

    fn field_body(&self, _executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.article_media.body)
    }

    fn field_description(&self, _executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.article_media.description)
    }

    fn field_located(&self, _executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.article_media.located)
    }

    fn field_by_line(&self, _executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.article_media.by_line)
    }

    fn field_mimetype(&self, _executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.article_media.mimetype)
    }

    fn field_usage_terms(&self, _executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.article_media.usage_terms)
    }

    fn field_article(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Article, Walked>,
    ) -> FieldResult<&Article> {
        Ok(self.article.try_unwrap()?)
    }

    fn field_image(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Image, Walked>,
    ) -> FieldResult<&Image> {
        Ok(self.image.try_unwrap()?)
    }

    fn field_renditions(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, ImageRendition, Walked>,
    ) -> FieldResult<&Vec<ImageRendition>> {
        self.renditions.try_unwrap().map_err(From::from)
    }
}

impl ArticleSeoMetadataFields for ArticleSeoMetadata {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.article_seo_metadata.id)
    }

    fn field_meta_title(&self, _executor: &Executor<'_, Context>) -> FieldResult<&Option<String>> {
        Ok(&self.article_seo_metadata.meta_title)
    }

    fn field_meta_description(&self, _executor: &Executor<'_, Context>) -> FieldResult<&Option<String>> {
        Ok(&self.article_seo_metadata.meta_description)
    }

    fn field_og_title(&self, _executor: &Executor<'_, Context>) -> FieldResult<&Option<String>> {
        Ok(&self.article_seo_metadata.og_title)
    }

    fn field_og_description(&self, _executor: &Executor<'_, Context>) -> FieldResult<&Option<String>> {
        Ok(&self.article_seo_metadata.og_description)
    }
    
    fn field_twitter_title(&self, _executor: &Executor<'_, Context>) -> FieldResult<&Option<String>> {
        Ok(&self.article_seo_metadata.twitter_title)
    }

    fn field_twitter_description(&self, _executor: &Executor<'_, Context>) -> FieldResult<&Option<String>> {
        Ok(&self.article_seo_metadata.twitter_description)
    }

    fn field_seo_meta_media<'a>(
        &self,
        _executor: &Executor<'a, Context>,
        _trail: &QueryTrail<'a, ArticleSeoMedia, Walked>,
    ) -> FieldResult<Option<&ArticleSeoMedia>> {
        let meta_media = self
            .seo_meta_media
            .try_unwrap()?
            .as_ref()
            .map(|boxed| &**boxed);

        Ok(meta_media)
    }
        
    fn field_seo_og_media<'a>(
        &self,
        _executor: &Executor<'a, Context>,
        _trail: &QueryTrail<'a, ArticleSeoMedia, Walked>,
    ) -> FieldResult<Option<&ArticleSeoMedia>> {
        let og_media = self
            .seo_og_media
            .try_unwrap()?
            .as_ref()
            .map(|boxed| &**boxed);

        Ok(og_media)
    }

    fn field_seo_twitter_media<'a>(
        &self,
        _executor: &Executor<'a, Context>,
        _trail: &QueryTrail<'a, ArticleSeoMedia, Walked>,
    ) -> FieldResult<Option<&ArticleSeoMedia>> {
        let twitter_media = self
            .seo_twitter_media
            .try_unwrap()?
            .as_ref()
            .map(|boxed| &**boxed);

        Ok(twitter_media)
    }
}

impl ArticleSeoMediaFields for ArticleSeoMedia {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.article_seo_media.id)
    }

    fn field_key(&self, _executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.article_seo_media.key)
    }

    fn field_image(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Image, Walked>,
    ) -> FieldResult<&Image> {
        Ok(self.image.try_unwrap()?)
    }
}

impl RelatedArticleFields for RelatedArticle {
    fn field_article(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Article, Walked>,
    ) -> FieldResult<&Article> {
        Ok(self.article.try_unwrap()?)
    }
}

impl SlideshowFields for Slideshow {
    fn field_article(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Article, Walked>,
    ) -> FieldResult<&Article> {
        Ok(self.article.try_unwrap()?)
    }

    fn field_code(&self, _executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.slideshow.code)
    }

    fn field_items(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, SlideshowItem, Walked>,
    ) -> FieldResult<&Vec<SlideshowItem>> {
        Ok(self.items.try_unwrap()?)
    }
}

impl SlideshowItemFields for SlideshowItem {
    fn field_slideshow(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Slideshow, Walked>,
    ) -> FieldResult<&Slideshow> {
        Ok(self.slideshow.try_unwrap()?)
    }

    fn field_media(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, ArticleMedia, Walked>,
    ) -> FieldResult<&ArticleMedia> {
        Ok(self.media.try_unwrap()?)
    }

    fn field_position(&self, _executor: &Executor<'_, Context>) -> FieldResult<&Option<i32>> {
        Ok(&self.slideshow_item.position)
    }
}

impl SourceFields for Source {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.source.id)
    }

    fn field_name(&self, _executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.source.name)
    }
}

impl QueryFields for Query {
    fn field_api_version(
        &self,
        _executor: &Executor<'_, Context>
    ) -> FieldResult<String> {
        Ok("1.0".to_string())
    }

    fn field_articles(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, ArticleConnection, Walked>,
        after: Option<Cursor>,
        first: i32,
    ) -> FieldResult<Option<ArticleConnection>> {
        let conn = &executor.context().db_con;
        let articles_connection = Some(articles_connections(after, first, trail, conn)?);

        Ok(articles_connection)
    }
}

fn articles_connections(
    cursor: Option<Cursor>,
    page_size: i32,
    trail: &QueryTrail<'_, ArticleConnection, Walked>,
    conn: &PgConnection,
) -> QueryResult<ArticleConnection> {
    use crate::{models::pagination::*, schema::swp_article};

    let page_size = i64::from(page_size);

    let cursor_value = cursor
        .unwrap_or_else(|| Cursor("MQ==".to_string()))
        .0
        .parse::<String>()
        .expect("invalid cursor");

    let decoded_cursor_value = String::from_utf8(decode(&cursor_value).unwrap()[..].to_vec()).unwrap();
    let page_number = decoded_cursor_value.parse::<i64>().unwrap();

    let val = (page_number + 1).to_string();
    let next_page_cursor = Cursor(encode(&val));

    let base_query = swp_article::table
        .select(swp_article::all_columns)
        .order(swp_article::id);

    let (article_models, total_count) = base_query
        .paginate(page_number)
        .per_page(page_size)
        .load_and_count_pages::<ArticleModel>(conn)?;

    let articles = if let Some(article_trail) = trail.edges().node().walk() {
        map_models_to_graphql_nodes(&article_models, &article_trail, conn)?
    } else {
        vec![]
    };

    let edges = articles
        .into_iter()
        .map(|article| Edge {
            node: article,
            cursor: next_page_cursor.clone(),
        })
        .collect::<Vec<_>>();

    // TODO https://facebook.github.io/relay/graphql/connections.htm#sec-undefined.PageInfo
    // implement before and last params
    let page_info = PageInfo {
        start_cursor: edges.first().map(|edge| edge.cursor.clone()),
        end_cursor: edges.last().map(|edge| edge.cursor.clone()),
        has_next_page: if total_count as i32 > (page_size as i32 * page_number as i32) { true } else { false },
        has_previous_page: if page_number as i32 > 1 && page_size as i32 == total_count as i32 { true } else { false }
    };

    Ok(ArticleConnection {
        edges,
        page_info,
        total_count: total_count as i32,
    })
}

impl ImageFields for Image {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.image.id)
    }

    fn field_asset_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.image.asset_id)
    }

    fn field_file_extension(&self, _executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.image.file_extension)
    }

    fn field_url(&self, _executor: &Executor<'_, Context>) -> FieldResult<String> {
        use crate::{graphql::generator::*};

        Ok(generate_asset_url(&self.image.asset_id, &self.image.file_extension))
    }
}

impl ImageRenditionFields for ImageRendition {
    fn field_id(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.image_rendition.id)
    }

    fn field_width(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.image_rendition.width)
    }

    fn field_height(&self, _executor: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.image_rendition.height)
    }

    fn field_name(&self, _executor: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.image_rendition.name)
    }

    fn field_image(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Image, Walked>,
    ) -> FieldResult<&Image> {
        Ok(self.image.try_unwrap()?)
    }

    fn field_media(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, ArticleMedia, Walked>,
    ) -> FieldResult<&ArticleMedia> {
        Ok(self.media.try_unwrap()?)
    }
}

use juniper_eager_loading::{EagerLoadAllChildren, GraphqlNodeForModel};

fn map_models_to_graphql_nodes<'a, T, M: Clone>(
    models: &[M],
    trail: &QueryTrail<'a, T, Walked>,
    con: &PgConnection,
) -> Result<Vec<T>, diesel::result::Error>
where
    T: EagerLoadAllChildren
        + GraphqlNodeForModel<Model = M, Connection = PgConnection, Error = diesel::result::Error>,
{
    let mut nodes = T::from_db_models(models);
    T::eager_load_all_children_for_each(&mut nodes, models, con, trail)?;
    
    Ok(nodes)
}

pub struct PageInfo {
    start_cursor: Option<Cursor>,
    end_cursor: Option<Cursor>,
    has_next_page: bool,
    has_previous_page: bool,
}

impl PageInfoFields for PageInfo {
    fn field_start_cursor(&self, _: &Executor<'_, Context>) -> FieldResult<&Option<Cursor>> {
        Ok(&self.start_cursor)
    }

    fn field_end_cursor(&self, _: &Executor<'_, Context>) -> FieldResult<&Option<Cursor>> {
        Ok(&self.end_cursor)
    }

    fn field_has_next_page(&self, _: &Executor<'_, Context>) -> FieldResult<&bool> {
        Ok(&self.has_next_page)
    }

    fn field_has_previous_page(&self, _: &Executor<'_, Context>) -> FieldResult<&bool> {
        Ok(&self.has_previous_page)
    }
}

pub struct ArticleConnection {
    edges: Vec<ArticleEdge>,
    page_info: PageInfo,
    total_count: i32,
}

impl ArticleConnectionFields for ArticleConnection {
    fn field_edges(
        &self,
        _: &Executor<'_, Context>,
        _: &QueryTrail<'_, ArticleEdge, Walked>,
    ) -> FieldResult<&Vec<ArticleEdge>> {
        Ok(&self.edges)
    }

    fn field_page_info(
        &self,
        _: &Executor<'_, Context>,
        _: &QueryTrail<'_, PageInfo, Walked>,
    ) -> FieldResult<&PageInfo> {
        Ok(&self.page_info)
    }

    fn field_total_count(&self, _: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.total_count)
    }
}

pub struct Edge<T> {
    node: T,
    cursor: Cursor,
}

pub type ArticleEdge = Edge<Article>;

impl ArticleEdgeFields for ArticleEdge {
    fn field_node(
        &self,
        _: &Executor<'_, Context>,
        _: &QueryTrail<'_, Article, Walked>,
    ) -> FieldResult<&Article> {
        Ok(&self.node)
    }

    fn field_cursor(&self, _: &Executor<'_, Context>) -> FieldResult<&Cursor> {
        Ok(&self.cursor)
    }
}
