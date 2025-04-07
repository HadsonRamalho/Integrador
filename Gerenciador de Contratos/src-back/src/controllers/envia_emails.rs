use axum::{http::StatusCode, Json};
use dotenv::dotenv;
use std::env;
use lettre::{message::header::ContentType, transport::smtp::authentication::{Credentials, Mechanism}, Message, SmtpTransport, Transport};

pub fn verifica_credenciais_email() -> Result<(String, String), String>{
    dotenv().ok();
    let smtp_username = match
        env::var("smtp_username") {
        Ok(username) => {username},
        Err(e) => {
            return Err(format!("smtp_username não definido no arquivo .env: {}", e))
        }
    };
    let smtp_password = match env::var("smtp_password"){
        Ok(password) => {password},
        Err(e) => {
            return Err(format!("smtp_password não definido no arquivo .env: {}", e))
        }
    };
    let credenciais = (smtp_username, smtp_password);
    Ok(credenciais)
}

pub async fn envia_email_codigo(email: String, assunto: &str, codigo: String) 
    -> Result<(StatusCode, Json<String>), (StatusCode, Json<String>)>{
    // carregando as credenciais SMTP
    let credenciais = match verifica_credenciais_email(){
        Ok(credenciais) => {
            credenciais
        },
        Err(e) => {
            return Err((StatusCode::SERVICE_UNAVAILABLE, Json(e)))
        }
    };

    let smtp_username = credenciais.0;
    let smtp_password = credenciais.1;

    let assunto_titulo;
    let assunto_corpo;
    match assunto{
        "recuperação de senha" => {
            assunto_titulo = "Recuperação de Senha";
            assunto_corpo = "Você solicitou a redefinição da sua senha. Use o código abaixo para redefinir sua senha:"
        },
        "ativação de conta" => {
            assunto_titulo = "Ativação de Conta";
            assunto_corpo = "Bem-Vindo(a) ao MaqExpress! Use o código abaixo para ativar sua conta:"
        },
        _ => {
            return Err((StatusCode::BAD_REQUEST, Json("Assunto inválido.".to_string())))
        }
    }

    // o servidor SMTP e porta
    let smtp_server = "smtp.gmail.com";
    //let smtp_port = 587;
    // credenciais de autenticação SMTP
    let smtp_credentials = Credentials::new(
        smtp_username.to_string(), smtp_password.to_string()
    );

    // transporte SMTP
    let smtp_transport = SmtpTransport::starttls_relay(&smtp_server)
        .unwrap()
        .credentials(smtp_credentials)
        .authentication(vec![Mechanism::Plain])
        .build();

    let email = Message::builder()
        .from("gerenciadordecontratosgdc@gmail.com".parse().unwrap())
        .to(email.parse().unwrap())
        .subject(format!("MaqExpress | {}", assunto_titulo))
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
                            color:rgb(2, 119, 74);
                            padding: 10px;
                            border: 1px solidrgb(3, 102, 64);
                            border-radius: 5px;
                            display: inline-block;
                        }}
                    </style>
                </head>
                <body>
                    <div class="container">
                        <div class="header">{assunto_titulo}</div>
                        <p>Olá,</p>
                        <p>{assunto_corpo}</p>
                        <div class="code">{codigo}</div>
                        <p>Se você não solicitou isso, por favor ignore este e-mail.</p>
                        <p>Atenciosamente,<br>Equipe do MaqExpress </p>
                    </div>
                </body>
                </html>
                "#,
                codigo = codigo, assunto_corpo = assunto_corpo
            )
        )
        .unwrap();

    // enviar o e-mail usando o transporte SMTP
    match smtp_transport.send(&email) {
        Ok(_) => println!("E-mail enviado com sucesso!"),
        Err(err) => eprintln!("Erro ao enviar e-mail: {:?}", err),
    }
    return Ok((StatusCode::CREATED, Json(codigo)))
}