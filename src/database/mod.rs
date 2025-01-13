use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel:: PgConnection;


pub fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool");
  pool
}