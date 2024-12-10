use dotenv::dotenv;
use std::env;
use lettre::{message::header::ContentType, transport::smtp::authentication::{Credentials, Mechanism}, Message, SmtpTransport, Transport};

use super::gera_hash;

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

pub fn envia_email_codigo(email: String, assunto: &str) -> Result<String, String>{
    // carregando as credenciais SMTP
    let credenciais = verifica_credenciais_email()?;
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
            assunto_corpo = "Bem-Vindo(a) ao Gerenciador de Contratos! Use o código abaixo para ativar sua conta:"
        },
        _ => {
            return Err("Assunto inválido.".to_string())
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

    let codigo = gera_hash(&email);
    let codigo = codigo.get(8..12).unwrap().to_string();
    // conteúdo do e-mail
    let code = format!("{}", codigo);
    let email = Message::builder()
        .from("gerenciadordecontratosgdc@gmail.com".parse().unwrap())
        .to(email.parse().unwrap())
        .subject(format!("Gerenciador de Contratos | {}", assunto_titulo))
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
                        <p>{assunto_corpo}</p>
                        <div class="code">{code}</div>
                        <p>Se você não solicitou isso, por favor ignore este e-mail.</p>
                        <p>Atenciosamente,<br>Equipe do Gerenciador de Contratos | GDC </p>
                    </div>
                </body>
                </html>
                "#,
                code = code, assunto_corpo = assunto_corpo
            )
        )
        .unwrap();

    // enviar o e-mail usando o transporte SMTP
    match smtp_transport.send(&email) {
        Ok(_) => println!("E-mail enviado com sucesso!"),
        Err(err) => eprintln!("Erro ao enviar e-mail: {:?}", err),
    }
    return Ok(codigo)
}