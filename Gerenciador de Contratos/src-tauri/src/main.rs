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
mod db;
//

#[derive(Debug, Default)] //  Serialize, Deserialize,
struct Usuario{ // Objeto de usuário para unificar dados
    nome:String, email:String, senha:String,
}

impl Usuario{
    fn novo_usuario(nome: String, email: String, senha: String) -> Self{
        Usuario {nome, email, senha}
    }
}

fn valida_email(email: String) -> bool{
    let mut verificador = false;
    if email.contains("@") && email.contains(".") {
        verificador = true;
    }
    verificador
}

// Por ora, retorna a mensagem que vai ser exibida na interface, e um bool no sucesso da criação da conta
// também possui chamadas a funções obsoletas
#[tauri::command] 
async fn cria_conta(nomeCompleto: &str, email: &str, senha1: &str, senha2: &str) -> Result<bool, bool> { 
    let email:String = email.chars().filter(|c| !c.is_whitespace()).collect(); // Removendo todos os espaços em branco do email
    let usuario = Usuario::novo_usuario(nomeCompleto.to_string(), email.to_string(), senha1.to_string()); // Cria um novo usuário
    let validacao_email = valida_email(email);
    if validacao_email == false{
        return Ok(false); // Conta não criada
    }
    if senha1 != senha2 {
        return Ok(false); // Conta não criada
    }
    
    let mut email_repetido:u32 = 0;
    let _consome_result = save_data(&usuario.nome, &usuario.email, &usuario.senha, &mut email_repetido).await;
    if email_repetido == 0{   
        return Ok(true); // Conta criada   
    }
    return Ok(false); // Conta não foi criada
}

#[tauri::command]
fn login_email(email: &str) -> Result<bool, bool> { // Retorna uma mensagem de sucesso ou falha para o front
    let vazio = ""; // String vazia a ser comparada caso a verificação no front falhe
    if email == vazio{
        return Err(false)
    }
    return Ok(true)
}

#[tauri::command]
async fn login_senha(email: &str, senha: &str) -> Result<bool, bool>{ // Retorna uma mensagem para o front e um booleano
    let vazio = ""; 
    if senha == vazio{ // Verificação caso o campo do front falhe
        return Ok(false)
    }
    let mut senha_correta:u32 = 0;
    let _x = _verifica_senha(email, senha, &mut senha_correta).await;
    if senha_correta != 0 {
        return Ok(true)
    } else{
        return Ok(false)
    }
    
}

async fn save_data(nome: &str, email: &str, senha: &str, email_repetido: &mut u32) -> Result<u32, String> { // Parâmetros devem ser alterados conforme a necessidade posterior
    let pool = db::create_pool().await.map_err(|e| format!("{}", e))?;
    db::save_data(&pool, nome, &email, senha, email_repetido).await.map_err(|e| format!("{}", e))?; // Usa o arquivo db.rs para salvar dados no banco
    Ok(*(email_repetido))
}

async fn _verifica_senha(email: &str, senha: &str, senha_correta: &mut u32) -> Result<u32, String> { // Parâmetros devem ser alterados conforme a necessidade posterior
    let pool = db::create_pool().await.map_err(|e| format!("{}", e))?;
    db::verifica_senha(&pool, &email, senha,senha_correta).await.map_err(|e| format!("{}", e))?; // Usa o arquivo db.rs para salvar dados no banco
    Ok(*(senha_correta))
}


fn main() {
    tauri::Builder::default()
       .invoke_handler(tauri::generate_handler![cria_conta, login_senha, login_email]) // Registra funções do Tauri
       .run(tauri::generate_context!())
        .expect("erro ao tentar executar a aplicação Tauri");
}
