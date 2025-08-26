use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let route01 = Router::new().route("/test",get(test_fn).post(test_fn_post));

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener,route01).await.unwrap();    
}

#[derive(Debug,Serialize,Deserialize)]
struct SampleData{
    id:String,
    name:String,
    age:u64
}


async fn test_fn()->Json<SampleData>{
    println!("test function called");
    Json::from(
        SampleData{
            id:uuid::Uuid::new_v4().to_string(),
            name:"MEhan".to_string(),
            age:20
        }
    )
}

async fn test_fn_post(){
    println!("calling post function");
}
