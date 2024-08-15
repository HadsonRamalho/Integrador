use serde::{Deserialize, Serialize};

use super::enc_senha;

#[derive(Serialize, Deserialize)]
pub struct Endereco{
    pub logradouro: String,
    pub cep: String,
    pub complemento: String,
    pub numeroendereco: String,
    pub cidade: String,
    pub uf: String,
    pub id: String
}

#[tauri::command]
pub fn estrutura_endereco(logradouro: String, cep: String, complemento: String, numeroendereco: String, cidade: String, uf: String) -> Result<serde_json::Value, bool>{
    let id = enc_senha(&cep);
    let endereco = serde_json::json!({
        "id": id,
        "logradouro": logradouro,
        "cep": cep,
        "complemento": complemento,
        "numeroendereco": numeroendereco,
        "cidade": cidade,
        "uf": uf,
    });
    return Ok(endereco)
}

#[tauri::command]
pub async fn _salva_endereco(endereco: serde_json::Value){
    println!("salva?");
    let x = crate::model::endereco::salva_endereco(endereco).await;
    match x{
        Ok(_) => {
            println!("Salvou");
        }
        Err(e) => {
            eprintln!("Erro ao salvar o endereço: {:?}", e);
        }
    }
}

fn atualiza_endereco(endereco: serde_json::Value){

}