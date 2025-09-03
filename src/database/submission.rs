use serde::{Deserialize, Serialize};

/// TABLE submission
#[derive(Deserialize, Serialize)]
pub struct Submission {
    pub id: Option<u32>,
    pub source_code: String,
    pub language: String,
    pub user_id: u32,
    pub contest_id: u32,
    pub problem_id: u32,
    pub cases: Vec<CaseResult>,
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

impl Submission {
    pub fn new(
        source_code: String,
        language: String,
        user_id: u32,
        contest_id: u32,
        problem_id: u32,
        cases_num: usize,
    ) -> Self {
        Self {
            id: None,
            source_code,
            language,
            user_id,
            contest_id,
            problem_id,
            cases: {
                let mut cases = Vec::with_capacity(cases_num + 1);
                for i in 0..=cases_num {
                    cases.push(CaseResult::new(i as u32));
                }
                cases
            },
        }
    }
}
