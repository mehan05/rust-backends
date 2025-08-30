use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct UserInfo{
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role : String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize,Deserialize,sqlx::FromRow)]
pub struct UserLogin{
    pub email: String,
    pub password: String
}


#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct Claims{
     pub sub:  String,  
    pub exp:  i64,       
    pub iat:   i64,       
    pub role:   String,   
    pub email: String
}
