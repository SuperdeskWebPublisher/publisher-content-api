use rocket::{get, post};
use rocket::response::content;
use rocket::State;
use graphql_api::graphql::schema;
use schema::Query;
use juniper::{EmptyMutation, RootNode};

pub type Schema = RootNode<'static, Query, EmptyMutation<DbConn>>;

#[get("/graphiql")]
pub fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

// #[get("/graphql?<request>")]
// pub fn get_graphql_handler(
//     context: Context,
//     request: juniper_rocket::GraphQLRequest,
//     schema: State<Schema>
// ) -> juniper_rocket::GraphQLResponse {
//     request.execute(&schema, &Context { connection: context })
// }

// #[post("/graphql", data = "<request>")]
// pub fn post_graphql_handler(
//     context: Context,
//     request: juniper_rocket::GraphQLRequest,
//     schema: State<Schema>
// ) -> juniper_rocket::GraphQLResponse {
//     request.execute(&schema, &Context { connection: context })
// }
