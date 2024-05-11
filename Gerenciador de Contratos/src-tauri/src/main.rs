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

#[derive(Debug, Default)] //  Serialize, Deserialize,
struct Usuarios{
    usuarios: Vec<Usuario>, // Vetor de usuários
                         // Provavelmente será substituída por dados do SQL
}

impl Usuarios{
    fn adiciona_usuario(&mut self, usuario: Usuario){
        self.usuarios.push(usuario) // Salvando 
    }

    fn email_repetido(&self, email: &str) -> bool{
        for u in self.usuarios.as_slice(){
            if u.email.eq_ignore_ascii_case(email){
                return true
            }
        }
        return false
    }
    
    fn autentica(&self, email: &str, senha: &str) -> bool{
        if self.email_repetido(email){
            for u in self.usuarios.as_slice(){
                if u.senha.eq(senha){
                    return true
                }
            }
        }        
        return false
    }
}

// Por ora, retorna a mensagem que vai ser exibida na interface, e um bool no sucesso da criação da conta
// também possui chamadas a funções obsoletas
#[tauri::command] 
async fn cria_conta(nomeCompleto: &str, email: &str, senha1: &str, senha2: &str) -> Result<bool, bool> { 
    let email:String = email.chars().filter(|c| !c.is_whitespace()).collect(); // Removendo todos os espaços em branco do email
    if senha1 != senha2 {
        return Ok(false); // Conta não criada
    }     
    let usuario = Usuario::novo_usuario(nomeCompleto.to_string(), email.to_string(), senha1.to_string()); // Cria um novo usuário
    let mut email_repetido:u32 = 0;
    let x = save_data(&usuario.email, &mut email_repetido).await;
    if email_repetido == 0{   
        return Ok(true); // Conta criada   
    }
    return Ok(false); // Conta não foi criada
}

#[tauri::command]
fn login_email(email: &str) -> String { // Retorna uma mensagem de sucesso ou falha para o front
    let vazio = ""; // String vazia a ser comparada caso a verificação no front falhe
    if(email == vazio){
        return format!("Campo de e-mail não deve ficar em branco {}", vazio);
    }
    return format!("{}", vazio);
}

#[tauri::command]
fn login_senha(email: &str, senha: &str) -> (String, bool){ // Retorna uma mensagem para o front e um booleano
    let vazio = ""; 
    if senha == vazio{ // Comparação caso a verificação no front falhe
        return (format!("Campo de senha não deve ficar em branco {}", vazio), false)
    }
    
   let usuarios:Usuarios = Default::default();
        if usuarios.autentica(email, senha) {
            return (format!("Entrando! {}", vazio), true)
        }
    return (format!("Senha incorreta! {}", vazio), false)
    
}

//DB
async fn save_data(email: &str, email_repetido: &mut u32) -> Result<u32, String> { // Parâmetros devem ser alterados conforme a necessidade posterior
    let pool = db::create_pool().await.map_err(|e| format!("{}", e))?;
    db::save_data(&pool, &email, email_repetido).await.map_err(|e| format!("{}", e))?; // Usa o arquivo db.rs para salvar dados no banco
    Ok(*(email_repetido))
}

#[tauri::command]
async fn email_repetido(email: &str) -> Result<(), String> {
    let mut repetido = 0;
    let pool = db::create_pool().await.map_err(|e| format!("{}", e))?;
    db::email_repetido(&pool, &email, &mut repetido).await.map_err(|e| format!("{}", e))?;
    Ok(())
}
//

fn main() {
    tauri::Builder::default()
       .invoke_handler(tauri::generate_handler![cria_conta, login_senha, login_email]) // Registra funções do Tauri
       .run(tauri::generate_context!())
        .expect("erro ao tentar executar a aplicação Tauri");
}
