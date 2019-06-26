use juniper::{Context as JuniperContext, FieldResult, FieldError};
use graphql_api::models::articles;
use articles::Article;

pub struct Context {
    pub connection: DbConn
}

impl JuniperContext for Context {}

pub struct Query;

#[juniper::object(
    Context = Context,
)]
impl Query {

    fn apiVersion() -> &str {
        "1.0"
    }

    fn article(context: &Context, id: String) -> FieldResult<Article> {
        let connection = context.connection;
        //let article = articles::find(&id, connection)?;

        Ok(article)
    }
}

pub struct MutationRoot;
