use chrono::NaiveDateTime;
use diesel::{prelude::{Insertable, Queryable}, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, Selectable, SelectableHelper};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::controllers::cria_conn;

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = crate::schema::enderecos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Endereco{
    pub idendereco: String,
    pub pais: String,
    pub estado: String,
    pub cidade: String,
    pub cep: String,
    pub bairro: String,
    pub logradouro: String,
    pub numero: String,
    pub complemento: String
}

pub async fn cadastra_endereco(conn: &mut PgConnection, endereco: Endereco)
    -> Result<Endereco, String>{
    use crate::schema::enderecos::dsl::*;

    let res: Result<Endereco, diesel::result::Error> = diesel::insert_into(enderecos)
        .values(endereco)
        .get_result(conn);

    match res{
        Ok(endereco) => {
            return Ok(endereco)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

pub async fn busca_endereco_id(conn: &mut PgConnection, id: String)
    -> Result<Endereco, String>{
    use crate::schema::enderecos::dsl::*;

    let res: Result<Endereco, diesel::result::Error> = enderecos.filter(idendereco.eq(id))
        .get_result(conn);

    match res{
        Ok(endereco) => {
            return Ok(endereco)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

pub async fn deleta_endereco(id: String)
    -> Result<String, String>{
    // Só utilizar em testes
    use crate::schema::enderecos::dsl::*;

    let conn = &mut cria_conn().unwrap();

    let res: Result<Endereco, diesel::result::Error> = diesel::delete(enderecos)
        .filter(idendereco.eq(id))
        .get_result(conn);

    match res{
        Ok(enderecoapagado) => {
            return Ok(enderecoapagado.idendereco)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}