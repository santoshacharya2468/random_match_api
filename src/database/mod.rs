use sqlx::{postgres::PgPoolOptions, Pool, Postgres};




pub async fn establish_connection() -> Pool<Postgres> {
    let db_url = std::env::var("DATABASE_URL").unwrap();
   let pool= PgPoolOptions::new()
    .max_connections(5)
    .connect(&db_url).await.unwrap();
    pool
}