#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
// #[macro_use] extern crate juniper;
// #[macro_use] extern crate juniper_rocket;
#[macro_use] extern crate diesel;

// #[macro_use] 
// extern crate diesel;
// extern crate dotenv;

use rocket::routes;
use graphql_api::*;
use diesel::prelude::*;
use self::models::*;

// pub mod graphql;
pub mod routes;
//pub mod services;


 use graphql_api::*;

//  use self::routes::*;
// use diesel::prelude::*;
// use rocket::response::content;
// use rocket::State;


// #[get("/")]
// fn index(conn: DbConn) -> &'static str {
//     let articles = services::articles::all(conn);

//     println!("Displaying {} posts", articles.len());
//     for post in articles {
//         println!("{}", post.title);
//         println!("-----------\n");
//         println!("{}", post.body);
//     }

//     "OK"
// }

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes![
            //index,
            routes::graphql::graphiql
        ])
        .launch();
}
