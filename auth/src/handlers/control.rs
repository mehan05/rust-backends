use axum::{body::Body, extract::{Request, State}, http::StatusCode, Extension, Json};
use serde_json::json;
use sqlx::{error, Pool, Postgres, QueryBuilder};
use crate::{auth::{self, jwt::generate_token, password::hash_password}, errors::CustomErrors, models::user::{Claims, UserInfo, UserLogin}};


pub async fn register_user(State(pool):State<Pool<Postgres>>,Json(payload):Json<UserInfo>)->Result<(StatusCode,Json<serde_json::Value>),CustomErrors>
{   

    if payload.username.is_empty() || payload.email.is_empty() || payload.password.is_empty(){
        return Err(CustomErrors::EmptyPayload)
    }

    let token = generate_token(payload.id.to_string(),payload.role.to_string(),payload.email.to_string());

    let mut query_builder = QueryBuilder::new("
        INSERT INTO users (username,email,password,role) ");
    let role = payload.role.to_lowercase();

    query_builder.push_values([(
        &payload.username,
        &payload.email,
        &payload.password,
        &role
    )], |mut b, (name, email, password, role)|{
        b.push_bind(name);
        b.push_bind(email);
        b.push_bind(hash_password(&password));
        b.push_bind(role);
        
    });

    query_builder.push(" RETURNING *");

    let query  = query_builder.build_query_as::<UserInfo>();

    let result = query.fetch_one(&pool)
    .await.map_err(|e|{
        println!("{:?}",e);
        CustomErrors::InternalServerError
    })?;

            Ok((StatusCode::CREATED,Json(json!({"token":token.unwrap(),"user":result}))))
    
}

#[axum::debug_handler]
pub async fn login_user(Extension(req):Extension<Claims>,State(pool):State<Pool<Postgres>>,Json(payload):Json<UserLogin>)->Result<(StatusCode,Json<serde_json::Value>),CustomErrors>{
    if payload.email.is_empty() || payload.password.is_empty(){
        return Err(CustomErrors::EmptyPayload)
    }

    let token_decoded_val =  req;
    // let token_decoded_val = req.extensions().get::<Claims>();

    if payload.email != token_decoded_val.email{
            return Err(CustomErrors::InvalidUser)
    }

    if !token_decoded_val.sub.is_empty(){
        let val:i32 = token_decoded_val.sub.parse().expect("cant convert into i32");

        println!("{:?}",val);

        let mut query = QueryBuilder::new("SELECT * from users where ");

        query.push("id = ");

        query.push_bind(&val);

        let user = query.build_query_as::<UserInfo>().fetch_one(&pool).await.map_err(|e|{
            println!("{:?}",e);
            CustomErrors::InternalServerError
        })?;

        if let Some(user)=Some(user){
            return Ok((StatusCode::ACCEPTED,Json(json!({"user":user}))))
        }
        else{
            return Err(CustomErrors::NoUserFound)
        }
    }
    else{
        return  Err(CustomErrors::EmptyPayload)
    }

}