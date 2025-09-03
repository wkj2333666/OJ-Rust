use anyhow::Result;
use lazy_static::lazy_static;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde::{Deserialize, Serialize};

mod problem;
mod submission;
use problem::Problem;
use submission::{CaseResult, Submission};

const DATABASE_PATH: &str = "data/database.db";

lazy_static! {
    pub static ref CONN_POOL: Pool<SqliteConnectionManager> = {
        let manager = SqliteConnectionManager::file(DATABASE_PATH);
        Pool::new(manager).expect("Failed to create database connection pool")
    };
}
