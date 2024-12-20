use crate::controller::usuario::UsuarioInput;
use crate::controller::{self, cria_pool, gera_hash};
use dotenv::dotenv;
use erro::MeuErro;
use mysql_async::{prelude::*, Pool};
use serde::{Deserialize, Serialize};
use std::env;
pub mod endereco;
pub mod locadora;
pub mod locatario;
pub mod maquina;
pub mod socioadm;
pub mod usuario;
pub mod contrato;
pub mod erro;

// crates para envio de email
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::{Message, SmtpTransport, Transport};

/// Estrutura que representa um usuário.
#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Usuario {
    // Objeto de usuário para unificar dados
    pub nome: String,
    pub email: String,
    pub senha: String,
    pub cpf: String,
    pub cnpj: String
}

impl From<UsuarioInput> for Usuario{
    fn from(value: UsuarioInput) -> Self {
        Usuario{
            nome: value.nome,
            email: value.email,
            senha: value.senha1,
            cpf: value.cpf,
            cnpj: value.cnpj
        }
    }
}

impl Usuario {
    pub async fn ja_cadastrado(&self) -> bool {
        let email = busca_email(&self.email).await;
        match email {
            Ok(_ok) => return true,
            Err(_e) => return false,
        }
    }
}

pub async fn create_pool() -> Result<Pool, mysql_async::Error> {
    dotenv().ok();
    let dblocal = match env::var("DB_LOCAL"){
        Ok(dblocal) => {dblocal},
        Err(_e) => {"".to_string()}
    };
    if dblocal == "true"{
        let db_local_password = env::var("DB_LOCAL_PASSWORD").unwrap();
        let url = format!("mysql://root:{}@127.0.0.1:3307/aws", {db_local_password}); // A porta pode ser 3306 em outras máquinas; A senha pode ser diferente
        println!("{}", url);
        let pool = Pool::from_url(url);
        return pool
    }
    let db_host = env::var("DB_HOST").expect("DB_HOST não definido no arquivo .env");
    let db_user = env::var("DB_USER").expect("DB_USER não definido no arquivo .env");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD não definido no arquivo .env");
    let db_name = env::var("DB_NAME").expect("DB_NAME não definido no arquivo .env");
    let url = format!(
        "mysql://{}:{}@{}/{}",
        db_user, db_password, db_host, db_name
    );
    println!("{}", url);
    let pool = Pool::from_url(url);
    pool
}

pub async fn salva_usuario(
    usuario: Usuario
) -> Result<(), mysql_async::Error> {
    let pool = cria_pool().await?;
    let mut conn = pool.get_conn().await?;

    let uuid = controller::gera_hash(&usuario.email);
    // Se o email não for repetido, crie uma conta nova
    let resultado_insert = conn.exec_drop(
        "INSERT INTO usuarios (email, nomecompleto, senha, UUID, cpf, cnpj) VALUES (:email, :nome_completo, :senha, :uuid, :cpf, :cnpj)", // Interrogações são substituídas pelos parâmetros
        params! {"email" => usuario.email, "nome_completo" => usuario.nome, "senha" => usuario.senha, "uuid" => uuid,
        "cpf" => usuario.cpf, "cnpj" => usuario.cnpj} // Parâmetros a serem substituídos na query
    ).await;
    match resultado_insert{
        Ok(_) => {
            println!("Conta criada!");
            return Ok(())
        }, 
        Err(e) => {
            println!("{:?}", e);
            return Err(mysql_async::Error::Other(Box::new(MeuErro::SalvarUsuario)))
        }
    }
}

pub async fn busca_email(email: &str) -> Result<String, mysql_async::Error> {
    let pool = cria_pool().await?;
    let mut conn = pool.get_conn().await?; // Conectando no banco
    let email_db: Option<String> = conn
        .exec_first(
            "SELECT email FROM usuarios WHERE email = :email",
            params! {"email" => email},
        )
        .await?;
    match email_db {
        None => return Err(mysql_async::Error::Other(Box::new(MeuErro::EmailNaoEncontrado))),
        Some(_) => {
            return Ok(email.to_string());
        }
    }
}

pub async fn verifica_senha(
    email: &str,
    senha: &str,
) -> Result<(), mysql_async::Error> {
    let pool = cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let email_encontrado;
    match busca_email(email).await {
        Ok(data) => {
            email_encontrado = data;
        }
        Err(_e) => return Err(mysql_async::Error::Other(Box::new(MeuErro::EmailNaoEncontrado))),
    }
    let senhas_db: Result<Option<String>, mysql_async::Error> =
        conn.exec_first(
            "SELECT senha FROM usuarios WHERE email = :email", // Carrega o hash da senha do email selecionado
            params! {"email" => email_encontrado}, // Parâmetro email_encontrado é utilizado para selecionar o email
        )
        .await;
    let hash_senha = match senhas_db{
        Ok(hash_senha) =>{
            hash_senha
        },
        Err(e) => {
            println!("{:?}", e);
            return Err(mysql_async::Error::Other(Box::new(MeuErro::EmailNaoEncontrado)))
        }
    };
    let hash_senha = match hash_senha{
        Some(hash_senha) => {hash_senha},
        None => {
            return Err(mysql_async::Error::Other(Box::new(MeuErro::HashNaoEncontrado)))
        }
    };
    let hash_dec = controller::verifica_hash(senha, hash_senha.to_string()); // Verificando o hash da senha
    if hash_dec {
        // Se o hash estiver correto, valida o login
        return Ok(());
    }
    Err(mysql_async::Error::Other(Box::new(MeuErro::SenhaIncorreta)))
}

/// Envia um e-mail de verificação.
///
/// # Parâmetros
/// - email: Endereço de e-mail para onde o e-mail será enviado.
///
/// Esta função configura e utiliza o servidor SMTP do Gmail para enviar um e-mail de verificação com um código.
pub fn envia_email(email: String) -> String{
    // carregando as credenciais SMTP
    dotenv().ok();
    let smtp_username =
        env::var("smtp_username").expect("smtp_username não definido no arquivo .env");
    let smtp_password =
        env::var("smtp_password").expect("smtp_password não definido no arquivo .env");

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

    let id = gera_hash(&email);
    let id = id.get(8..12).unwrap().to_string();
    // conteúdo do e-mail
    let code = format!("{}", id);
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
                        <p>Se você não solicitou isso, por favor ignore este e-mail.</p>
                        <p>Atenciosamente,<br>Equipe do Gerenciador de Contratos | GDC </p>
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
    return id
}