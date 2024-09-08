use diesel::QueryDsl;
use domain::comment::{Comment,NewComment};
use diesel::prelude::*;
use diesel::result::Error;
use crate::db::establish_connection;

pub fn find_comment_by_id(job_id:i32,comment_id:i32)->Result<Comment,Error>{
    use domain::schema::comments;
    comments::table.filter(comments::id.eq(comment_id).and(comments::job_id.eq(job_id))).first::<Comment>(&mut establish_connection() )
}
pub fn find_all_comment(job_id:i32)->Result<Vec<Comment>,Error>{
    use domain::schema::comments;
    comments::table.filter(comments::job_id.eq(job_id)).load::<Comment>(&mut establish_connection())
}
pub fn create_comment(comment:NewComment)->Result<Comment, Error>{
    use domain::schema::comments;
    diesel::insert_into(comments::table).values(comment).get_result::<Comment>(&mut establish_connection())

}
pub fn update_comment(comment:NewComment,job_id:i32,comment_id:i32)->Result<Comment, Error>{
    use domain::schema::comments;
    diesel::update(comments::table).set(comments::description.eq(comment.description)).filter(comments::id.eq(comment_id).and(comments::job_id.eq(job_id))).get_result::<Comment>(&mut establish_connection())
}

pub fn delete_comment(job_id:i32,comment_id:i32)->Result<usize, Error>{
    use domain::schema::comments;
    diesel::delete(comments::table).filter(comments::id.eq(comment_id).and(comments::job_id.eq(job_id))).execute(&mut establish_connection())
}
pub fn delete_comment_by_job_id(job_id:i32)->Result<usize, Error>{
    use domain::schema::comments;
    diesel::delete(comments::table).filter(comments::job_id.eq(job_id)).execute(&mut establish_connection())
}