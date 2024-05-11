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
pub async fn save_data(pool: &Pool, email: &str, email_rep: &mut u32) -> Result<(), mysql_async::Error> {
    let mut conn = pool.get_conn().await?;
    let nulo = ""; // Não to pegando o nome completo do usuário ainda
    
    let mut qtd_users = conn.exec_map( // exec_map retorna um vetor do tipo definido no parâmetro f:
        "SELECT COUNT(UUID) FROM usuarios",
        (),
        |qtd_usuarios:u32| qtd_usuarios , // Um vetor com um único objeto (aqui, ele contém um inteiro)
    ).await?;
    let qtd; // Variável que vai armazenar o retorno do objeto de QuantidadeUsuarios
    qtd = qtd_users.pop();

    let mut repetido = 0; // Um iterador que aumenta quando um email repetido é encontrado (Problemas pra usar bool)
    email_repetido(pool, email, &mut repetido).await?;
    *email_rep = repetido;
    if repetido != 0{ // Se o email for repetido, não faça nada
       return Ok(())
    }
    else { // Se o email não for repetido, crie uma conta nova
        conn.exec_drop(
            "INSERT INTO usuarios (email, nome_completo, UUID) VALUES (?, ?, ?)",
            (email, nulo, qtd) // Campos a serem inseridos na tabela
        ).await?;
     
        Ok(())
    }
    
}

// Ainda não arrumei isto, mexe não rs
pub async fn email_repetido(pool: &Pool, email:&str, repetido:&mut u32) -> Result<(), mysql_async::Error>{
    let mut conn = pool.get_conn().await?;
    let mut emails_db = conn.exec_map(
        "SELECT email FROM usuarios",
        (), |email:String| email ,
    ).await?;
    let mut x:u32 = 0;
    for u in emails_db.iter_mut(){
        let email_db = u.as_mut();
        if email_db == email{
            *repetido += 1;
            return Ok(())
        }
        x += 1;
    }
    if *repetido == 0 as u32{
        println!("CONTA CRIADA");
    }
    println!("{}", x);
    Ok(())
}