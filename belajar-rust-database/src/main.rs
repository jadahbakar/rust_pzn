fn main() {
    println!("Hello, Database!");
}

#[cfg(test)]
mod test {
    use chrono::{Local, NaiveDateTime};
    use sqlx::postgres::{PgPoolOptions, PgRow};
    use sqlx::prelude::FromRow;
    use sqlx::{Connection, Error, PgConnection, Pool, Postgres, Row};
    use std::result;
    use std::time::Duration;

    #[tokio::test]
    async fn test_manual_connection() -> Result<(), Error> {
        let url = "postgres://test:test*123@192.168.1.8:5432/test";
        let connection = PgConnection::connect(url).await?;
        connection.close().await?;
        Ok(())
    }

    // POOL ---------------------------------------------------------------------
    async fn get_pool() -> Result<Pool<Postgres>, Error> {
        let url = "postgres://test:test*123@localhost:5432/test";
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

    // EXECUTE QUERY ---------------------------------------------------------------------
    #[tokio::test]
    async fn test_execute() -> Result<(), Error> {
        let pool = get_pool().await?;
        sqlx::query("INSERT INTO category(id, name, description) VALUES ('A1','test 1', 'test 1')")
            .execute(&pool)
            .await?;
        Ok(())
    }

    // PREPARE QUERY ---------------------------------------------------------------------
    #[tokio::test]
    async fn test_prepare_statement() -> Result<(), Error> {
        let pool = get_pool().await?;
        sqlx::query("INSERT INTO category(id, name, description) VALUES ($1,$2,$3)")
            .bind("B1")
            .bind("TEST B1")
            .bind("TEST B1")
            .execute(&pool)
            .await?;
        Ok(())
    }

    // QUERY SQL ---------------------------------------------------------------------
    /*
    fetch_optional(): Result<Option>, jika menghasilkan satu data atau kosong
    fetch_one(): Result, jika menghasilkan satu data, jika tidak maka akan error
    fetch_all(): Vec<Result>, untuk data banyak
    fetch(): Stream<Result>, untuk mengambil data dalam bentuk stream (lazy)
    */

    #[tokio::test]
    async fn test_fetch_optional() -> Result<(), Error> {
        let pool = get_pool().await?;
        let result = sqlx::query("SELECT * FROM category WHERE id = $1")
            .bind("A1")
            .fetch_optional(&pool)
            .await?;

        if let Some(row) = result {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let desc: String = row.get("description");
            println!("id: {}, name: {}, description: {}", id, name, desc);
        } else {
            println!("Data not Found");
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_one() -> Result<(), Error> {
        let pool = get_pool().await?;
        let result = sqlx::query("SELECT * FROM category WHERE id = $1")
            .bind("A1")
            .fetch_one(&pool)
            .await?;

        let id: String = result.get("id");
        let name: String = result.get("name");
        let desc: String = result.get("description");
        println!("id: {}, name: {}, description: {}", id, name, desc);
        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_all() -> Result<(), Error> {
        let pool = get_pool().await?;
        let result = sqlx::query("SELECT * FROM category")
            .fetch_all(&pool)
            .await?;

        for row in result {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let desc: String = row.get("description");
            println!("id: {}, name: {}, description: {}", id, name, desc);
        }
        Ok(())
    }

    // FETCH ---------------------------------------------------------------------
    /*
        Khusus untuk method fetch(), hasil return dari method nya berupa Stream (versi async dari Iterator)
        Oleh karena itu, kita perlu menambah library futures untuk mengambil data di Stream tersebut
        Silahkan tambahkan library futures menggunakan perintah :
        cargo add futures
    */

    use futures::TryStreamExt;
    #[tokio::test]
    async fn test_fetch() -> Result<(), Error> {
        let pool = get_pool().await?;
        let mut result = sqlx::query("SELECT * FROM category").fetch(&pool);

        while let Some(row) = result.try_next().await? {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let desc: String = row.get("description");
            println!("id: {}, name: {}, description: {}", id, name, desc);
        }
        Ok(())
    }

    // RESULT MAPPING ---------------------------------------------------------------------

    #[derive(Debug, FromRow)]
    struct Category {
        id: String,
        name: String,
        description: String,
    }

    #[tokio::test]
    async fn test_result_mapping() -> Result<(), Error> {
        let pool = get_pool().await?;
        let result = sqlx::query("SELECT * FROM category")
            .map(|row: PgRow| Category {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
            })
            .fetch_all(&pool)
            .await?;
        for category in result {
            println!("{:?}", category);
        }
        Ok(())
    }

    // AUTOMATIC RESULT MAPPING ---------------------------------------------------------------------
    #[tokio::test]
    async fn test_auto_result_mapping() -> Result<(), Error> {
        let pool = get_pool().await?;
        // let result: Vec<Category> = sqlx::query_as("SELECT * FROM category")
        //     .fetch_all(&pool)
        //     .await?;

        let result = sqlx::query_as("SELECT * FROM category")
            .fetch_all(&pool)
            .await?;

        for category in result {
            println!("{:?}", category);
        }
        Ok(())
    }

    // DATA TYPE ---------------------------------------------------------------------

    /*
    CREATE TABLE brands (
        id TEXT PRIMARY KEY,
        name TEXT,
        description TEXT,
        created_at timestamp default CURRENT_TIMESTAMP,
        updated_at timestamp default CURRENT_TIMESTAMP
    );
    */

    #[tokio::test]
    async fn test_insert_brands() -> Result<(), Error> {
        let pool = get_pool().await?;
        sqlx::query(
            "INSERT INTO brands(id,name,description,created_at,updated_at) VALUES ($1,$2,$3,$4,$5)",
        )
        .bind("1")
        .bind("Satu")
        .bind("S A T U ")
        .bind(Local::now().naive_local())
        .bind(Local::now().naive_local())
        .execute(&pool)
        .await?;

        Ok(())
    }

    #[derive(Debug, FromRow)]
    struct Brand {
        id: String,
        name: String,
        description: String,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
    }

    #[tokio::test]
    async fn test_get_brands() -> Result<(), Error> {
        let pool = get_pool().await?;
        let data: Vec<Brand> = sqlx::query_as("SELECT * FROM brands")
            .fetch_all(&pool)
            .await?;

        for brand in data {
            println!("{:?}", brand);
        }
        Ok(())
    }

    // TRANSACTION ---------------------------------------------------------------------
    #[tokio::test]
    async fn test_transaction() -> Result<(), Error> {
        let pool = get_pool().await?;
        let mut transaction = pool.begin().await?;

        sqlx::query(
            "INSERT INTO brands(id,name,description,created_at,updated_at) VALUES ($1,$2,$3,$4,$5)",
        )
        .bind("2")
        .bind("Dua")
        .bind("D U A")
        .bind(Local::now().naive_local())
        .bind(Local::now().naive_local())
        .execute(&mut *transaction)
        .await?;

        sqlx::query(
            "INSERT INTO brands(id,name,description,created_at,updated_at) VALUES ($1,$2,$3,$4,$5)",
        )
        .bind("3")
        .bind("Tiga")
        .bind("T I G A")
        .bind(Local::now().naive_local())
        .bind(Local::now().naive_local())
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;
        Ok(())
    }

    // TRANSACTION ---------------------------------------------------------------------
    /*
    CREATE TABLE sellers (
        id serial primary key,
        name TEXT
    );
    */
    #[tokio::test]
    async fn test_auto_increment() -> Result<(), Error> {
        let pool = get_pool().await?;
        let res = sqlx::query("INSERT INTO sellers(name) VALUES ($1) RETURNING id")
            .bind("Seller Satu")
            .fetch_one(&pool)
            .await?;
        let id: i32 = res.get("id");
        println!("{:?}", id);
        Ok(())
    }

    #[tokio::test]
    async fn test_auto_increment_with_transaction() -> Result<(), Error> {
        let pool = get_pool().await?;
        let mut trx = pool.begin().await?;

        sqlx::query("INSERT INTO sellers(name) VALUES ($1)")
            .bind("Seller Dua")
            .execute(&mut *trx)
            .await?;

        let data: PgRow = sqlx::query("SELECT lastval() as id")
            .fetch_one(&mut *trx)
            .await?;

        let id: i32 = data.get_unchecked("id");
        println!("Id Seller : {:?}", id);

        trx.commit().await?;
        Ok(())
    }
}
