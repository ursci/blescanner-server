//! These are the configuration utils for the DB connection handling and migrations

use diesel::r2d2::{self, ConnectionManager};
use diesel::{prelude::PgConnection, Connection, ConnectionError};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Run pedning migration
pub fn run_migrations(conn: &PgConnection) {
    let _ = diesel_migrations::run_pending_migrations(&*conn);
}

/// Handling connection manager for respective database environment
pub fn establish_connection() -> Pool {
    dotenv::dotenv().ok();

    let database_url;

    if cfg!(test) {
        database_url = dotenv::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);

        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create test DB pool.");

        run_migrations(&pool.get().unwrap());

        pool
    } else {
        database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(&database_url);

        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create DB pool.")
    }
}

#[allow(dead_code)]
pub fn establish_test_connection() -> Result<PgConnection, ConnectionError> {
    let database_url = dotenv::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    match PgConnection::establish(&database_url) {
        Ok(value) => Ok(value),
        Err(e) => {
            println!("Could not connect to PostgreSQL.");
            println!("Error connecting to {}", database_url);
            Err(e)
        }
    }
}
