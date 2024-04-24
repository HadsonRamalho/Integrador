// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::fs::File;
use std::io::{self, Read, Write};
use bincode::serialize;
use serde::{Deserialize, Serialize};
use bincode::deserialize;

#[derive(Debug, Serialize, Deserialize, Default)]
struct Usuario{
    nome:String, email:String, senha:String, uid:u32
}

impl Usuario{
    fn novo_usuario(nome: String, email: String, senha: String, uid: u32) -> Self{
        Usuario {nome, email, senha, uid}
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Usuarios{
    usuarios: Vec<Usuario>, qtd: u32
}

impl Usuarios{
    fn adiciona_usuario(&mut self, usuario: Usuario){
        self.usuarios.push(usuario)
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

fn exportar_arquivo(usuarios: &Usuarios) -> io::Result<()> {
    let file_path = "usuarios.bin";
    let encoded: Vec<u8> = serialize(usuarios).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("Failed to serialize: {}", e))
    })?;
    let mut file = File::create(file_path)?;
    file.write_all(&encoded)?;
    Ok(())
}

fn importar_arquivo() -> io::Result<Usuarios> {
    let file_path = "usuarios.bin";
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let usuarios: Usuarios = deserialize(&buffer).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("Failed to deserialize: {}", e))
    })?;
    Ok(usuarios)
}

fn inicializa_usuarios() -> String{
    let mut vet_users:Usuarios = Default::default();

    vet_users.adiciona_usuario(Usuario{email: "user1@u.com".to_string(), senha: "s1".to_string(), nome: "nome1".to_string(), uid: 00});
    vet_users.adiciona_usuario(Usuario{email: "user2@u.com".to_string(), senha: "s2".to_string(), nome: "nome2".to_string(), uid: 00});
    vet_users.adiciona_usuario(Usuario{email: "user3@u.com".to_string(), senha: "s3".to_string(), nome: "nome3".to_string(), uid: 00});
    vet_users.qtd = 3;
    let x = exportar_arquivo(&vet_users);
    if x.is_ok(){
        return format!("Ok")
    }
    return format!("err?")
}

#[tauri::command]
fn cria_conta(nome_completo: &str, email: &str, senha1: &str, senha2: &str) -> String {
    if senha1 != senha2 {
        return format!("Senhas diferentes. Corrija!");
    } 

    inicializa_usuarios();

    let resultado_importacao = importar_arquivo();
    if let Ok(mut usuarios) = resultado_importacao {
        let usuario = Usuario::novo_usuario(nome_completo.to_string(), email.to_string(), senha1.to_string(), 00);
        if usuarios.email_repetido(email){
            return format!("Erro: Esse e-mail já está sendo utilizado.")
        }
        usuarios.adiciona_usuario(usuario);
        
        if let Err(e) = exportar_arquivo(&usuarios) {
            return format!("Erro ao exportar arquivo: {}", e);
        }

        return format!("Conta criada com sucesso!");
    }
    return format!("Erro ao importar arquivo de usuários!");
}

#[tauri::command]
fn login_email(email: &str) -> String {
    let mut encontrado = false;
    let vazio = "";
    if email == vazio{
        return format!("Campo de e-mail não deve ficar em branco {}", vazio)
    }

    let resultado_importacao = importar_arquivo();
    if let Ok(usuarios) = resultado_importacao{
        encontrado = usuarios.email_repetido(email);
    }

    if encontrado{
        format!("E-mail {} encontrado!", vazio)
    } else {
        format!("E-mail {} não existe na base de dados! Verifique se escreveu corretamente ou tente criar uma nova conta.", vazio)
    }
}

#[tauri::command]
fn login_senha(email: &str, senha: &str) -> (String, bool){
    let vazio = "";
    if senha == vazio{
        return (format!("Campo de senha não deve ficar em branco {}", vazio), false)
    }
    
    let resultado_importacao = importar_arquivo();
    if let Ok(usuarios) = resultado_importacao{
        if usuarios.autentica(email, senha) {
            return (format!("Entrando! {}", vazio), true)
        }
    }    
    return (format!("Senha incorreta! {}", vazio), false)
    
}

fn main() {
    tauri::Builder::default()
       .invoke_handler(tauri::generate_handler![cria_conta, login_senha, login_email])
       .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
