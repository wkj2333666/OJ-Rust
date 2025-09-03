use lazy_static::lazy_static;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde::{Deserialize, Serialize};

const DATABASE_PATH: &str = "data/database.db";

lazy_static! {
    static ref CONN_POOL: Pool<SqliteConnectionManager> = {
        let manager = SqliteConnectionManager::file(DATABASE_PATH);
        Pool::new(manager).expect("Failed to create database connection pool")
    };
}

#[derive(Deserialize, Serialize)]
struct Case {
    score: u32,
    input_file: String,
    answer_file: String,
    time_limit: u32,
    memory_limit: u32,
}

#[derive(Deserialize, Serialize)]
struct Problem {
    id: u32,
    name: String,
    r#type: String,
    // misc: Option<serde_json::Value>,
    cases: Vec<Case>,
}
