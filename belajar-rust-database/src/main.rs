fn main() {
    println!("Hello, Database!");
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use sqlx::{postgres::PgPoolOptions, Connection, Error, PgConnection, Pool, Postgres};

    #[tokio::test]
    async fn test_manual_connection() -> Result<(), Error> {
        let url = "postgres://test:test*123@192.168.1.8:5432/test";
        let connection = PgConnection::connect(url).await?;
        connection.close().await?;
        Ok(())
    }

    // POOL ---------------------------------------------------------------------
    async fn get_pool() -> Result<Pool<Postgres>, Error> {
        let url = "postgres://test:test*123@192.168.1.8:5432/test";
        PgPoolOptions::new()
            .max_connections(10)
            .min_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .idle_timeout(Duration::from_secs(60))
            .connect(url)
            .await
    }
    #[tokio::test]
    async fn test_pool_connection() -> Result<(), Error> {
        let pool = get_pool().await?;
        pool.close().await;
        Ok(())
    }
}
