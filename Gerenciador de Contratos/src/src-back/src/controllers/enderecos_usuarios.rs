use axum::{extract::Query, Json};
use hyper::StatusCode;

use crate::models::{self, enderecos_usuarios::EnderecoUsuario};

use super::{cria_conn, gera_hash};

pub struct EnderecoUsuarioInput{
    pub idendereco: String,
    pub idusuario: String
}

impl From<EnderecoUsuarioInput> for EnderecoUsuario{
    fn from(endereco: EnderecoUsuarioInput) -> Self {
        Self{
            idenderecousuario: gera_hash(&endereco.idendereco),
            idendereco: endereco.idendereco,
            idusuario: endereco.idusuario            
        }
    }
}

pub async fn cadastra_endereco_usuario(input: Json<EnderecoUsuarioInput>)
    -> Result<(StatusCode, Json<String>), (StatusCode, Json<String>)>{
    if input.idendereco.trim().is_empty() || input.idusuario.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }

    let conn = &mut cria_conn()?;

    let enderecousuario = EnderecoUsuario{
        idenderecousuario: gera_hash(&input.idendereco),
        idendereco: input.idendereco.clone(),
        idusuario: input.idusuario.clone(),
    };

    match models::enderecos_usuarios::cadastra_endereco_usuario(conn, enderecousuario).await{
        Ok(id) => {
            println!("ID ENDERECO USUARIO: {}", id);
            return Ok((StatusCode::OK, Json(id)))
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)));
        }
    }
}

pub async fn busca_enderecousuario_idusuario(Query(params): Query<String>)
    -> Result<(StatusCode, Json<EnderecoUsuario>), (StatusCode, Json<String>)>{
    if params.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }
    let idusuario = params.to_string();

    let conn = &mut cria_conn()?;

    match models::enderecos_usuarios::busca_enderecousuario_idusuario(conn, idusuario).await{
        Ok(enderecosusuario) => {
            return Ok((StatusCode::OK, Json(enderecosusuario)))
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    }
}