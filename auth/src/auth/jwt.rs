use jsonwebtoken::*;
use serde::{Deserialize, Serialize};
use crate::models::user::UserRoles;


#[derive(Debug,Serialize,Deserialize)]
pub struct Claims{
     pub sub:  String,  
    pub exp:  i64,       
    pub iat:   i64,       
    pub role:   UserRoles,   
}

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

const SECRET_KEY: &[u8] = b"secret_123";

pub fn generate_token(user_id:  String,role:UserRoles)->Result<String,CustomError>{

    let claims = Claims{
        sub: user_id,
        exp:(chrono::Utc::now()+chrono::Duration::hours(1)).timestamp(),
        iat:chrono::Utc::now().timestamp(),
        role,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY)).map_err(Into::into)
       
}


pub fn validate_token(token:String)->Result<Claims,CustomError>{

    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    let claims_decode = decode::<Claims>(token.as_str(), &DecodingKey::from_secret(SECRET_KEY), &validation)?;

    if claims_decode.claims.iat < chrono::Utc::now().timestamp(){
        return Err(CustomError::TokenExpired)
    }

    Ok(claims_decode.claims)




}