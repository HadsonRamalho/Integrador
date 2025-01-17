use axum::Json;
use hyper::StatusCode;

use crate::models::{self, maquinas_usuarios::MaquinaUsuario};

use super::{cria_conn, gera_hash, maquinas::busca_maquina_id};

pub struct MaquinaUsuarioInput{
    pub idmaquina: String,
    pub idusuario: String
}

pub async fn cadastra_maquina_usuario(input: Json<MaquinaUsuarioInput>)
    -> Result<(StatusCode, Json<String>), (StatusCode, Json<String>)>{
    if input.idmaquina.trim().is_empty() || input.idusuario.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }
    let maquinausuario = MaquinaUsuario{
        idmaquina: input.idmaquina.trim().to_string(),
        idusuario: input.idusuario.trim().to_string(),
        idmaquinausuario: gera_hash(&input.idusuario)
    };
    let conn = &mut cria_conn()?;
    match models::maquinas_usuarios::cadastra_maquina_usuario(conn, maquinausuario).await{
        Ok(id) => {
            return Ok((StatusCode::OK, Json(id)))
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    }
}

pub async fn busca_maquinas_usuario_idusuario(input: Json<String>)
    -> Result<(StatusCode, Json<Vec<models::maquinas::Maquina>>), (StatusCode, Json<String>)>{
    if input.0.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }
    let conn = &mut cria_conn()?;
    let id = input.0.trim().to_string();
    let maqs = match models::maquinas_usuarios::busca_maquinas_usuario_idusuario(conn, id).await{
        Ok(maqs) => {
            maqs
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    };
    let mut maquinas = vec![];
    for maq in maqs{
        maquinas.push(busca_maquina_id(Json(maq.idmaquina)).await?.1.0);
    }
    return Ok((StatusCode::OK, Json(maquinas)))
}
