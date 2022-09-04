use std::env;

use sqlx::SqlitePool;

#[derive(sqlx::FromRow, Debug)]
pub struct Server
{
    pub discord_uuid: i64,
    pub language: String,
}

pub async fn try_get_server(discord_uuid: i64) -> Option<Server>
{
    let db_url = String::from(
        env::var("DATABASE_URL").expect("Missing database url. Please check your .env file."),
    );
    let instances = SqlitePool::connect(&db_url).await.unwrap();
    let query = "SELECT * FROM server WHERE discord_uuid = ?";

    match sqlx::query_as::<_, Server>(query)
        .bind(discord_uuid.to_string())
        .fetch_one(&instances)
        .await
    {
        Ok(result) =>
        {
            instances.close().await;
            Some(result)
        }

        Err(_) => insert_server(discord_uuid, "en-US".to_string()).await,
    }
}

async fn insert_server(discord_uuid: i64, language: String) -> Option<Server>
{
    let db_url = String::from(
        env::var("DATABASE_URL").expect("Missing database url. Please check your .env file."),
    );
    let instances = SqlitePool::connect(&db_url).await.unwrap();
    let query = "INSERT INTO server (discord_uuid, language) VALUES (?1, ?2)";

    match sqlx::query(query)
        .bind(discord_uuid)
        .bind(language.to_string())
        .execute(&instances)
        .await
    {
        Ok(_) => Some(Server {
            discord_uuid,
            language,
        }),

        Err(err) =>
        {
            println!("update failed: {}", err);
            None
        }
    }
}

pub async fn update_server_language(discord_uuid: i64, new_language: String) -> bool
{
    let db_url = String::from(
        env::var("DATABASE_URL").expect("Missing database url. Please check your .env file."),
    );
    let instances = SqlitePool::connect(&db_url).await.unwrap();
    let query = "UPDATE server SET language = ? WHERE discord_uuid = ?";

    match sqlx::query(query)
        .bind(new_language)
        .bind(discord_uuid)
        .execute(&instances)
        .await
    {
        Ok(_) => true,

        Err(err) =>
        {
            println!("sql update failed: {}", err);
            false
        }
    }
}
