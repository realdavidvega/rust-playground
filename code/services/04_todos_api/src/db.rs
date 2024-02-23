use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dropshot::HttpError;
use lazy_static::lazy_static;
use std::time::Duration;
use std::env;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

embed_migrations!();

lazy_static! {
    static ref POOL: Pool = {
        let database_url = match env::var("DATABASE_URL") {
            Ok(v) => v,
            Err(_) => panic!("Provide DATABASE_URL")
        };

        let timeout = Duration::from_millis(120000);
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::builder()
            .connection_timeout(timeout)
            .build(manager)
            .expect("Failed to create database pool!")
    };
}

pub fn init() {
    lazy_static::initialize(&POOL);
    let conn = connection().expect("Failed to get db connection");
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout())
        .expect("Failed to run migrations!");
}

pub fn connection() -> Result<DbConnection, HttpError> {
    POOL.get()
        .map_err(|e| HttpError::for_internal_error(format!("Failed getting db connection: {}", e)))
}
