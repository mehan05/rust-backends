use anyhow::Ok;
use axum::{body::Body, extract::Request, http::{HeaderMap, StatusCode}, middleware::Next, response::Response};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{auth::jwt::SECRET_KEY, errors::CustomErrors, models::user::Claims};

pub async fn auth_middleware(headers:HeaderMap,mut request:Request,next:Next)->Response{

    let token = get_token(headers);
    let value = validate_token(token).unwrap();
    request.extensions_mut().insert(value);
    let res = next.run(request).await;
        
    return res;

}


pub fn get_token(headers:HeaderMap)->String{
    let token = headers.get("Authorization").unwrap().to_str().unwrap().to_string();
    let token_split = token.strip_prefix("Bearer ").unwrap().to_string();
    token_split
    
}

pub fn validate_token(token:String)->Result<Claims,anyhow::Error>{
    let validate   = Validation::new(jsonwebtoken::Algorithm::HS256);
    let decoded_claims = decode::<Claims>(
        &token.as_str(),
        &DecodingKey::from_secret(SECRET_KEY),
        &validate
    );

    Ok(decoded_claims?.claims)
}