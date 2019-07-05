use rocket::{get, post};
use rocket::response::content;
use rocket::State;
use super::graphql::Query;
use juniper::{EmptyMutation, RootNode};
use crate::db::DbConn;
use crate::graphql::Context;

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>>;

#[get("/graphiql")]
pub fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
pub fn get_graphql_handler(
    _conn: DbConn,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &Context { connection: _conn})
}

#[post("/graphql", data = "<request>")]
pub fn post_graphql_handler(
    _conn: DbConn,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &Context { connection: _conn})
}
