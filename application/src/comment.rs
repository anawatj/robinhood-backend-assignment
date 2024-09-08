use domain::comment::{Comment, NewComment};
use infrastructure::*;
use rocket::http::Status;
use rocket::response::status::{Created, Custom};
use rocket::serde::json::Json;
use shared::response_models::{Response, ResponseBody};
use domain::jwt::JWT;

pub fn find_one_comment(job_id: i32,comment_id:i32,key:Result<JWT,Response<String>>) -> Result<String, Custom<String>> {
    match key {
        Ok(_)=>{
            match comment::find_comment_by_id(job_id,comment_id) {
                Ok(job) => {
                    let response = Response {
                        body: ResponseBody::<Comment>::Data(job),
                    };
                    Ok(serde_json::to_string(&response).unwrap())
                }
                Err(err) => match err {
                    diesel::result::Error::NotFound => {
                        let response = Response {
                            body: ResponseBody::<Comment>::Message(format!("Error Not Found Comment")),
                        };
                        Err(Custom(
                            Status { code: Status::NotFound.code },
                            serde_json::to_string(&response).unwrap(),
                        ))
                    }
                    _ => {
                        let response = Response {
                            body: ResponseBody::<Comment>::Message(format!("Database error - {}", err)),
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

pub fn list_comments(job_id:i32,key:Result<JWT,Response<String>>) -> Result<String, Custom<String>> {
    match key {
        Ok(_)=>{
            match comment::find_all_comment(job_id) {
                Ok(mut comments) => {
                    match comments.len()==0 {
                        true=>{
                            let response = Response {
                                body: ResponseBody::<Vec<Comment>>::Message(format!("Error Not Found Comment")),
                            };
                            return Err(Custom(
                                Status { code: Status::NotFound.code },
                                serde_json::to_string(&response).unwrap(),
                            ));
                        },
                        _ =>{
                            comments.sort();
                            let response = Response {
                                body: ResponseBody::<Vec<Comment>>::Data(comments),
                            };
                            Ok(serde_json::to_string(&response).unwrap())
                        }
                        
                    }
                    
                }
                Err(err) => match err {
                    diesel::result::Error::NotFound => {
                        let response = Response {
                            body: ResponseBody::<Vec<Comment>>::Message(format!("Error Not Found Comment")),
                        };
                        return Err(Custom(
                            Status { code: Status::NotFound.code },
                            serde_json::to_string(&response).unwrap(),
                        ));
                    }
                    _ => {
                        let response: Response<_> = Response {
                            body: ResponseBody::<Comment>::Message(format!("Database error - {}", err)),
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
    

pub fn create_comment(comment: Json<NewComment>,job_id:i32,key:Result<JWT,Response<String>>) -> Result<Created<String>, Custom<String>> {
    match key {
        Ok(jwt)=>{
            let user = user::find_user_by_id(jwt.claims.subject_id);
            match user {
                Ok(user)=>{
                    let mut  comment = comment.into_inner();
                    comment.create_by=Some(user.email);
                    comment.job_id=Some(job_id);
                    let errors = validate_comment(comment.clone());
                    match errors.len() > 0 {
                        true => {
                            let message = errors.join(",");
                            let response = Response {
                                body: ResponseBody::<Comment>::Message(message),
                            };
                            Err(Custom(
                                Status { code: Status::BadRequest.code },
                                serde_json::to_string(&response).unwrap(),
                            ))
                        }
                        _ => match comment::create_comment(comment) {
                            Ok(job) => {
                                let response = Response {
                                    body: ResponseBody::<Comment>::Data(job),
                                };
                                Ok(Created::new("").tagged_body(serde_json::to_string(&response).unwrap()))
                            }
                            Err(err) => match err {
                                _ => {
                                    let response: Response<_> = Response {
                                        body: ResponseBody::<Comment>::Message(format!("Database error - {}", err)),
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
                        body: ResponseBody::<Comment>::Message("Unauthorize".to_string()),
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

pub fn update_comment(comment: Json<NewComment>, job_id: i32,comment_id:i32,key:Result<JWT,Response<String>>) -> Result<String, Custom<String>> {
    match key {
        Ok(_)=>{
            let comment = comment.into_inner();
            let errors = validate_comment(comment.clone());
            match errors.len()>0 {
                true=>{
                    let message = errors.join(",");
                    let response = Response {
                        body: ResponseBody::<Comment>::Message(message),
                    };
                    Err(Custom(
                        Status { code: Status::BadRequest.code },
                        serde_json::to_string(&response).unwrap(),
                    ))
                },
                _ =>{
                    match comment::update_comment(comment, job_id, comment_id) {
                        Ok(comment) => {
                            let response = Response {
                                body: ResponseBody::<Comment>::Data(comment),
                            };
                            Ok(serde_json::to_string(&response).unwrap())
                        }
                        Err(err) => match err {
                            _ => {
                                let response = Response {
                                    body: ResponseBody::<Comment>::Message(format!("Database error - {}", err)),
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
pub fn delete_comment(job_id: i32,comment_id:i32,key:Result<JWT,Response<String>>) -> Result<String, Custom<String>> {
    match key {
        Ok(_)=>{
            match comment::delete_comment(job_id, comment_id){
                Ok(_) => {
                    let response = Response {
                        body: ResponseBody::<String>::Data("Success".to_string()),
                    };
                    Ok(serde_json::to_string(&response).unwrap())
                }
                Err(err) => match err {
                    _ => {
                        let response = Response {
                            body: ResponseBody::<Comment>::Message(format!("Database error - {}", err)),
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

pub fn validate_comment(comment: NewComment) -> Vec<String> {
    let errors: Vec<Option<String>> = Vec::from([
       
        match comment.description == "" {
            true => Some("comment description is required".to_string()),
            _ => None,
        },
    ]);
    errors.into_iter().flatten().collect()
}