use serde::{Deserialize, Serialize};

/// TABLE submission
#[derive(Deserialize, Serialize)]
pub struct Submission {
    id: u32,
    source_code: String,
    language: String,
    user_id: u32,
    contest_id: u32,
    problem_id: u32,
    cases: Vec<CaseResult>,
}

#[derive(Deserialize, Serialize)]
pub struct CaseResult {
    id: u32,
    result: String,
    time: u32,
    memory: u32,
    info: String,
}

impl CaseResult {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            result: String::from("Waiting"),
            time: 0,
            memory: 0,
            info: String::from(""),
        }
    }
}
