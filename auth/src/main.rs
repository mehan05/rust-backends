use std::{net::Ipv4Addr, net::SocketAddr};

pub mod routes;
pub mod models;
pub mod auth;
pub mod handlers;
pub mod config;
pub mod middleware;
pub mod errors;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let uri  = std::env::var("URI").expect("Cant find env");
    let pool = config::connect_db(&uri.as_str()).await.unwrap();

    // config::migration(&pool).await;

    let route = routes::route::create_route(pool);
   let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED,3000));
   let listener = tokio::net::TcpListener::bind(address).await.unwrap();

   axum::serve(listener,route.into_make_service()).await.unwrap();

}
