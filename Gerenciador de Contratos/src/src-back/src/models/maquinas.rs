use chrono::NaiveDateTime;
use diesel::{prelude::{Insertable, Queryable}, BoolExpressionMethods, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IdsMaquina{
    pub idmaquina: String,
    pub idpublico: String
}


#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::maquinas)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Maquina{
    pub idmaquina: String,
    pub idpublico: String,
    pub nome: String,
    pub numeroserie: String,
    pub categoria: String,
    pub valoraluguel: f64,
    pub disponivelaluguel: String,
    pub status: String,
    pub datacadastro: NaiveDateTime,
    pub dataatualizacao: NaiveDateTime,
    pub descricao: String
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

pub async fn lista_todas_maquinas(conn: &mut PgConnection)
    -> Result<Vec<Maquina>, String>{
    use crate::schema::maquinas::dsl::*;

    let res: Result<Vec<Maquina>, diesel::result::Error> = maquinas
        .filter(status.eq("Ativo").and(disponivelaluguel.eq("Sim")))
        .get_results(conn);

    match res{
        Ok(maqs) => {
            if !maqs.is_empty(){
                return Ok(maqs)
            }
            return Err("Não encontramos máquinas cadastradas no sistema.".to_string())
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

pub async fn busca_maquina_id(conn: &mut PgConnection, id: String)
    -> Result<Maquina, String>{
    use crate::schema::maquinas::dsl::*;

    let res: Result<Maquina, diesel::result::Error> = maquinas.filter(idmaquina.eq(id))
        .get_result(conn);

    match res{
        Ok(maquina) => {
            return Ok(maquina)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}