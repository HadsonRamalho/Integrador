// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    let mut vet_usuarios:[Usuario;10] = Default::default();
    let mut encontrado = false;
    vet_usuarios[0].email = "user1@u.com".to_string();
    vet_usuarios[1].email = "user2@u.com".to_string();
    vet_usuarios[3].email = "user3@u.com".to_string();
    let mut indice:u32 = 0;
    for i in vet_usuarios.iter() {
        indice +=1 ;
        if i.email.eq_ignore_ascii_case(name.trim()) {
            encontrado = true;            
            break;
        }
    }
    if(encontrado){
        format!("E-mail {} encontrado!", name)
    } else {
        format!("E-mail {} não encontrado!", name)
    }
}

#[derive(Default)]
struct Usuario{
    nome:String, email:String, senha:String, UID:u32
}

fn main() {
    tauri::Builder::default()
       .invoke_handler(tauri::generate_handler![greet])
       .run(tauri::generate_context!())
        .expect("error while running tauri application");
}