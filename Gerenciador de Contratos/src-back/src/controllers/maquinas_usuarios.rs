use axum::{extract::Query, Json};
use hyper::StatusCode;

use crate::models::{self, maquinas_usuarios::MaquinaUsuario};

use super::{cria_conn, gera_hash, maquinas::busca_maquina_id, usuarios::{busca_usuario_id, IdInput, UsuarioReturn}};

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

pub async fn busca_maquinas_usuario_idusuario(Query(input): Query<IdInput>)
    -> Result<(StatusCode, Json<Vec<models::maquinas::Maquina>>), (StatusCode, Json<String>)>{
    if input.id.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }
    let conn = &mut cria_conn()?;
    let id = input.id.trim().to_string();
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
        maquinas.push(busca_maquina_id(Query(IdInput{id: maq.idmaquina})).await?.1.0);
    }
    return Ok((StatusCode::OK, Json(maquinas)))
}

pub async fn busca_usuario_idmaquina(Query(id): Query<IdInput>)
    -> Result<(StatusCode, Json<UsuarioReturn>), (StatusCode, Json<String>)>{
    if id.id.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }
    let id = id.id.trim().to_string();

    let conn = &mut cria_conn()?;

    let idusuario = match models::maquinas_usuarios::busca_idusuario_idmaquina(conn, id).await{
        Ok(id) => {
            id
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    };

    let usuario = busca_usuario_id(Query(IdInput{id: idusuario})).await?;
    return Ok(usuario)
}