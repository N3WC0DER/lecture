use std::env;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_user = env::var("DATABASE_USER").expect("DATABASE_USER must be set");
    let database_password = env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    let database_host = env::var("DATABASE_HOST").expect("DATABASE_HOST must be set");

    let database_url = format!(
        "postgresql://{}:{}@{}/{}",
        database_user, database_password, database_host, database_name
    );

    log::debug!("Connection to {}", database_url);

    let manager = ConnectionManager::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("Could not build connection pool")
}
