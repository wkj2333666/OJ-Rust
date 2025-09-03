use anyhow::Result;
use r2d2_sqlite::rusqlite::Row;
use serde::{Deserialize, Serialize};

/// TABLE problem
#[derive(Deserialize, Serialize)]
pub struct Problem {
    id: u32,
    name: String,
    r#type: String,
    // misc: Option<serde_json::Value>,
    pub cases: Vec<Case>,
}

#[derive(Deserialize, Serialize)]
pub struct Case {
    score: u32,
    input_file: String,
    answer_file: String,
    time_limit: u32,
    memory_limit: u32,
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
