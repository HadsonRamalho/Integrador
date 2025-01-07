use axum::{http::StatusCode, Json};
use diesel::{ExpressionMethods, RunQueryDsl};
use rand::random;

use crate::models::{self, maquinas::Maquina, str_to_f64_bigdecimal};

use super::{cria_conn, gera_hash};

pub struct MaquinaInput{
    pub nome: String,
    pub numeroserie: String,
    pub valoraluguel: f64,
    pub disponivelaluguel: String,
    pub status: String
}

pub async fn cadastra_maquina(input: Json<MaquinaInput>)
    -> Result<(StatusCode, Json<models::maquinas::IdsMaquina>), (StatusCode, Json<String>)>{
    if input.nome.trim().is_empty() || input.numeroserie.trim().is_empty()
        || input.valoraluguel.to_string().trim().is_empty()
        || input.disponivelaluguel.trim().is_empty() || input.status.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, 
            Json("Um ou mais campos estão vazios.".to_string())))
    }
    if input.valoraluguel <= 0.0 {
        return Err((StatusCode::BAD_REQUEST,
            Json("O valor do aluguel não pode ser menor ou igual a zero.".to_string())))
    }
    let id: i32 = random();
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
        
    };
    let conn = &mut cria_conn()?;
    match models::maquinas::cadastra_maquina(conn, maquina).await{
        Ok(ids) => {
            return Ok((StatusCode::OK, Json(ids)))
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
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