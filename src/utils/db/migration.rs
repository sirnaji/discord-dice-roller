use sqlx::{migrate::MigrateDatabase, sqlite::SqliteQueryResult, Sqlite, SqlitePool};
use std::{env, result::Result};

use crate::utils;

pub async fn init_database()
{
    let db_url =
        env::var("DATABASE_URL").expect("Missing database url. Please check your .env file.");
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false)
    {
        Sqlite::create_database(&db_url).await.unwrap();
    }

    match utils::db::migration::create_schema(&db_url).await
    {
        Ok(_) => println!("Servers database successfuly updated."),
        Err(err) => panic!("{}", err),
    }
}

pub async fn create_schema(db_url: &str) -> Result<SqliteQueryResult, sqlx::Error>
{
    let pool = SqlitePool::connect(db_url).await?;

    let qry = "PRAGMA forgein_keys = ON ;
    CREATE TABLE IF NOT EXISTS server (
        discord_uuid   INTEGER PRIMARY KEY NOT NULL,
        language       TEXT NOT NULL
    )";

    let result = sqlx::query(qry).execute(&pool).await;
    pool.close().await;

    result
}
