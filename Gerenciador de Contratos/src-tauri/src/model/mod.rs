use mysql_async::{prelude::*, Pool};
use dotenv::dotenv;
use std::env;
use crate::controller::{self, enc_senha};
pub mod endereco;
// crates para envio de email
use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header::ContentType;

/// Estrutura que representa um usuário.
///
/// A estrutura contém os seguintes campos:
/// - nome: Nome completo do usuário.
/// - email: Endereço de email do usuário.
/// - senha: Senha do usuário.
#[derive(Default, Debug)]
pub struct Usuario{ // Objeto de usuário para unificar dados
    nome:String, email:String, senha:String,
}


impl Usuario{
    /// Cria uma nova instância de um usuário.
    ///
    /// # Parâmetros
    /// - nome: Nome completo do usuário.
    /// - email: Endereço de email do usuário.
    /// - senha: Senha do usuário.
    ///
    /// # Retornos
    /// - Usuario: Retorna uma nova instância de `Usuario`.
    pub fn novo_usuario(nome: String, email: String, senha: String) -> Self{
        Usuario {nome, email, senha}
    }

    /// Obtém o nome do usuário.
    ///
    /// # Retornos
    /// - &str: Retorna uma referência para o nome do usuário.
    pub fn _get_nome(&mut self) -> &str{
        return &self.nome;
    }

    /// Obtém o email do usuário.
    ///
    /// # Retornos
    /// - &str: Retorna uma referência para o email do usuário.
    pub fn _get_email(&mut self) -> &str{
        return &self.email;
    }

    /// Obtém a senha (hash) do usuário.
    ///
    /// # Retornos
    /// - &str: Retorna uma referência para a senha (hash) do usuário.
    pub fn get_hash(&mut self) -> &str{
        return &self.senha;
    }

    pub fn get_all(&mut self) -> (&String, &String, &String){
        return (&self.nome, &self.email, &self.senha)
    }

    pub async fn ja_cadastrado(&self) -> bool{
        let pool = controller::cria_pool().await.unwrap();
        let email = busca_email(&pool, &self.email).await;
        match email{
            Ok(_ok) => {
                return true
            },
            Err(_e) => {
                return false
            }
        }
    }
}

/// Cria uma pool de conexões com o banco de dados usando as credenciais do arquivo .env.
///
/// # Retornos
/// - Result<Pool, mysql_async::Error>: Retorna Ok(pool) se a pool for criada com sucesso, 
///   ou Err(mysql_async::Error) se houver um erro na criação da pool.
pub async fn create_pool() -> Result<Pool, mysql_async::Error> {
    dotenv().ok();
    let db_host = env::var("DB_HOST")
        .expect("DB_HOST não definido no arquivo .env");
    let db_user = env::var("DB_USER")
        .expect("DB_USER não definido no arquivo .env");
    let db_password = env::var("DB_PASSWORD")
        .expect("DB_PASSWORD não definido no arquivo .env");
    let db_name = env::var("DB_NAME")
        .expect("DB_NAME não definido no arquivo .env");
    
    let url = format!("mysql://{}:{}@{}/{}", db_user, db_password, db_host, db_name);
    let pool = Pool::from_url(url);
    pool
}

/// Insere um novo usuário no banco de dados.
///
/// # Parâmetros
/// - pool: Pool de conexões com o banco de dados.
/// - nome: Nome completo do usuário.
/// - email: Endereço de email do usuário.
/// - senha: Senha do usuário.
/// - email_rep: Referência mutável para um contador de emails repetidos.
///
/// # Retornos
/// - Result<(), mysql_async::Error>: Retorna Ok(()) se o usuário for inserido com sucesso,
///   ou Err(mysql_async::Error) se houver um erro na inserção dos dados.
pub async fn save_data(pool: &Pool, nome:&str, email: &str, senha: &str) -> Result<bool, mysql_async::Error> {
    let mut conn = pool.get_conn().await?;
    
    let uuid = controller::enc_senha(&email);
    // Se o email não for repetido, crie uma conta nova
    conn.exec_drop(
        "INSERT INTO usuarios (email, nome_completo, senha, UUID) VALUES (:email, :nome_completo, :senha, :uuid)", // Interrogações são substituídas pelos parâmetros
        params! {"email" => email, "nome_completo" => nome, "senha" => senha, "uuid" => uuid} // Parâmetros a serem substituídos na query
    ).await?;
    println!("Insert!");
    Ok(true)
}

/// Verifica se um email já está cadastrado no banco de dados.
///
/// # Parâmetros
/// - pool: Pool de conexões com o banco de dados.
/// - email: Endereço de email a ser verificado.
/// - repetido: Referência mutável para um contador que será incrementado se o email já estiver cadastrado.
///
/// # Retornos
/// - Result<String, mysql_async::Error>: Retorna Ok("Encontrado") se o email for encontrado,
///   ou Err(mysql_async::Error) se houver um erro na verificação.

pub async fn busca_email(pool: &Pool, email:&str) -> Result<String, mysql_async::Error>{
    let mut conn = pool.get_conn().await?; // Conectando no banco
    let email_db: Option<String> = conn.exec_first( // emails_db é um vetor de emails que é adquirido do banco de dados
        "SELECT email FROM usuarios WHERE email = :email",
        params! {"email" => email},
    ).await?;
    let server_error = mysql_async::ServerError{
        code: 1045,  // Código de erro (ex: acesso negado)
        message: "Email não encontrado".to_string(), // Mensagem de erro
        state: "28000".to_string(), // Estado SQL
    };
    match email_db{
        None => {
            return Err(mysql_async::Error::Server(server_error))
        },
        Some(_) => {
            return  Ok(email.to_string());
        }
    }
    
}

/// Verifica a senha de um usuário.
///
/// # Parâmetros
/// - pool: Pool de conexões com o banco de dados.
/// - email: Endereço de email do usuário.
/// - senha: Senha digitada pelo usuário.
/// - senha_correta: Referência mutável para um contador que será incrementado se a senha estiver correta.
///
/// # Retornos
/// - Result<(), mysql_async::Error>: Retorna Ok(()) se a senha for verificada com sucesso,
///   ou Err(mysql_async::Error) se houver um erro na verificação.
pub async fn verifica_senha(pool: &Pool, email:&str, senha:&str) -> Result<Usuario, mysql_async::Error>{
    let mut conn = pool.get_conn().await?;     
    let email_encontrado;
    let server_error = mysql_async::ServerError{
        code: 1045,  // Código de erro
        message: "Email não encontrado".to_string(), // Mensagem de erro
        state: "28000".to_string(), // Estado SQL
    };
    match busca_email(pool, email).await {
        Ok(data) => {
            email_encontrado = data;
        },
        Err(_e) => return Err(mysql_async::Error::Server(server_error)),        
    }
    let senhas_db : Option<String> = conn.exec_first( // senhas_db é um vetor que armazena as senhas dos usuários
        "SELECT senha FROM usuarios WHERE email = :email", // Carrega a senha atual do email selecionado
        params! {"email" => email_encontrado}, // Parâmetro email_encontrado é utilizado para selecionar o email
    ).await?;
    let hash_senha: String = senhas_db.unwrap();
    let hash_dec = controller::dec_senha(senha, hash_senha.to_string()); // Verificando o hash da senha
    let usuario_autenticado = Usuario::novo_usuario("".to_string(), email.to_string(), hash_senha.to_string());
    if hash_dec{ // Se o hash estiver correto, valida o login
        return Ok(usuario_autenticado);
    }
    let senha_error = mysql_async::ServerError{
        code: 1049,  // Código de erro (ex: acesso negado)
        message: "Senha incorreta".to_string(), // Mensagem de erro
        state: "28000".to_string(), // Estado SQL
    };
    Err(mysql_async::Error::Server(senha_error))
}

/// Envia um e-mail de verificação.
///
/// # Parâmetros
/// - email: Endereço de e-mail para onde o e-mail será enviado.
///
/// Esta função configura e utiliza o servidor SMTP do Gmail para enviar um e-mail de verificação com um código.
pub fn envia_email(email: String){
    // carregando as credenciais SMTP
    dotenv().ok();
    let smtp_username = env::var("smtp_username")
        .expect("smtp_username não definido no arquivo .env");
    let smtp_password = env::var("smtp_password")
        .expect("smtp_password não definido no arquivo .env");

    // o servidor SMTP e porta
    let smtp_server = "smtp.gmail.com";
    //let smtp_port = 587;
    // credenciais de autenticação SMTP
    let smtp_credentials = Credentials::new(smtp_username.to_string(), smtp_password.to_string());

    // transporte SMTP
    let smtp_transport = SmtpTransport::starttls_relay(&smtp_server)
        .unwrap()
        .credentials(smtp_credentials)
        .authentication(vec![Mechanism::Plain])
        .build();


    let id = enc_senha(&email);
    let id = id.get(8..12).unwrap().to_string();
    // conteúdo do e-mail
    let code = format!("Seu código de verificação é {}", id);
    let email = Message::builder()
        .from("gerenciadordecontratosgdc@gmail.com".parse().unwrap())
        .to(email.parse().unwrap())
        .subject("Gerenciador de Contratos | Reset de senha")
        .header(ContentType::parse("text/html").unwrap()) // Define o tipo de conteúdo como HTML
        .body(
            format!(
                r#"
                <!DOCTYPE html>
                <html>
                <head>
                    <style>
                        body {{
                            font-family: Arial, sans-serif;
                            margin: 20px;
                        }}
                        .container {{
                            padding: 20px;
                            border: 1px solid #ddd;
                            border-radius: 5px;
                            background-color: #f9f9f9;
                        }}
                        .header {{
                            font-size: 24px;
                            font-weight: bold;
                            color: #333;
                        }}
                        .code {{
                            font-size: 18px;
                            font-weight: bold;
                            color: #007bff;
                            padding: 10px;
                            border: 1px solid #007bff;
                            border-radius: 5px;
                            display: inline-block;
                        }}
                    </style>
                </head>
                <body>
                    <div class="container">
                        <div class="header">Reset de Senha</div>
                        <p>Olá,</p>
                        <p>Você solicitou a redefinição da sua senha. Use o código abaixo para redefinir sua senha:</p>
                        <div class="code">{code}</div>
                        <p>Se você não solicitou isso, por favor ignore este email.</p>
                        <p>Atenciosamente,<br>Equipe do Gerenciador de Contratos</p>
                    </div>
                </body>
                </html>
                "#,
                code = code
            )
        )
        .unwrap();

    // enviar o e-mail usando o transporte SMTP
    match smtp_transport.send(&email) {
        Ok(_) => println!("E-mail enviado com sucesso!"),
        Err(err) => eprintln!("Erro ao enviar e-mail: {:?}", err),
    }
}