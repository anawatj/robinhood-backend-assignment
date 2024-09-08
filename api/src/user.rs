use domain::user::{NewUser,LoginRequest};
use rocket::{ post,  response::status::{ Created, Custom}};
use application::user;
use rocket::serde::json::Json;


#[post("/users", format = "application/json", data = "<user>")]
pub fn sign_up(user:Json<NewUser>)->Result<Created<String>,Custom<String>>{
    let result = user::sign_up(user);
    result
}
#[post("/login", format = "application/json", data = "<login>")]
pub fn log_in(login:Json<LoginRequest>)->Result<String,Custom<String>>{
    let result = user::log_in(login);
    result
}

