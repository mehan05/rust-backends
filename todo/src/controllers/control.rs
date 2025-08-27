use axum::{extract::{Path, State}, http::StatusCode, Json};
use chrono::Utc;
use sqlx::{Pool, Postgres, QueryBuilder};
use crate::model::model::{self, Todo};


pub async fn todo_add(State(pool):State<Pool<Postgres>>,Json(payload):Json<Todo>)->Result<(StatusCode,Json<Todo>),(StatusCode,String)>{

        let todo = sqlx::query_as::<_,Todo>(
            "INSERT INTO todos (title, description, completed, created_at, updated_at) VALUES ($1,$2,$3,$4,$5) RETURNING 
            id, title, description, completed, created_at, updated_at
            "
        )
         .bind(&payload.title)
         .bind(&payload.description)
         .bind(&payload.completed)
         .bind(Utc::now())
         .bind(Utc::now())
         .fetch_one(&pool)
         .await
         .map_err(|e| 
            
            {
                println!("error is {}",e);
                (StatusCode::INTERNAL_SERVER_ERROR,e.to_string())
            })?;

                

        Ok((StatusCode::CREATED,Json(todo)))

}

pub async fn todo_update(State(pool):State<Pool<Postgres>>,Path(id):Path<i32>,Json(payload):Json<Todo>)->Result<(StatusCode,Json<Todo>),(StatusCode,String)>{
    
    let mut query_builder = QueryBuilder::new("UPDATE todos SET ");

    if let Some(title) = payload.title{
        query_builder.push("title = ");
        query_builder.push_bind(title);
    }

    if let Some(description) = payload.description{
        query_builder.push(", description = ");
        query_builder.push_bind(description);
    }

    if let Some(completed) = payload.completed{
        query_builder.push(", completed = ");
        query_builder.push_bind(completed);
    }

    query_builder.push(", updated_at = ");
    query_builder.push_bind(Utc::now());

    query_builder.push(" WHERE id = ");
    query_builder.push_bind(id);
    query_builder.push(" RETURNING * ");


    let query = query_builder.build_query_as::<Todo>();

    let update_todo = query.fetch_one(&pool)
    .await
    .map_err(|e| {
        println!("error is {}",e);
        (StatusCode::INTERNAL_SERVER_ERROR,e.to_string())
    })?;







    Ok((StatusCode::OK,Json(update_todo)))
}

pub async fn todo_delete(State(pool):State<Pool<Postgres>>,Path(id):Path<i32>)->Result<(StatusCode,Json<Todo>),(StatusCode,String)>{

    let delete_todo = sqlx::query_as::<_,Todo>("DELETE FROM todos WHERE id = $1 RETURNING *")
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        println!("error is {}",e);
        (StatusCode::INTERNAL_SERVER_ERROR,e.to_string())
    })?;


    Ok((StatusCode::OK,Json(delete_todo)))   
}

pub async fn todo_get_all(State(pool):State<Pool<Postgres>>)->Result<(StatusCode,Json<Vec<Todo>>),(StatusCode,String)>{

    let todos = sqlx::query_as::<_,Todo>("SELECT * FROM todos")
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        println!("error is {}",e);
        (StatusCode::INTERNAL_SERVER_ERROR,e.to_string())
    })?;

    Ok((StatusCode::OK,Json(todos)))
}


pub async fn get_todo_by_id(State(pool):State<Pool<Postgres>>,Path(id):Path<i32>)->Result<(StatusCode,Json<Todo>),(StatusCode,String)>{

    let todo = sqlx::query_as::<_,Todo>("SELECT * FROM todos WHERE id = $1")
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        println!("error is {}",e);
        (StatusCode::INTERNAL_SERVER_ERROR,e.to_string())
    })?;

    Ok((StatusCode::OK,Json(todo)))
}