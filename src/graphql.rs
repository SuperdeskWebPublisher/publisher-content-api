use juniper::{Executor, Context as JuniperContext, FieldResult, FieldError};
use super::models::Image as ImageModel;
use super::models::Article as ArticleModel;
use super::models::Route as RouteModel;
use super::models::ArticleMedia as ArticleMediaModel;
use juniper_eager_loading::{prelude::*, *};
use juniper_from_schema::graphql_schema_from_file;
use crate::db::DbConn;
use diesel::prelude::*;
use diesel::debug_query;
use chrono::prelude::*;
use diesel::sql_query;
use crate::juniper::LookAheadMethods;

graphql_schema_from_file!("schema.graphql");

pub struct Context {
    pub connection: DbConn
}

impl JuniperContext for Context {}

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

impl ArticleFields for Article {
    fn field_id(&self, _: &Executor<'_, Context>) -> FieldResult<&i32> {
        Ok(&self.article.id)
    }

    fn field_title(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.article.title)
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

    // fn field_published_at(&self, _: &Executor<'_, Context>) -> FieldResult<&Option<DateTime<Utc>>> {
    //     Ok(&self.article.published_at)
    // }

    fn field_route(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Route, Walked>,
    ) -> FieldResult<&Route> {
        Ok(self.route.try_unwrap()?)
    }

    fn field_media(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, ArticleMedia, Walked>,
    ) -> FieldResult<&Vec<ArticleMedia>> {
        Ok(self.media.try_unwrap()?)
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
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Article, Walked>,
    ) -> FieldResult<&Article> {
        Ok(self.article.try_unwrap()?)
    }

    fn field_image(
        &self,
        executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Image, Walked>,
    ) -> FieldResult<&Image> {
        Ok(self.image.try_unwrap()?)
    }
}

impl QueryFields for Query {
    fn field_articles(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, Article, Walked>,
    ) -> FieldResult<Vec<Article>> {
        use crate::schema::*;

        let article_models = swp_article::dsl::swp_article
            .load::<ArticleModel>(&*executor.context().connection)?;
        
        let articles = map_models_to_graphql_nodes(&article_models, &trail, &executor.context().connection)?;

        Ok(articles)
    }
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
}

use juniper_eager_loading::{EagerLoadAllChildren, GraphqlNodeForModel};

fn map_models_to_graphql_nodes<'a, T, M: Clone>(
    models: &[M],
    trail: &QueryTrail<'a, T, Walked>,
    con: &PgConnection,
) -> Result<Vec<T>, diesel::result::Error>
where
    T: EagerLoadAllChildren<QueryTrail<'a, T, Walked>>
        + GraphqlNodeForModel<Model = M, Connection = PgConnection, Error = diesel::result::Error>,
{
    let mut articles = T::from_db_models(models);
    T::eager_load_all_children_for_each(&mut articles, models, con, trail)?;
    Ok(articles)
}
