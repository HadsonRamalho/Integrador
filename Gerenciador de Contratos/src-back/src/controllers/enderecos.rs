use axum::{extract::Query, Json};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use tracing::span::Id;
use utoipa::ToSchema;

use crate::{controllers::usuarios::{busca_usuario_id, IdInput}, models::{self, enderecos::Endereco}};

use super::{cria_conn, enderecos_usuarios::busca_enderecousuario_idusuario, gera_hash, usuarios::UserId};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EnderecoUsuarioInput{
    pub idusuario: String,
    pub pais: String,
    pub estado: String,
    pub cidade: String,
    pub cep: String,
    pub bairro: String,
    pub logradouro: String,
    pub numero: String,
    pub complemento: Option<String>
}

#[utoipa::path(
    post,
    tag = "Endereço",
    path = "/cadastra_endereco_usuario",
    description = "Cadastra um endereço no sistema.",
    responses(
        (
            status = 200, 
            description = "Dados válidos. O endereço foi cadastrado.",
            body = Endereco
        ),
        (
            status = 500,
            description = "Erro ao cadastrar o endereço."
        ),
        (
            status = 400,
            description = "Algum dos campos inseridos está incorreto."
        ),
    ),
    request_body = EnderecoUsuarioInput    
)]

pub async fn cadastra_endereco_usuario(input: Json<EnderecoUsuarioInput>)
    -> Result<(StatusCode, Json<Endereco>), (StatusCode, Json<String>)>{
    if input.bairro.trim().is_empty() || input.cep.trim().is_empty() || input.cidade.trim().is_empty()
        || input.estado.trim().is_empty() || input.logradouro.trim().is_empty() 
        || input.numero.trim().is_empty() || input.pais.trim().is_empty()
        || input.idusuario.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }
    let complemento = input.complemento.clone();
    let complemento = match complemento{
        Some(complemento) => {
            complemento
        },
        None => {
            "Sem complemento".to_string()
        }
    };

    let endereco = Endereco{
        idendereco: gera_hash(&input.logradouro),
        pais: input.pais.to_string(),
        estado: input.estado.to_string(),
        cidade: input.cidade.to_string(),
        cep: input.cep.to_string(),
        bairro: input.bairro.to_string(),
        logradouro: input.logradouro.to_string(),
        numero: input.numero.to_string(),
        complemento,
    };

    let idusuario = busca_usuario_id(Query(IdInput{id: input.idusuario.clone()})).await?.1.idusuario.clone();
    let conn = &mut cria_conn()?;

    let endereco = match models::enderecos::cadastra_endereco(conn, endereco).await{
        Ok(endereco) => {
            endereco
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    };

    match crate::controllers::enderecos_usuarios::cadastra_endereco_usuario(Json(crate::controllers::enderecos_usuarios::EnderecoUsuarioInput{
        idendereco: endereco.idendereco.clone(),
        idusuario
    })).await{
        Ok(_res) => {
            return Ok((StatusCode::OK, Json(endereco)))
        },
        Err(e) => {
            return Err(e)
        }
    }
}

#[utoipa::path(
    get,
    tag = "Endereço",
    path = "/busca_endereco_id/{idendereco}",
    description = "Busca um endereço no sistema.",
    responses(
        (
            status = 200, 
            description = "O endereço foi encontrado.",
            body = Endereco
        ),
        (
            status = 500,
            description = "ID inválido. Erro ao buscar o endereço."
        ),
        (
            status = 400,
            description = "Algum dos campos inseridos está incorreto."
        ),
    ),
    params(
        ("idendereco" = String, Path, description = "ID do endereço"),
    )  
)]
pub async fn busca_endereco_id(Query(params): Query<String>)
    -> Result<(StatusCode, Json<Endereco>), (StatusCode, Json<String>)>{
    if params.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }

    let id = params.to_string();

    let conn = &mut cria_conn()?;

    match models::enderecos::busca_endereco_id(conn, id).await{
        Ok(endereco) => {
            return Ok((StatusCode::OK, Json(endereco)))
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    }
}

#[utoipa::path(
    get,
    tag = "Endereço",
    path = "/busca_endereco_idusuario/{idusuario}",
    description = "Busca um endereço no sistema, usando o ID de um usuário.",
    responses(
        (
            status = 200, 
            description = "O endereço foi encontrado.",
            body = Endereco
        ),
        (
            status = 500,
            description = "ID inválido. Erro ao buscar o endereço."
        ),
        (
            status = 400,
            description = "Algum dos campos inseridos está incorreto."
        ),
    ),
    params(
        ("idusuario" = UserId, Path, description = "ID do usuário"),
    )  
)]
pub async fn busca_endereco_idusuario(Query(params): Query<UserId>)
    -> Result<(StatusCode, Json<Endereco>), (StatusCode, Json<String>)>{
    if params.idusuario.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }

    let idendereco = busca_enderecousuario_idusuario(axum::extract::Query(params)).await?.1.idendereco.to_string();

    let endereco = busca_endereco_id(Query(idendereco)).await;
    endereco
}

pub async fn atualiza_endereco(endereco: Json<Endereco>)
    -> Result<(StatusCode, Json<Endereco>), (StatusCode, Json<String>)>{
    let endereco = endereco.0;

    let conn = &mut cria_conn()?;

    match models::enderecos::atualiza_endereco(endereco).await{
        Ok(endereco) => {
            return Ok((StatusCode::OK, Json(endereco)))
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    }
}