#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

pub mod schema;
pub mod models;
// pub mod graphql;

#[database("db")]
pub struct DbConn(diesel::pg::PgConnection);
