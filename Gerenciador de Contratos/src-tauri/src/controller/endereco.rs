use super::gera_hash;
use crate::{controller, model};

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
    if logradouro.trim().is_empty() || cep.trim().is_empty()
        || numeroendereco.trim().is_empty() ||
        cidade.trim().is_empty() || uf.trim().is_empty(){
            return Err("Erro: Preencha todos os campos.".to_string())
    }
    let cep = match controller::formata_cep(&cep){
        Ok(cep) => {cep},
        Err(e) => {return Err(e)}
    };
    let id = gera_hash(&cep);
    // Estrutura os dados do endereço em formato JSON
    let endereco = serde_json::json!({
        "idendereco": id,
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
    // Converte os dados recebidos em um objeto `Endereco`
    let endereco = crate::model::endereco::Endereco {
        idendereco: endereco["idendereco"].as_str().unwrap_or("").to_string().split_off(15 as usize),
        logradouro: endereco["logradouro"].as_str().unwrap_or("").to_string(),
        cep: endereco["cep"].as_str().unwrap_or("").to_string(),
        complemento: endereco["complemento"].as_str().unwrap_or("").to_string(),
        numeroendereco: endereco["numeroendereco"].as_str().unwrap_or("").to_string(),
        cidade: endereco["cidade"].as_str().unwrap_or("").to_string(),
        uf: endereco["uf"].as_str().unwrap_or("").to_string(),
    };
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

#[tauri::command]
pub async fn busca_endereco_id(idendereco: &str) -> Result<serde_json::Value, String>{
    let endereco: model::endereco::Endereco = match model::endereco::busca_endereco_id(idendereco).await{
        Ok(endereco) => {endereco},
        Err(e) => {return Err(e.to_string())}
    };
    let endereco = serde_json::json!({
        "idendereco": endereco.idendereco,
        "logradouro": endereco.logradouro,
        "cep": endereco.cep,
        "complemento": endereco.complemento,
        "numeroendereco": endereco.numeroendereco,
        "cidade": endereco.cidade,
        "uf": endereco.uf
    });
    return Ok(endereco)
}