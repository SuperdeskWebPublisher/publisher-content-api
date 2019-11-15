#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate diesel;

mod graphql;
mod db;
mod routes;
mod models;
mod schema;

use crate::graphql::*;

fn main() {
    dotenv::dotenv().ok();
    rocket::ignite()
        .manage(db::db_pool())
        .manage(Schema::new(Query, Mutation))
        .mount("/", routes![
            routes::graphiql,
            routes::post_graphql_handler,
            routes::get_graphql_handler
        ])
        .launch();
}
