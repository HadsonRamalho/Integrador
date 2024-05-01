use mysql_async::{Pool, prelude::*};
use dotenv::dotenv;
use std::env;

pub async fn create_pool() -> Result<Pool, mysql_async::Error> {
    dotenv().ok();
    let db_host = env::var("DB_HOST").expect("DB_HOST não definido no arquivo .env");
    let db_user = env::var("DB_USER").expect("DB_USER não definido no arquivo .env");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD não definido no arquivo .env");
    let db_name = env::var("DB_NAME").expect("DB_NAME não definido no arquivo .env");
    println!("{db_host}, {db_user}, {db_password}, {db_name}");
    
    let url = format!("mysql://{}:{}@{}/{}", db_user, db_password, db_host, db_name);
    let pool = Pool::from_url(url);
    pool
}

pub async fn insert_data(pool: &Pool, email: &str) -> Result<(), mysql_async::Error> {
    let mut conn = pool.get_conn().await?;
    conn.exec_drop(
        "INSERT INTO usuarios (email) VALUES (?)",
        (email,)
    ).await?;
    Ok(())
}