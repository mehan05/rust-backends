use jsonwebtoken::*;

use crate::models::user::Claims;



#[derive(Debug)]
pub enum CustomError{
    InvalidToken(jsonwebtoken::errors::Error),
    TokenExpired
}

impl From<jsonwebtoken::errors::Error> for CustomError{
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        CustomError::InvalidToken(err)
    }
}

pub const SECRET_KEY: &[u8] = b"secret_123";

pub fn generate_token(user_id:  String,role:String,email:String)->Result<String,CustomError>{

    let claims = Claims{
        sub: user_id,
        exp:(chrono::Utc::now()+chrono::Duration::hours(1)).timestamp(),
        iat:chrono::Utc::now().timestamp(),
        role,
        email
    };

   let token =  encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY)).map_err(Into::into);
    println!("{:?}",token);


    token
}


