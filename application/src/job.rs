use domain::job::{Job, NewJob};
use infrastructure::*;
use rocket::http::Status;
use rocket::response::status::{Created, Custom};
use rocket::serde::json::Json;
use shared::response_models::{Response, ResponseBody};
use domain::jwt::JWT;


pub fn find_one_job(job_id: i32,key:Result<JWT,Response<String>>) -> Result<String, Custom<String>> {
    match key {
        Ok(_)=>{
            match job::find_job_by_id(job_id) {
                Ok(job) => {
                    let response = Response {
                        body: ResponseBody::<Job>::Data(job),
                    };
                    Ok(serde_json::to_string(&response).unwrap())
                }
                Err(err) => match err {
                    diesel::result::Error::NotFound => {
                        let response = Response {
                            body: ResponseBody::<Job>::Message(format!("Error Not Found Job")),
                        };
                        Err(Custom(
                            Status { code: Status::NotFound.code },
                            serde_json::to_string(&response).unwrap(),
                        ))
                    }
                    _ => {
                        let response = Response {
                            body: ResponseBody::<Job>::Message(format!("Database error - {}", err)),
                        };
                        Err(Custom(
                            Status { code: Status::InternalServerError.code },
                            serde_json::to_string(&response).unwrap(),
                        ))
                    }
                },
            }
        },
        Err(err)=>{
            Err(Custom(
                Status { code: Status::Unauthorized.code },
                serde_json::to_string(&err).unwrap(),
            ))
        }
    }
    
}

pub fn list_jobs(key:Result<JWT,Response<String>>) -> Result<String, Custom<String>> {
    match key {
        Ok(_)=>{
            match job::find_all_job() {
                Ok(mut jobs) => {
                    match jobs.len()==0 {
                        true=>{
                            let response = Response {
                                body: ResponseBody::<Vec<Job>>::Message(format!("Error Not Found Job")),
                            };
                            return Err(Custom(
                                Status { code: Status::NotFound.code },
                                serde_json::to_string(&response).unwrap(),
                            ));
                        },
                        _ =>{
                            jobs.sort();
                            let response = Response {
                                body: ResponseBody::<Vec<Job>>::Data(jobs),
                            };
                            Ok(serde_json::to_string(&response).unwrap())
                        }
                    }
                   
                }
                Err(err) => match err {
                    diesel::result::Error::NotFound => {
                        let response = Response {
                            body: ResponseBody::<Vec<Job>>::Message(format!("Error Not Found Job")),
                        };
                        return Err(Custom(
                            Status { code: Status::NotFound.code },
                            serde_json::to_string(&response).unwrap(),
                        ));
                    }
                    _ => {
                        let response = Response {
                            body: ResponseBody::<Job>::Message(format!("Database error - {}", err)),
                        };
                        Err(Custom(
                            Status { code: Status::InternalServerError.code },
                            serde_json::to_string(&response).unwrap(),
                        ))
                    }
                },
            }
        },
        Err(err)=>{
           
            Err(Custom(
                Status { code: Status::Unauthorized.code },
                serde_json::to_string(&err).unwrap(),
            ))
        }
    }
   
}

pub fn create_job(job: Json<NewJob>,key:Result<JWT,Response<String>>) -> Result<Created<String>, Custom<String>> {
    match  key {
        Ok(jwt)=>{
            let user = user::find_user_by_id(jwt.claims.subject_id);
            match user {
                Ok(user)=>{
                    let mut job = job.into_inner();
                    job.create_by=Some(user.email);
                    let errors = validate_job(job.clone());
                    match errors.len() > 0 {
                        true => {
                            let message = errors.join(",");
                            let response = Response {
                                body: ResponseBody::<Job>::Message(message),
                            };
                            Err(Custom(
                                Status { code: Status::BadRequest.code },
                                serde_json::to_string(&response).unwrap(),
                            ))
                        }
                        _ => match job::create_job(job) {
                            Ok(job) => {
                                let response = Response {
                                    body: ResponseBody::<Job>::Data(job),
                                };
                                Ok(Created::new("").tagged_body(serde_json::to_string(&response).unwrap()))
                            }
                            Err(err) => match err {
                                _ => {
                                    let response = Response {
                                        body: ResponseBody::<Job>::Message(format!("Database error - {}", err)),
                                    };
                                    Err(Custom(
                                        Status { code: Status::InternalServerError.code },
                                        serde_json::to_string(&response).unwrap(),
                                    ))
                                }
                            },
                        },
                    }
                },
                _ =>{
                    let response = Response {
                        body: ResponseBody::<Job>::Message("Unauthorize".to_string()),
                    };
                    Err(Custom(
                        Status { code: Status::Unauthorized.code },
                        serde_json::to_string(&response).unwrap(),
                    ))
                }
            }
            
        },
        Err(err)=>{
            Err(Custom(
                Status { code: Status::Unauthorized.code },
                serde_json::to_string(&err).unwrap(),
            ))
        }
    }
   
}

pub fn update_job(job: Json<NewJob>, job_id: i32,key:Result<JWT,Response<String>>) -> Result<String, Custom<String>> {
    match key {
        Ok(_)=>{
            let job = job.into_inner();
            let errors = validate_job(job.clone());
            match errors.len()>0 {
                true=>{
                    let message = errors.join(",");
                    let response = Response {
                        body: ResponseBody::<Job>::Message(message),
                    };
                    Err(Custom(
                        Status { code: Status::BadRequest.code },
                        serde_json::to_string(&response).unwrap(),
                    ))
                },
                _ =>{
                    match job::update_job(job, job_id) {
                        Ok(job) => {
                            let response = Response {
                                body: ResponseBody::<Job>::Data(job),
                            };
                            Ok(serde_json::to_string(&response).unwrap())
                        }
                        Err(err) => match err {
                            _ => {
                                let response = Response {
                                    body: ResponseBody::<Job>::Message(format!("Database error - {}", err)),
                                };
                                Err(Custom(
                                    Status { code: Status::InternalServerError.code },
                                    serde_json::to_string(&response).unwrap(),
                                ))
                            }
                        },
                    }
                }
            }
        },
        Err(err)=>{
            Err(Custom(
                Status { code: Status::Unauthorized.code },
                serde_json::to_string(&err).unwrap(),
            ))
        }
        
    }
    
    
}
pub fn delete_job(job_id: i32,key:Result<JWT,Response<String>>) -> Result<String, Custom<String>> {
    match key {
        Ok(_)=>{
            match comment::delete_comment_by_job_id(job_id) {
                Ok(_)=>{
                    match job::delete_job(job_id) {
                        Ok(_) => {
                            let response = Response {
                                body: ResponseBody::<String>::Data("Success".to_string()),
                            };
                            Ok(serde_json::to_string(&response).unwrap())
                        }
                        Err(err) => match err {
                            _ => {
                                let response = Response {
                                    body: ResponseBody::<Job>::Message(format!("Database error - {}", err)),
                                };
                                Err(Custom(
                                    Status { code: Status::InternalServerError.code },
                                    serde_json::to_string(&response).unwrap(),
                                ))
                            }
                        },
                    }
                },
                Err(err) => match err {
                    _ =>{
                            let response = Response {
                                body: ResponseBody::<Job>::Message(format!("Database error - {}", err)),
                            };
                            Err(Custom(
                                Status { code: Status::InternalServerError.code },
                                serde_json::to_string(&response).unwrap(),
                            ))
                        }
                }
            }
            
        },
        Err(err)=>{
            Err(Custom(
                Status { code: Status::Unauthorized.code },
                serde_json::to_string(&err).unwrap(),
            ))
        }
        
    }
    
}

pub fn validate_job(job: NewJob) -> Vec<String> {
    let errors: Vec<Option<String>> = Vec::from([
        match job.title == "" {
            true => Some("job title is required".to_string()),
            _ => None,
        },
        match job.status == "" {
            true => Some("job status is required".to_string()),
            _ => None,
        },
        match job.description == "" {
            true => Some("job description is required".to_string()),
            _ => None,
        },
    ]);
    errors.into_iter().flatten().collect()
}
