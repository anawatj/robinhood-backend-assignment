use domain::user::{User, NewUser,LoginRequest};
use infrastructure::*;
use rocket::http::Status;
use rocket::response::status::{Created, Custom};
use rocket::serde::json::Json;
use shared::response_models::{Response, ResponseBody};
use shared::password::*;
use domain::jwt::*;
pub fn sign_up(user:Json<NewUser>)->Result<Created<String>,Custom<String>>{
    let user = user.into_inner();
    let errors = validate_user(user.clone());
    match errors.len()>0 {
        true=>{
            let response = Response {
                body: ResponseBody::<User>::Message(errors.join(",")),
            };
            Err(Custom(
                Status { code: 400 },
                serde_json::to_string(&response).unwrap(),
            ))
        },
        _ =>{
            match user::create_user(user) {
                 Ok(user)=>{
                    let response = Response {
                        body: ResponseBody::<User>::Data(user),
                    };
                    Ok(Created::new("").tagged_body(serde_json::to_string(&response).unwrap()))
                },
                Err(err)=>{
                    let response = Response {
                        body: ResponseBody::<User>::Message(format!("Database error - {}", err)),
                    };
                    Err(Custom(
                        Status { code: 500 },
                        serde_json::to_string(&response).unwrap(),
                    ))
                }
            }
        }
    }

}
pub fn log_in(login:Json<LoginRequest>)->Result<String,Custom<String>>{
    let login = login.into_inner();
    let errors = validate_login(login.clone());
    match errors.len()>0 {
        true=>{
            let response = Response {
                body: ResponseBody::<User>::Message(errors.join(",")),
            };
            Err(Custom(
                Status { code: 400 },
                serde_json::to_string(&response).unwrap(),
            ))
        },
        _ =>{
            match user::find_user_by_email(login.email) {
                Ok(user)=>{
                    match user.id>0 {
                        true=>{
                            match verify(login.password, user.password)  {
                                true =>{
                                  match create_jwt(user.id) {
                                      Ok(jwt)=>{
                                        let response = Response {
                                            body: ResponseBody::<String>::Data(jwt.to_string()),
                                        };
                                        Ok(serde_json::to_string(&response).unwrap())
                                      },
                                      _ =>{
                                        let response = Response {
                                            body: ResponseBody::<User>::Message("Login Fail".to_string()),
                                        };
                                        Err(Custom(
                                            Status { code: 401 },
                                            serde_json::to_string(&response).unwrap(),
                                        ))
                                      }
                                  }  
                                },
                                _ =>{
                                    let response = Response {
                                        body: ResponseBody::<User>::Message("Login Fail".to_string()),
                                    };
                                    Err(Custom(
                                        Status { code: 401 },
                                        serde_json::to_string(&response).unwrap(),
                                    ))
                                }
                            }
                        },
                        _ => {
                            let response = Response {
                                body: ResponseBody::<User>::Message("Login Fail".to_string()),
                            };
                            Err(Custom(
                                Status { code: 401 },
                                serde_json::to_string(&response).unwrap(),
                            ))
                        }
                    }
                },
                Err(_)=>{
                    let response = Response {
                        body: ResponseBody::<User>::Message("Login Fail".to_string()),
                    };
                    Err(Custom(
                        Status { code: 401 },
                        serde_json::to_string(&response).unwrap(),
                    ))
                }
            }
        }
    }
}
pub fn validate_login(login:LoginRequest)->Vec<String>{
    let errors:Vec<Option<String>>=Vec::from([
        match login.email=="" {
            true=>Some("email is required".to_string()),
            _ => None 
        },
        match login.password==""{
            true=>Some("password is required".to_string()),
            _=>None
        }
    ]);
    errors.into_iter().flatten().collect()
}
pub fn validate_user(user:NewUser)->Vec<String>{
    let errors: Vec<Option<String>>=Vec::from([
        match  user.email=="" {
            true=>Some("email is required".to_string()),
            _ => None
            
        },
        match user.password==""{
            true=>Some("password is required".to_string()),
            _=>None
        },
        match user.first_name==""{
            true=>Some("first name is required".to_string()),
            _=>None
        },
        match user.last_name==""{
            true=>Some("last name is required".to_string()),
            _=>None
        }
    ]);
    errors.into_iter().flatten().collect()
}