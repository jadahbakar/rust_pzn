fn main() {
    println!("Hello, REDIS!");
}

#[cfg(test)]
mod test {

    use redis::{aio::MultiplexedConnection, AsyncCommands, Client, Commands, RedisError};

    #[test]
    fn test_connection() {
        let mut client = Client::open("redis://localhost:6379/").unwrap();
        let _: () = client.set("name", "Dedy").unwrap();

        let value: String = client.get("name").unwrap();
        println!("{:#?}", value);
    }

    async fn get_client() -> Result<MultiplexedConnection, RedisError> {
        let client = Client::open("redis://localhost:6379/")?;
        client.get_multiplexed_async_connection().await
    }

    #[tokio::test]
    async fn test_async_connection() -> Result<(), RedisError> {
        let mut conn = get_client().await?;
        let _: () = conn.set("name", "eko").await?;
        let value: String = conn.get("name").await?;
        println!("{:#?}", value);
        Ok(())
    }

    // String
    #[tokio::test]
    async fn test_string() -> Result<(), RedisError> {
        let mut conn = get_client().await?
        
        Ok(())
    }
}
