use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct UserInfo{
    pub id: i32,
    pub username: String,
    pub password: String,
    pub role : UserRoles,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize,Debug)]
pub enum UserRoles{
    Admin,
    User,
}