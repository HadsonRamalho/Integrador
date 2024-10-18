use std::fmt::format;

use axum::{http::StatusCode, Json};
use serde::Deserialize;

use super::gera_hash;
use crate::{controller, model::{self, endereco::Endereco, erro::MeuErro}};

#[derive(Deserialize)]
pub struct EnderecoInput {
    pub logradouro: String,
    pub cep: String,
    pub complemento: String,
    pub numeroendereco: String,
    pub cidade: String,
    pub uf: String,
}

impl From<axum::Json<Endereco>> for EnderecoInput{
    fn from(value: axum::Json<Endereco>) -> Self {
        EnderecoInput{
            logradouro: value.logradouro.clone(),
            cep: value.cep.clone(),
            complemento: value.cep.clone(),
            numeroendereco: value.numeroendereco.clone(),
            cidade: value.cidade.clone(),
            uf: value.uf.clone()
        }
    }
}

fn endereco_vazio() -> Endereco{
    let endereco_vazio = Endereco{
        idendereco: "".to_string(),
        logradouro: "".to_string(),
        cep: "".to_string(),
        complemento: "".to_string(),
        numeroendereco: "".to_string(),
        cidade: "".to_string(),
        uf: "".to_string()
    };
    endereco_vazio
}

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
pub fn estrutura_endereco(Json(input): Json<EnderecoInput>) -> Result<(StatusCode, axum::Json<Endereco>), (StatusCode, String)> {
    let endereco_vazio = endereco_vazio();
    if input.logradouro.trim().is_empty()
        || input.cep.trim().is_empty()
        || input.numeroendereco.trim().is_empty()
        || input.cidade.trim().is_empty()
        || input.uf.trim().is_empty()
    {
        return Err((StatusCode::BAD_REQUEST, MeuErro::CamposVazios.to_string()))
    }
    // Gera um ID único para o endereço com base no CEP
    let cep_formatado = match controller::formata_cep(&input.cep) {
        Ok(cep) => cep,
        Err(e) => {
            return Err((StatusCode::BAD_REQUEST, e))
        }
    };

    let id = gera_hash(&cep_formatado).split_off(15 as usize);
    // Estrutura os dados do endereço em formato JSON
    let endereco = model::endereco::Endereco {
        idendereco: id,
        logradouro: input.logradouro,
        cep: cep_formatado,
        complemento: input.complemento,
        numeroendereco: input.numeroendereco,
        cidade: input.cidade,
        uf: input.uf,
    };

    // Retorna o JSON do endereço
    return Ok((StatusCode::OK, Json(endereco)))
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
pub async fn _salva_endereco(Json(input): Json<EnderecoInput>) -> Result<Json<String>, (StatusCode, String)>{
    let resposta = match estrutura_endereco(axum::Json(input)){
        Ok(resposta) => {resposta},
        Err(e) => {
            return Err(e)
        }
    };
    let mut endereco = resposta.1;
    let endereco = crate::model::endereco::Endereco {
        idendereco: endereco.idendereco.clone(),
        logradouro: endereco.logradouro.clone(),
        cep: endereco.cep.clone(),
        complemento: endereco.complemento.clone(),
        numeroendereco: endereco.numeroendereco.clone(),
        cidade: endereco.cidade.clone(),
        uf: endereco.uf.clone(),
    };
    let resultado_insert = crate::model::endereco::salva_endereco(endereco).await;
    match resultado_insert{
        Ok(id) => {
            return Ok(Json(id))
        }
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    }
}
/// ## Recebe o ID de um endereço e retornar um serde_json::Value contendo os dados do endereço
/// Se o ID recebido como parâmetro estiver vazio, retorna erro.
/// Caso contrário, chama a função que faz a busca no banco de dados e que retorna um objeto do tipo Endereco.
/// Em seguida, converte os valores do objeto retornado para um serde_json::Value, que contém campos semelhantes ao do objeto Endereco.
/// Por fim, a função retorna um serde_json::Value contendo os dados.
pub async fn busca_endereco_id(input: Json<String>) -> Result<(StatusCode, Json<Endereco>), (StatusCode, String)>{
    let idendereco = input.0;
    if idendereco.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, MeuErro::CamposVazios.to_string()))
    }
    let endereco: model::endereco::Endereco = match model::endereco::busca_endereco_id(&idendereco).await{
        Ok(endereco) => {endereco},
        Err(e) => {
            println!("{:?}", e);
            return  Err((StatusCode::BAD_REQUEST, format!("{}", e)))
        }
    };
    return Ok((StatusCode::OK, axum::Json(endereco)))
}