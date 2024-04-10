use bcrypt;

pub fn hash(s :&str )-> String{
    bcrypt::hash(s, 4).unwrap()
}

pub fn validate(hashed_str : &str , s : &str) -> bool{
    bcrypt::verify(s,hashed_str).unwrap()
}