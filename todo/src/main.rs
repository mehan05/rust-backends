use std::net::{Ipv4Addr, SocketAddr};

use axum::{routing::{delete, post,get,put}, Router};
 
 pub mod connect_db;

pub mod routes;

pub mod model;
pub mod controllers;

#[tokio::main]
async fn main(){

    dotenvy::dotenv().ok();

    let uri = std::env::var("URI").expect("cant find uri");
    let pool = connect_db::db_connection(&uri.as_str()).await.unwrap_or_else(|_|{
        panic!("Cant connect")
    });
    // connect_db::migrate_func(&pool).await;
    
    let route = routes::route::create_routes(pool);
    
    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED,3000));
    let listner = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listner,route.into_make_service())
    .await.unwrap();
       
}


