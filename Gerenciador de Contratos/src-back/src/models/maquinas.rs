use chrono::NaiveDateTime;
use diesel::{prelude::{Insertable, Queryable}, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, Selectable, SelectableHelper};
use serde::{Deserialize, Serialize};

use crate::controllers::cria_conn;

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::maquinas)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Maquina{
    pub idmaquina: String,
    pub idpublico: String,
    pub nome: String,
    pub numeroserie: String,
    pub valoraluguel: f64,
    pub disponivelaluguel: String,
    pub status: String,
    pub datacadastro: NaiveDateTime,
    pub dataatualizacao: NaiveDateTime
}

pub struct IdsMaquina{
    pub idmaquina: String,
    pub idpublico: String
}

pub async fn cadastra_maquina(conn: &mut PgConnection, maquina: Maquina)
    -> Result<IdsMaquina, String>{
    use crate::schema::maquinas::dsl::*;

    let res: Result<Maquina, diesel::result::Error> = diesel::insert_into(maquinas)
        .values(maquina)
        .get_result(conn);

    match res{
        Ok(maquina) => {
            return Ok(IdsMaquina{
                idmaquina: maquina.idmaquina,
                idpublico: maquina.idpublico
            })
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}