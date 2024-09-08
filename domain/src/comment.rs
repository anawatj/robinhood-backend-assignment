use crate::schema::comments;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};
// Queryable will generate the code needed to load the struct from an SQL statement


#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd,Selectable)]
pub struct Comment {
    pub id:i32 ,
    pub job_id:i32,
    pub description:String,
    pub create_by:String ,
    pub create_date:Option<NaiveDateTime>
}

#[derive(Insertable, Deserialize,Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = comments)]
pub struct NewComment {
    pub description:String ,
    pub create_by:Option<String> ,
    pub job_id:Option<i32>
    
}