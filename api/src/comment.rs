use domain::comment:: NewComment;
use rocket::{delete, get, post, put, response::status::{ Created, Custom}};
use application::comment;
use rocket::serde::json::Json;
use domain::jwt::JWT;
use shared::response_models::Response;
#[get("/jobs/<job_id>/comments")]
#[warn(unreachable_patterns)]
pub fn list_comment(job_id:i32,key:Result<JWT,Response<String>>) -> Result<String,Custom<String>> {
    let results = comment::list_comments(job_id,key);
    results
}

#[get("/jobs/<job_id>/comments/<comment_id>")]
#[warn(unreachable_patterns)]
pub fn get_comment(job_id: i32,comment_id:i32,key:Result<JWT,Response<String>>) -> Result<String, Custom<String>> {
    let result = comment::find_one_comment(job_id, comment_id,key);
    result
}

#[post("/jobs/<job_id>/comments", format = "application/json", data = "<comment>")]
#[warn(unreachable_patterns)]
pub fn create_comment(comment: Json<NewComment>,job_id:i32,key:Result<JWT,Response<String>>) -> Result<Created<String>,Custom<String>> {
     let result = comment::create_comment(comment,job_id,key);
     result
}
#[put("/jobs/<job_id>/comments/<comment_id>", format = "application/json", data = "<comment>")]
#[warn(unreachable_patterns)]
pub fn update_comment(comment:Json<NewComment>,job_id:i32,comment_id:i32,key:Result<JWT,Response<String>>) -> Result<String,Custom<String>>{
    let result = comment::update_comment(comment, job_id, comment_id,key);
    result
}
#[delete("/jobs/<job_id>/comments/<comment_id>")]
#[warn(unreachable_patterns)]
pub fn delete_comment(job_id:i32,comment_id:i32,key:Result<JWT,Response<String>>)->Result<String,Custom<String>>{
    let result = comment::delete_comment(job_id, comment_id,key);
    result
}