use sqlx::{PgPool, Pool, Postgres};

pub async fn connect(db_url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPool::connect(db_url).await
}
