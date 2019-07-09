use diesel::{prelude::*, r2d2::ConnectionManager};

pub type DbConnPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConn = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub fn db_pool() -> DbConnPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    r2d2::Pool::builder()
        .max_size(10)
        .build(ConnectionManager::<PgConnection>::new(database_url))
        .expect("failed to create db connection pool")
}
