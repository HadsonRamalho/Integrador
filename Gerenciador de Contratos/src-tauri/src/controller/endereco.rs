use serde::{Deserialize, Serialize};

use super::gera_hash;

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

/// Função para estruturar os dados de um endereço em formato JSON.
///
/// Esta função cria um objeto JSON que contém os dados do endereço fornecidos como parâmetros,
/// gerando um ID único para o endereço com base no CEP. 
/// Esse JSON pode ser usado para armazenar ou manipular os dados do endereço posteriormente.
///
/// # Parâmetros
/// - `logradouro`: O nome da rua ou avenida do endereço.
/// - `cep`: O código postal (CEP) do endereço.
/// - `complemento`: Complemento do endereço, como apartamento, bloco, etc.
/// - `numeroendereco`: O número do endereço.
/// - `cidade`: A cidade do endereço.
/// - `uf`: A unidade federativa (UF) do estado correspondente ao endereço.
///
/// # Retornos
/// - `Result<serde_json::Value, bool>`: Retorna um objeto JSON com os dados estruturados do endereço em caso de sucesso.
///   Em caso de falha, retorna `Err(false)`.
///
/// # Exemplo de uso
/// ```
/// let endereco = estrutura_endereco(
///     "Rua Exemplo".to_string(),
///     "12345-678".to_string(),
///     "Apto 101".to_string(),
///     "10".to_string(),
///     "Cidade Exemplo".to_string(),
///     "SP".to_string()
/// ).unwrap();
/// ```
#[tauri::command]
pub fn estrutura_endereco(logradouro: String, cep: String, complemento: String, numeroendereco: String, cidade: String, uf: String) -> Result<serde_json::Value, String>{
    // Gera um ID único para o endereço com base no CEP
    if logradouro.is_empty() || cep.is_empty() || complemento.is_empty()
        || numeroendereco.is_empty() ||
        cidade.is_empty() || uf.is_empty(){
            return Err("Preencha todos os campos".to_string())
    }
    let id = gera_hash(&cep);
    // Estrutura os dados do endereço em formato JSON
    let endereco = serde_json::json!({
        "id": id,
        "logradouro": logradouro,
        "cep": cep,
        "complemento": complemento,
        "numeroendereco": numeroendereco,
        "cidade": cidade,
        "uf": uf,
    });

    // Retorna o JSON do endereço
    return Ok(endereco)
}

/// Função para criar a estrutura de um endereço.
///
/// # Parâmetros
/// - logradouro: Logradouro do endereço.
/// - cep: Código postal do endereço.
/// - complemento: Complemento do endereço (opcional).
/// - numeroendereco: Número do endereço.
/// - cidade: Cidade do endereço.
/// - uf: Unidade federativa (estado) do endereço.
///
/// # Retornos
/// - Result<serde_json::Value, bool>: Retorna um objeto JSON com os dados do endereço e um ID criptografado,
///   ou um erro booleano indicando falha na criação da estrutura.
#[tauri::command]
pub async fn _salva_endereco(endereco: serde_json::Value) -> Result<String, String>{
    let resultado_insert = crate::model::endereco::salva_endereco(endereco).await;
    match resultado_insert{
        Ok(id) => {
            return Ok(id)
        }
        Err(e) => {
            return Err(e.to_string());
        }
    }
}