use diesel::QueryDsl;
use domain::job::{Job,NewJob};
use diesel::prelude::*;
use diesel::result::Error;
use crate::db::establish_connection;
pub fn find_job_by_id(job_id:i32)->Result<Job,Error>{
    use domain::schema::jobs;
    jobs::table.find(job_id).first::<Job>(&mut establish_connection() )
}
pub fn find_all_job()->Result<Vec<Job>, Error>{
    use domain::schema::jobs;
    jobs::table.select(jobs::all_columns).load::<Job>(&mut establish_connection())
}
pub fn create_job(job:NewJob)->Result<Job, Error>{
    use domain::schema::jobs;
    diesel::insert_into(jobs::table).values(&job).get_result::<Job>(&mut establish_connection())
}
pub fn update_job(job:NewJob,job_id:i32)->Result<Job, Error>{
    use domain::schema::jobs;
    diesel::update(jobs::table).set((jobs::description.eq(job.description),jobs::title.eq(job.title),jobs::status.eq(job.status))).filter(jobs::id.eq(job_id)).get_result::<Job>(&mut establish_connection())
}
pub fn delete_job(job_id:i32)->Result<usize, Error>{
    use domain::schema::jobs;
    diesel::delete(jobs::table).filter(jobs::id.eq(job_id)).execute(&mut establish_connection())
}

