use axum::Json;
use hyper::StatusCode;

use crate::models::{self, maquinas_usuarios::MaquinaUsuario};

use super::{cria_conn, gera_hash};

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