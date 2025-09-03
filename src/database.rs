use anyhow::Result;
use lazy_static::lazy_static;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2_sqlite::rusqlite::Row;
use serde::{Deserialize, Serialize};

const DATABASE_PATH: &str = "data/database.db";

lazy_static! {
    pub static ref CONN_POOL: Pool<SqliteConnectionManager> = {
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
pub struct Problem {
    id: u32,
    name: String,
    r#type: String,
    // misc: Option<serde_json::Value>,
    cases: Vec<Case>,
}

impl Problem {
    pub fn from_row(row: &Row) -> Result<Self> {
        Ok(Problem {
            id: row.get(0)?,
            name: row.get(1)?,
            r#type: row.get(2)?,
            cases: serde_json::from_str(&row.get::<usize, String>(3)?)?,
        })
    }
}
