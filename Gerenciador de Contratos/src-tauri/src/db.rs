use mysql_async::{Pool, prelude::*};
use dotenv::dotenv;
use std::env; // borrow::Borrow,

// Carregando as credenciais do arquivo .env
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

// Função save_data servirá para dar INSERT de um novo usuário no banco de dados
pub async fn save_data(pool: &Pool, email: &str) -> Result<(), mysql_async::Error> {
    let mut conn = pool.get_conn().await?;
    let nulo = ""; // Não to pegando o nome completo do usuário ainda
    
    let mut qtd_users = conn.exec_map( // exec_map retorna um vetor do tipo definido no parâmetro f:
        "SELECT COUNT(UUID) FROM usuarios",
        (),
        |qtd_usuarios:u32| qtd_usuarios , // Um vetor com um único objeto (aqui, ele contém um inteiro)
    ).await?;
    let qtd; // Variável que vai armazenar o retorno do objeto de QuantidadeUsuarios
    qtd = qtd_users.pop();

    conn.exec_drop(
        "INSERT INTO usuarios (email, nome_completo, UUID) VALUES (?, ?, ?)",
        (email, nulo, qtd) // Campos a serem inseridos na tabela
    ).await?;
    Ok(())
}

// Ainda não arrumei isto, mexe não rs
/*
pub async fn email_repetido(pool: &Pool, email:&str) -> Result<(), mysql_async::Error>{
    let mut conn = pool.get_conn().await?;
    let mut emails_db = conn.exec_map(
        "SELECT email FROM usuarios",
        (), |email| VerificacaoEmail { email },
    ).await?;
    for u in emails_db.iter_mut(){
        let email_db = u.retorna_email();
        if email_db == email{
            println!("CONTA JÁ EXISTE");
        } else{
            println!("CONTA CRIADA");
        }
    }
    Ok(())
}*/