#![feature(once_cell)]

use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{PgPool, Pool, Postgres, query};
use std::lazy::SyncOnceCell as OnceCell;
use std::path::PathBuf;
use std::time::Duration;

pub static DATABASE_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

pub async fn load_db() -> Pool<Postgres> {
    let db = PgPoolOptions::new()
        .max_connections(8_192)
        .min_connections(32)
        .max_lifetime(Some(Duration::from_secs(3600)))
        .connect_with(
            PgConnectOptions::new()
                .database("ferris_chat")
                .username("ferris_chat")
                .password("ferris_chat")
                .host("localhost")
                .statement_cache_capacity(1_048_576_usize),
        )
        .await
        .expect("Couldn't connect to DB");

    query!(
        "CREATE TABLE IF NOT EXISTS users (
        id BIGINT PRIMARY KEY,
        name varchar(100),
        guilds BIGINT
    );",
    )
        .execute(&db)
        .await
        .expect("Failed to create the user table!");

    query!(
        "CREATE TABLE IF NOT EXISTS guilds (
        id BIGINT PRIMARY KEY,
        owner_id BIGINT references users,
        name varchar(100),
        channels BIGINT,
        users BIGINT
    );",
    )
        .execute(&db)
        .await
        .expect("Failed to create the guild table!");

    query!(
        "CREATE TABLE IF NOT EXISTS channels (
        id BIGINT PRIMARY KEY,
        name varchar(100)
    );",
    )
        .execute(&db)
        .await
        .expect("Failed to create the channels table!");

    query!(
        "CREATE TABLE IF NOT EXISTS members (
        user_id BIGINT references users,
        guild_id BIGINT references guilds
    );",
    )
        .execute(&db)
        .await
        .expect("Failed to create the members table!");

    query!(
        "CREATE TABLE IF NOT EXISTS messages (
        id BIGINT PRIMARY KEY,
        content varchar(100),
        channel BIGINT references channels,
        reactions BIGINT
    );",
    )
        .execute(&db)
        .await
        .expect("Failed to create the messages table!.");

    DATABASE_POOL
        .set(db.clone())
        .expect("Pool was already set, don't call `load_db` more than once!");

    db
}