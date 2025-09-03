use serde::{Deserialize, Serialize};

const DATABASE_PATH: &str = "data/database.db";

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
