use pwhash::bcrypt;

pub fn hash(password:String)->String{
    bcrypt::hash(password).unwrap()
}
pub fn verify(password:String,hashed_password:String)->bool{
    bcrypt::verify(password, &hashed_password)
}