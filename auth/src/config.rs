use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};

pub async fn connect_db(uri:&str)->Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error>{
   let connect =  PgPoolOptions::new().max_connections(5).connect(uri).await;

    println!("Connected");
   return connect;
}

pub  async fn migration(pool:&Pool<Postgres>){
        let migrate = sqlx::migrate!("./migrations")
        .run(pool)
        .await.expect("Cant migrate");
        println!("migrated");
        migrate
}