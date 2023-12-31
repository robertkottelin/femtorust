use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite, Row};
use std::time::Instant;


async fn create_database(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS data (
            id TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            created TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn insert_data(pool: &Pool<Sqlite>, id: &str, value: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO data (id, value) VALUES (?, ?)")
        .bind(id)
        .bind(value)
        .execute(pool)
        .await?;

    Ok(())
}

async fn get_value_by_id(pool: &Pool<Sqlite>, id: &str) -> Result<String, sqlx::Error> {
    let row = sqlx::query("SELECT value FROM data WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;

    let value: String = row.try_get("value")?;

    Ok(value)
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let database_url = "sqlite:femtorust.db";
    let pool = SqlitePoolOptions::new().connect(&database_url).await?;
    create_database(&pool).await?;

    let start = Instant::now();

    for i in 1..=1000 {
        let id = i.to_string();
        let value = format!("HELLO WORLD {}", i);
        insert_data(&pool, &id, &value).await?;
    }

    for i in 1..=1000 {
        let id = i.to_string();
        let value = get_value_by_id(&pool, &id).await?;
        println!("Value for ID {} is {}", i, value);
    }

    let duration = start.elapsed();
    println!("Total execution time: {:?}", duration);

    Ok(())
}