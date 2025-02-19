use axum::{extract::Query, http::StatusCode, Json};
use diesel::{ExpressionMethods, RunQueryDsl};
use rand::random;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{controllers::{maquinas_usuarios::{cadastra_maquina_usuario, MaquinaUsuarioInput}, usuarios::{busca_usuario_id, IdInput}}, models::{self, maquinas::Maquina, str_to_f64_bigdecimal}};

use super::{cria_conn, gera_hash};

#[derive(Serialize, Deserialize)]
pub struct MaquinaInput{
    pub idusuario: String,

    pub nome: String,
    pub numeroserie: String,
    pub valoraluguel: f64,
    pub disponivelaluguel: String,
    pub status: String,
    pub categoria: String,
    pub descricao: String
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MaquinaReturn{
    pub idmaquina: String,
    pub idpublico: String,
    pub nome: String,
    pub numeroserie: String,
    pub categoria: String,
    pub valoraluguel: f64,
    pub disponivelaluguel: String,
    pub status: String,
    pub datacadastro: String,
    pub dataatualizacao: String,
    pub descricao: String
}

impl From<Maquina> for MaquinaReturn {
    fn from(maquina: Maquina) -> Self {
        Self {       
            nome: maquina.nome,
            idmaquina: maquina.idmaquina,
            idpublico: maquina.idpublico,
            numeroserie: maquina.numeroserie,
            categoria: maquina.categoria,
            valoraluguel: maquina.valoraluguel,
            disponivelaluguel: maquina.disponivelaluguel,
            status: maquina.status,
            datacadastro: maquina.datacadastro.to_string(),
            dataatualizacao: maquina.dataatualizacao.to_string(),
            descricao: maquina.descricao,
        }
    }
}

pub async fn cadastra_maquina(input: Json<MaquinaInput>)
    -> Result<(StatusCode, Json<models::maquinas::IdsMaquina>), (StatusCode, Json<String>)>{
    if input.nome.trim().is_empty() || input.numeroserie.trim().is_empty()
        || input.valoraluguel.to_string().trim().is_empty()
        || input.disponivelaluguel.trim().is_empty() || input.status.trim().is_empty()
        || input.idusuario.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, 
            Json("Um ou mais campos estão vazios.".to_string())))
    }

    assert!(busca_usuario_id(Query(IdInput{id: input.idusuario.clone()})).await.is_ok());

    if input.valoraluguel <= 0.0 {
        return Err((StatusCode::BAD_REQUEST,
            Json("O valor do aluguel não pode ser menor ou igual a zero.".to_string())))
    }
    let id: u32 = random();
    let datacadastro = chrono::Utc::now().naive_utc();
    let dataatualizacao = chrono::Utc::now().naive_utc();
    let maquina = Maquina{
        idmaquina: gera_hash(&input.numeroserie),
        idpublico: id.to_string(),
        nome: input.nome.to_string(),
        numeroserie: input.numeroserie.to_string(),
        valoraluguel: str_to_f64_bigdecimal(&input.valoraluguel.to_string()),
        disponivelaluguel: input.disponivelaluguel.to_string(),
        status: input.status.to_string(),
        datacadastro,
        dataatualizacao,
        descricao: input.descricao.to_string(),
        categoria: input.categoria.to_string()        
    };
    let conn = &mut cria_conn()?;
    let idsmaquina = match models::maquinas::cadastra_maquina(conn, maquina).await{
        Ok(ids) => {
            ids
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    };

    match cadastra_maquina_usuario(Json(MaquinaUsuarioInput{
        idmaquina: idsmaquina.idmaquina.clone(),
        idusuario: input.idusuario.trim().to_string()
    })).await{
        Ok(_res) => {
            return Ok((StatusCode::OK, Json(idsmaquina)))
        },
        Err(e) => {
            return Err(e)
        }
    }
}

pub async fn deleta_maquina_id(id: String)
    -> Result<String, String>{
    // Só utilizar em testes
    use crate::schema::maquinas::dsl::*;

    let conn = &mut cria_conn().unwrap();

    let res: Result<Maquina, diesel::result::Error> = diesel::delete(maquinas)
        .filter(idmaquina.eq(id))
        .get_result(conn);

    match res{
        Ok(maquinaapagada) => {
            return Ok(maquinaapagada.idmaquina)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

pub async fn lista_todas_maquinas()
    -> Result<(StatusCode, Json<Vec<Maquina>>), (StatusCode, Json<String>)>{
    let conn = &mut cria_conn()?;
    match models::maquinas::lista_todas_maquinas(conn).await{
        Ok(maquinas) => {
            return Ok((StatusCode::OK, Json(maquinas)))
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    };
}

pub async fn busca_maquina_id(input: Json<String>)
    -> Result<(StatusCode, Json<Maquina>), (StatusCode, Json<String>)>{
    if input.0.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }
    let conn = &mut cria_conn()?;
    let id = input.0.trim().to_string();
    match models::maquinas::busca_maquina_id(conn, id).await{
        Ok(maq) => {
            return Ok((StatusCode::OK, Json(maq)))
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    }
}

#[utoipa::path(
    get,
    tag = "Máquina",
    path = "/busca_maquina_idpublico/{id}",
    description = "Busca os dados de uma máquina a partir de seu ID público.",
    responses(
        (
            status = 200, 
            description = "ID válido. Máquina encontrada.",
            body = MaquinaReturn
        ),
        (
            status = 404,
            description = "O ID inserido não pertence a uma máquina registrada no sistema."
        ),
        (
            status = 400,
            description = "Parâmetro inválido ou ausente."
        ),
    ),
    params(
        ("id" = String, Path, description = "ID da máquina"),
    )
)]
pub async fn busca_maquina_idpublico(Query(params): Query<IdInput>)
    -> Result<(StatusCode, Json<MaquinaReturn>), (StatusCode, Json<String>)>{
    if params.id.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }
    let conn = &mut cria_conn()?;
    let idpublico = params.id.trim().to_string();
    match models::maquinas::busca_maquina_idpublico(conn, idpublico).await{
        Ok(maq) => {
            return Ok((StatusCode::OK, Json(MaquinaReturn::from(maq))))
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    }
}

pub async fn pesquisa_maquina(pesquisa: Json<String>)
    -> Result<(StatusCode, Json<Vec<Maquina>>), (StatusCode, Json<String>)>{
    if pesquisa.0.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())));
    }

    let pesquisa = pesquisa.0.to_string();

    let conn = &mut cria_conn()?;

    match models::maquinas::pesquisa_maquina(conn, pesquisa).await{
        Ok(maquinas) => {
            return Ok((StatusCode::OK, Json(maquinas)))
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    }
}