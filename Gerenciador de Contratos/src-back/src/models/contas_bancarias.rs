use diesel::{prelude::{Insertable, Queryable}, query_dsl::methods::FilterDsl, ExpressionMethods, PgConnection, RunQueryDsl, Selectable};
use serde::{Deserialize, Serialize};

use crate::controllers::cria_conn;

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::contas_bancarias)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ContaBancaria{
    pub idconta: String,
    pub idusuario: String,
    pub numeroconta: String,
    pub numeroagencia: String,
    pub nomebanco: String
}

pub async fn cadastra_conta_bancaria(conn: &mut PgConnection, conta: ContaBancaria)
    -> Result<String, String>{
    use crate::schema::contas_bancarias::dsl::*;

    let res: Result<ContaBancaria, diesel::result::Error> = diesel::insert_into(contas_bancarias)
      .values(conta)
      .get_result(conn);

    match res{
      Ok(conta) => {
        return Ok(conta.idconta)
      },
      Err(e) => {
        return Err(e.to_string());
      }
    }
}

pub async fn busca_conta_bancaria_idusuario(conn: &mut PgConnection, id: String)
    -> Result<ContaBancaria, String>{
    use crate::schema::contas_bancarias::dsl::*;

    let res: Result<ContaBancaria, diesel::result::Error> = contas_bancarias.filter(idusuario.eq(id))
      .get_result(conn);

    match res{
      Ok(conta) => {
        return Ok(conta)
      },
      Err(e) => {
        return Err(e.to_string())
      }
    }
}

pub async fn deleta_conta_bancaria(id: String)
    -> Result<(), String>{
    use crate::schema::contas_bancarias::dsl::*;

    let conn = &mut cria_conn().unwrap();

    let res = diesel::delete(contas_bancarias)
      .filter(idconta.eq(id))
      .execute(conn);
    
    match res{
      Ok(_) => {
        return Ok(())
      },
      Err(e) => {
        return Err(e.to_string())
      }
    }
}

pub async fn atualiza_conta_bancaria(conn: &mut PgConnection, conta: ContaBancaria)
    -> Result<ContaBancaria, String>{
    use crate::schema::contas_bancarias::dsl::*;

    let res: Result<ContaBancaria, diesel::result::Error> = diesel::update(contas_bancarias)
      .filter(idconta.eq(conta.idconta))
      .set((
        nomebanco.eq(conta.nomebanco),
        numeroagencia.eq(conta.numeroagencia),
        numeroconta.eq(conta.numeroconta)
      ))
      .get_result(conn);

    match res{
      Ok(conta) => {
        return Ok(conta)
      },
      Err(e) => {
        return Err(e.to_string())
      }
    }
}