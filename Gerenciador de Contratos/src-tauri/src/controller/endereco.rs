use super::gera_hash;
use crate::{controller, model::{self, erro::MeuErro}};

/// ## Transforma campos separados de um endereço em um serde_json::Value
/// Primeiro, verifica se algum dos campos está vazio, retornando erro caso estejam:
/// ```
/// if logradouro.trim().is_empty() || cep.trim().is_empty()
///     || numeroendereco.trim().is_empty() ||
///     cidade.trim().is_empty() || uf.trim().is_empty(){
///    return Err(MeuErro::CamposVazios.to_string())
/// }
/// ```
/// #### Em seguida, tenta formatar o CEP e usa o resultado para gerar um hash para o ID do endereço, retornando um erro caso a formação falhe.
/// #### Finalmente, atribui os valores aos campos equivalentes no serde_json::Value e retorna o objeto.
#[tauri::command]
pub fn estrutura_endereco(logradouro: String, cep: String, complemento: String, numeroendereco: String, cidade: String, uf: String) -> Result<serde_json::Value, String>{
    if logradouro.trim().is_empty() || cep.trim().is_empty()
        || numeroendereco.trim().is_empty() ||
        cidade.trim().is_empty() || uf.trim().is_empty(){
            return Err(MeuErro::CamposVazios.to_string())
    }
    // Gera um ID único para o endereço com base no CEP
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

/// ## Converte um serde::json::Value em um objeto Endereco
/// ### Se algum dos campos do Value estiver vazio, o campo equivalente no Endereco também ficará vazio.
/// ```
/// logradouro: endereco["logradouro"].as_str().unwrap_or("").to_string()
/// ```
/// Por fim, salva o endereço no banco de dados e retorna o ID que foi gerado:
/// ```
/// let resultado_insert = crate::model::endereco::salva_endereco(endereco).await;
/// ```
#[tauri::command]
pub async fn _salva_endereco(endereco: serde_json::Value) -> Result<String, String>{
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
/// ## Recebe o ID de um endereço e retornar um serde_json::Value contendo os dados do endereço
/// Se o ID recebido como parâmetro estiver vazio, retorna erro.
/// Caso contrário, chama a função que faz a busca no banco de dados e que retorna um objeto do tipo Endereco.
/// Em seguida, converte os valores do objeto retornado para um serde_json::Value, que contém campos semelhantes ao do objeto Endereco.
/// Por fim, a função retorna um serde_json::Value contendo os dados.
#[tauri::command]
pub async fn busca_endereco_id(idendereco: &str) -> Result<serde_json::Value, String>{
    if idendereco.trim().is_empty(){
        return Err(MeuErro::CamposVazios.to_string())
    }
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