#[macro_use] extern crate rocket;
use api::{job,comment,user};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/v1", routes![
            //api/v1/jobs
           job::list_job,
           job::get_job,
           job::create_job,
           job::update_job,
           job::delete_job,
           comment::list_comment,
           comment::get_comment,
           comment::create_comment,
           comment::update_comment,
           comment::delete_comment,
           user::sign_up,
           user::log_in
        ])
}