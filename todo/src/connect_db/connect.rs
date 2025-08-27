use sqlx::{postgres::PgPoolOptions,Pool, Postgres};

pub async fn db_connection(uri:&str)->Result<Pool<Postgres>,sqlx::Error>{
    PgPoolOptions::new()
    .max_connections(5)
    .connect(uri)
    .await

}

pub async fn migrate_func(pool:&Pool<Postgres>){
    let migrate = sqlx::migrate!("./migrations")
    .run(pool)
    .await
    .expect("Cant migrate");

    println!("Migrated");

    migrate
}

