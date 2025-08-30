use axum::{middleware, routing::{get, post}, Router};
use sqlx::{Pool, Postgres};
use crate::{handlers::control::*, middleware::{ middleware::auth_middleware}};

pub fn create_route(pool:Pool<Postgres>)->Router{
    let route = Router::new()
    .route("/register-user", post(register_user))
    .route("/login-user", post(login_user).layer(middleware::from_fn(auth_middleware)))
    .with_state(pool);

    route

    
}
