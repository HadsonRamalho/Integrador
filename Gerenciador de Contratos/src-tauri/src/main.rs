// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn inicializa_usuarios(){
    let mut vet_usuarios:[Usuario;10] = Default::default();
    vet_usuarios[0].email = "user1@u.com".to_string();
    vet_usuarios[1].email = "user2@u.com".to_string();
    vet_usuarios[3].email = "user3@u.com".to_string();

    vet_usuarios[0].senha = "s1".to_string();
    vet_usuarios[1].senha = "s2".to_string();
    vet_usuarios[3].senha = "s3".to_string();
}

#[tauri::command]
fn loginEmail(email: &str) -> String {
    let mut vet_usuarios:[Usuario;10] = Default::default();
    let mut encontrado = false;
    vet_usuarios[0].email = "user1@u.com".to_string();
    vet_usuarios[1].email = "user2@u.com".to_string();
    vet_usuarios[3].email = "user3@u.com".to_string();
    let mut indice:u32 = 0;
    for i in vet_usuarios.iter() {
        indice +=1 ;
        if i.email.eq_ignore_ascii_case(email.trim()) {
            encontrado = true;            
            break;
        }
    }
    let vazio = "";
    if(encontrado){
        format!("E-mail {} encontrado!", vazio)
    } else {
        format!("E-mail {} não existe na base de dados! Verifique se escreveu corretamente ou tente criar uma nova conta.", vazio)
    }
}

fn verifica_senha(u: &Usuario, senha: &str) -> (String, bool){
    let vazio = "";
    let mut encontrado = false;
    if u.senha.eq_ignore_ascii_case(senha.trim()) {
       encontrado = true;            
    }
    if(encontrado){
        return (format!("Senha {} correta!", vazio), true)
    }        
    return (format!("Senha {} incorreta.", vazio), false)
}

#[tauri::command]
fn loginSenha(email: &str, senha: &str) -> (String, bool){
    let mut vet_usuarios:[Usuario;10] = Default::default();
    let mut encontrado = false;
    let mut email_encontrado = false;
    vet_usuarios[0].email = "user1@u.com".to_string();
    vet_usuarios[1].email = "user2@u.com".to_string();
    vet_usuarios[3].email = "user3@u.com".to_string();
    vet_usuarios[0].senha = "s1".to_string();
    vet_usuarios[1].senha = "s2".to_string();
    vet_usuarios[3].senha = "s3".to_string();
    let mut indice:usize = 0;
    let vazio = "";
    for i in vet_usuarios.iter() {
        indice +=1 ;
        if i.email.eq_ignore_ascii_case(email.trim()) {
            email_encontrado = true;
            let u = i;
            return verifica_senha(u, senha);            
        }
    }
    return (format!("{}", vazio), false)
    
}

#[derive(Default)]
struct Usuario{
    nome:String, email:String, senha:String, UID:u32
}

fn main() {
    tauri::Builder::default()
       .invoke_handler(tauri::generate_handler![ loginEmail, loginSenha])
       .run(tauri::generate_context!())
        .expect("error while running tauri application");
}