use chrono::NaiveDateTime;
use diesel::{prelude::{Insertable, Queryable}, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, Selectable, SelectableHelper};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::codigos_recuperacao)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CodigoRecuperacao{
    codigo: String,
    datacriacao: NaiveDateTime,
    dataexpiracao: NaiveDateTime,
    status: String,
    idusuario: String,
    idcodigo: String
}

pub async fn cadastra_codigo_recuperacao(conn: &mut PgConnection, dados: CodigoRecuperacao)
    -> Result<String, String>{
    use crate::schema::codigos_recuperacao::dsl::*;

    let res: Result<CodigoRecuperacao, diesel::result::Error> = diesel::insert_into(codigos_recuperacao)
        .values(dados)
        .get_result(conn);

    match res{
        Ok(codigorecuperacao) => {
            return Ok(codigorecuperacao.idcodigo)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}


pub async fn verifica_codigo_recuperacao(conn: &mut PgConnection, idusuario_: String, codigo_: String) -> Result<String, String>{
    use crate::schema::codigos_recuperacao::dsl::*;

    let res: Result<CodigoRecuperacao, diesel::result::Error> = codigos_recuperacao
        .filter(idusuario.eq(idusuario_.clone()))
        .filter(codigo.eq(codigo_))
        .filter(status.eq("Não utilizado"))
        .first(conn);
    let idcodigo_ = match res{
        Ok(codigorecuperacao) => {
            codigorecuperacao.idcodigo
        },
        Err(e) => {
            return Err(e.to_string())
        }
    };

    let res: Result<CodigoRecuperacao, diesel::result::Error> = diesel::update(codigos_recuperacao.find(idcodigo_))
        .set(status.eq("utilizado"))
        .returning(CodigoRecuperacao::as_returning())
        .get_result(conn);
    match res{
        Ok(res) => {
            return Ok(res.codigo)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}