use sqlx::{PgPool, Row};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenvy::dotenv().ok();

    let puri: String = std::env::var("URI").expect("URI not set");
    let pool = sqlx::postgres::PgPool::connect(&puri).await?;


    // sqlx::migrate!("./migrations").run(&pool).await?;


    // let res = sqlx::query("SELECT 1+1 as sum").fetch_one(&pool).await?;

    // let ans:i32 = res.get("sum");
    // println!("sum is {}", ans);
    let datas = Data{
        id:1,
        name:"me".to_string(),
        email:"me@me.com".to_string(),
    };

    create(&datas,&pool).await?;
    Ok(())
}

struct Data{
    id:i32,
    name:String,
    email:String,
}



async fn create(data:&Data,pool:&PgPool)->Result<(),Box<dyn std::error::Error>>{

    let query = "INSERT INTO users VALUES($1,$2,$3)";

    sqlx::query(query)
        .bind(&data.id)
        .bind(&data.name)
        .bind(&data.email)
        .execute(pool)
        .await?;

        
        Ok(())
}