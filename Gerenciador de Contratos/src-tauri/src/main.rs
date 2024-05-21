// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

// Útil para salvar arquivos localmente
/* 
use std::fs::File;
use std::io::{self, Read, Write};
use bincode::serialize;
use serde::{Deserialize, Serialize};
use bincode::deserialize;
*/

// Relacionados ao banco de dados
use std::env;
//

mod model;
mod controller;

use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use lettre::{Message, SmtpTransport, Transport};

fn envia_email(email: String){
  // as credenciais SMTP
  let smtp_username = "gerenciadordecontratosgdc@gmail.com";
  let smtp_password = "qeaa rzhm inlt bcyh";

  // o servidor SMTP e porta
  let smtp_server = "smtp.gmail.com";
  let smtp_port = 587;

  // credenciais de autenticação SMTP
  let smtp_credentials = Credentials::new(smtp_username.to_string(), smtp_password.to_string());

  // transporte SMTP
  let smtp_transport = SmtpTransport::starttls_relay(&smtp_server)
      .unwrap()
      .credentials(smtp_credentials)
      .authentication(vec![Mechanism::Plain])
      .build();

  // conteúdo do e-mail
  let email = Message::builder()
      .from("gerenciadordecontratosgdc@gmail.com".parse().unwrap())
      .to(email.parse().unwrap())
      .subject("Gerenciador de Contratos | Reset de senha")
      .body("Seu código de verificação é: {XXXX}".to_string())
      .unwrap();

  // enviar o e-mail usando o transporte SMTP
  match smtp_transport.send(&email) {
      Ok(_) => println!("E-mail enviado com sucesso!"),
      Err(err) => eprintln!("Erro ao enviar e-mail: {:?}", err),
  }
}

#[tauri::command]
async fn encontra_email(email: &str) -> Result<bool, bool>{
    let mut repetido = 0;
    let pool = model::create_pool().await.map_err(|e| format!("{}", e)).unwrap();
    let _consome_result = model::email_repetido(&pool, email, &mut repetido).await;
    println!("{repetido}");
    if repetido != 0 {
        envia_email(_consome_result.unwrap());
        Ok(true)}
    else {
        Ok(false)
    }
}

fn main() {
    tauri::Builder::default()
       .invoke_handler(tauri::generate_handler![
        controller::cria_conta,
        controller::login_senha, 
        controller::login_email,
        encontra_email]) // Registra funções do Tauri
       .run(tauri::generate_context!())
        .expect("erro ao tentar executar a aplicação Tauri");
}
