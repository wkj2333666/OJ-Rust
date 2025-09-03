use actix_web::{Responder, get, post, web};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
struct Job {
    source_code: String,
    language: String,
    user_id: u32,
    contest_id: u32,
    problem_id: u32,
}

// #[get("/jobs")]
// async fn get_jobs() -> HttpResponse {
//     HttpResponse::Ok().json(vec![])
// }

#[post("/jobs")]
async fn create_job(job: web::Json<Job>) -> impl Responder {
    let result = do_create_job(job.into_inner());
    match result {
        Ok
    }
}

#[derive(Serialize)]
struct JobResponse {
    id: u32,
    created_time: String,
    updated_time: String,
    submission: Job,
    state: String,
    result: String,
    score: f32,
    cases: Vec<CaseResult>,
}

#[derive(Serialize)]
struct CaseResult {
    id: u32,
    result: String,
    time: u32,
    memory: u32,
    info: String,
}

impl JobResponse {
    fn new(
        job: &Job,
        id: u32,
        created_time: String,
        updated_time: String,
        cases_num: usize,
    ) -> Self {
        Self {
            id,
            created_time,
            updated_time,
            submission: job.clone(),
            state: String::from("Queueing"),
            result: String::from("Waiting"),
            score: 0.0,
            cases: {
                let mut new_vec = Vec::with_capacity(cases_num + 1);
                for _ in 0..=cases_num {
                    new_vec.push(CaseResult::new(id));
                }
                new_vec
            },
        }
    }
}

impl CaseResult {
    fn new(id: u32) -> Self {
        Self {
            id,
            result: String::from("Waiting"),
            time: 0,
            memory: 0,
            info: String::from(""),
        }
    }
}
