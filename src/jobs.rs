use crate::database::{CONN_POOL, CaseResult, Problem, Submission};
use actix_web::{HttpResponse, Responder, get, post, web};
use anyhow::{Error, Result};
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
    let result = do_create_job(job.into_inner()).await;
    match result {
        Ok(job_response) => HttpResponse::Ok().json(job_response),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

async fn do_create_job(job: Job) -> Result<JobResponse> {
    let conn = CONN_POOL.get()?;
    // get problem
    let problem: Problem = conn.query_one(
        "SELECT * FROM problem WHERE id = ?1;",
        [job.problem_id],
        |row| Ok(Problem::from_row(&row)),
    )??;

    let created_time: String = chrono::Utc::now()
        .format("%Y-%m-%dT%H:%M:%S%.3fZ")
        .to_string();

    // insert submission
    let submission = Submission::new(
        job.source_code.clone(),
        job.language.clone(),
        job.user_id,
        job.contest_id,
        job.problem_id,
        problem.cases.len(),
    );
    let submission_id: usize = conn.execute(
        "INSERT INTO submission (source_code, language, user_id, contest_id, problem_id, cases
            VALUES (?, ?, ?, ?, ?, ?)",
        (
            &submission.source_code,
            &submission.language,
            &submission.user_id,
            &submission.contest_id,
            &submission.problem_id,
            &submission.cases.len(),
        ),
    )?;

    // run background task

    let updated_time: String = chrono::Utc::now()
        .format("%Y-%m-%dT%H:%M:%S%.3fZ")
        .to_string();

    Ok(JobResponse {
        id: submission_id as u32,
        created_time,
        updated_time,
        submission: job,
        state: "pending".to_string(),
        result: "pending".to_string(),
        score: 0.0,
        cases: Vec::new(),
    })
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
