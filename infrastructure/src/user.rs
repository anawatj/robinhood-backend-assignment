use diesel::QueryDsl;
use domain::user::{User,NewUser};
use diesel::prelude::*;
use diesel::result::Error;
use shared::password::*;
use crate::db::establish_connection;
pub fn create_user(user:NewUser)->Result<User, Error>{
    use domain::schema::users;
    diesel::insert_into(users::table).values((users::email.eq(user.email),users::password.eq(hash(user.password)),users::first_name.eq(user.first_name),users::last_name.eq(user.last_name))).get_result::<User>(&mut establish_connection())

}
pub fn find_user_by_email(email:String)->Result<User,Error>{
    use domain::schema::users;
    users::table.select(users::all_columns).filter(users::email.eq(email)).first::<User>(&mut establish_connection())
}
pub fn find_user_by_id(user_id:i32)->Result<User,Error>{
    use domain::schema::users;
    users::table.select(users::all_columns).find(user_id).first::<User>(&mut establish_connection())
}