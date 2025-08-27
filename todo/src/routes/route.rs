use axum::{routing::{delete, get, post, put}, Router};
use sqlx::{Pool, Postgres};
use  crate::controllers::control::*;
pub  fn create_routes(pool:Pool<Postgres>)->Router{
    let route = Router::new()
    .route("/get-all",get(todo_get_all))
    .route("/get/{id}",get(get_todo_by_id))
    .route("/add",post(todo_add))
    .route("/update/{id}",put(todo_update))
    .route("/delete/{id}",delete(todo_delete))
    .with_state(pool);

    route
} 