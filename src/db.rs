use rocket_contrib::database;

#[database("db")]
pub struct DbConn(pub diesel::pg::PgConnection);
unsafe impl Sync for DbConn {}
