use rocket::serde::Serialize;
#[derive(Serialize,Debug)]
pub enum ResponseBody<T> {
    Message(String),
    Data(T)
}

#[derive(Serialize,Debug)]
#[serde(crate = "rocket::serde")]
pub struct Response<T> {
    pub body: ResponseBody<T>,
}


