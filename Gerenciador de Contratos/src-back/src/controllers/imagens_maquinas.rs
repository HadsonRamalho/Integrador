use axum::Json;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::models::{self, imagens_maquinas::{self}};

use super::cria_conn;

#[derive(Serialize, Deserialize)]
pub struct ImagemMaquinaInput{
    pub idimagem: String,
    pub idmaquina: String
}

pub async fn cadastra_imagem_maquina(input: Json<ImagemMaquinaInput>)
    -> Result<(StatusCode, Json<String>), (StatusCode, Json<String>)>{
    let idimagem = input.idimagem.to_string();
    let idmaquina = input.idmaquina.to_string();

    let conn = &mut cria_conn()?;

    match models::imagens_maquinas::cadastra_imagem_maquina(conn, idmaquina, idimagem).await{
        Ok(_) => {
            return Ok((StatusCode::OK, Json("Imagem cadastrada!".to_string())))
        },
        Err(e) => {
            println!("Erro [controller]: {}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    }
}

pub async fn recupera_imagem_maquina(input: Json<String>)
    -> Result<Json<String>, (StatusCode, Json<String>)> {

    let idmaquina = input.0.to_string();

    let conn = &mut cria_conn()?;

    match imagens_maquinas::recupera_imagem_maquina(conn, idmaquina).await {
        Ok(img) => {
            Ok(Json(img.1))
        },
        Err(e) => {
            println!("Erro [controller]: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    }
}

pub async fn busca_imagens_maquina(input: Json<String>)
    -> Result<(StatusCode, Json<Vec<String>>), (StatusCode, Json<String>)>{
    let idmaquina = input.0.to_string();

    let conn = &mut cria_conn()?;

    match models::imagens_maquinas::busca_imagens_maquina(conn, Json(idmaquina)).await{
        Ok(ids) => {
            return Ok((StatusCode::OK, Json(ids)))
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    }
}