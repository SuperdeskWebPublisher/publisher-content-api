use juniper::{Executor, Context as JuniperContext, FieldResult, FieldError};
use super::models::Image as ImageModel;
use super::models::Article as ArticleModel;
use super::models::Route as RouteModel;
use super::models::ArticleMedia as ArticleMediaModel;
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
extern crate base64;
use base64::{encode, decode};

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
}

impl QueryFields for Query {
    fn field_articles(
        &self,
        executor: &Executor<'_, Context>,
        trail: &QueryTrail<'_, ArticleConnection, Walked>,
        after: Option<Cursor>,
        first: i32,
    ) -> FieldResult<Option<ArticleConnection>> {
        use crate::schema::swp_article;
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
        .unwrap_or_else(|| Cursor("Mw==".to_string()))
        .0
        .parse::<String>()
        .expect("invalid cursor");

    let decoded_cursor_value = String::from_utf8(decode(&cursor_value).unwrap()[..].to_vec()).unwrap();
    let page_number = decoded_cursor_value.parse::<i64>().unwrap();

    let val = (page_number + 1).to_string();
    let next_page_cursor = Cursor(encode(&val));

    let (article_models, total_count) = swp_article::table
        .select(swp_article::all_columns)
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

impl Clone for Cursor {
    fn clone(&self) -> Self {
        Cursor(self.0.clone())
    }
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
