#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate juniper_rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_contrib;

use rocket::routes;
use diesel::prelude::*;
use db::DbConn;
use crate::graphql::Context;

pub mod graphql;
pub mod db;
pub mod routes;
pub mod models;
pub mod schema;

use routes::Schema;
use graphql::Query;
use juniper::{EmptyMutation, RootNode};

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .manage(Schema::new(
            Query,
            EmptyMutation::<Context>::new(),
        ))
        .mount("/", routes![
            routes::graphiql,
            routes::post_graphql_handler
        ])
        .launch();
}
