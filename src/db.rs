use std::env;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    log::debug!("Connection to {}", database_url);

    let manager = ConnectionManager::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("Could not build connection pool")
}
