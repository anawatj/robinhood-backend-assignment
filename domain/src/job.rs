use crate::schema::jobs;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};
// Queryable will generate the code needed to load the struct from an SQL statement
#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct Job {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub status: String,
    pub create_by: String,
    pub create_date:Option<NaiveDateTime>
}

#[derive(Insertable, Deserialize,Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = jobs)]
pub struct NewJob {
    pub title: String,
    pub description: String,
    pub status: String,
    pub create_by: Option<String>
}