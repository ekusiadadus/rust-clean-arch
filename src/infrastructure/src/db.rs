use sqlx::PgPool;

pub async fn establish_connection() -> Result<PgPool, sqlx::Error> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&db_url).await
}
