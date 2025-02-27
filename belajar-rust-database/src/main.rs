fn main() {
    println!("Hello, Database!");
}

#[cfg(test)]
mod test {
    use sqlx::{Connection, Error, PgConnection};

    #[tokio::test]
    async fn test_manual_connection() -> Result<(), Error> {
        let url = "postgres://test:test*123@192.168.1.8:5432/test";
        let connection = PgConnection::connect(url).await?;
        connection.close().await?;
        Ok(())
    }
}
