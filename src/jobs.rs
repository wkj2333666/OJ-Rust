use actix_web::{Responder, get, post, web};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
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
