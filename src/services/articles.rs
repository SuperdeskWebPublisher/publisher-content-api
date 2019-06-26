use diesel;
use diesel::prelude::*;
use graphql_api::models::articles;
use graphql_api::*;
use articles::Article;

// use rocket_contrib::json::Json;
pub fn all(conn: DbConn) -> Vec<Article> {
    use graphql_api::schema::swp_article::dsl::*;

    swp_article
        .limit(10)
        .load::<Article>(&*conn)
        .expect("Error loading posts")
}
