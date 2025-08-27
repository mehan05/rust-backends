use serde::{Serialize, Deserialize};

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Todo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}