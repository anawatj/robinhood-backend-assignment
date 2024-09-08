use domain::{job::NewJob, jwt::JWT};
use rocket::{delete, get, post, put, response::status::{ Created, Custom}};
use application::job;
use rocket::serde::json::Json;
use shared::response_models::Response;
#[get("/jobs")]
#[warn(unreachable_patterns)]
pub fn list_job(key:Result<JWT,Response<String>>) -> Result<String,Custom<String>> {
    let results = job::list_jobs(key);
    
    results
}

#[get("/jobs/<job_id>")]
#[warn(unreachable_patterns)]
pub fn get_job(job_id: i32,key:Result<JWT,Response<String>>) -> Result<String, Custom<String>> {
    let result = job::find_one_job(job_id,key);
    result
}

#[post("/jobs", format = "application/json", data = "<job>")]
#[warn(unreachable_patterns)]
pub fn create_job(job: Json<NewJob>,key:Result<JWT,Response<String>>) -> Result<Created<String>,Custom<String>> {
     let result = job::create_job(job,key);
     result
}
#[put("/jobs/<job_id>", format = "application/json", data = "<job>")]
#[warn(unreachable_patterns)]
pub fn update_job(job:Json<NewJob>,job_id:i32,key:Result<JWT,Response<String>>) -> Result<String,Custom<String>>{
    let result = job::update_job(job, job_id,key);
    result
}
#[delete("/jobs/<job_id>")]
#[warn(unreachable_patterns)]
pub fn delete_job(job_id:i32,key:Result<JWT,Response<String>>)->Result<String,Custom<String>>{
    let result = job::delete_job(job_id,key);
    result
}
