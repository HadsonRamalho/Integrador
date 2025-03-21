use diesel::{prelude::{Insertable, Queryable}, query_dsl::methods::FilterDsl, ExpressionMethods, PgConnection, RunQueryDsl, Selectable};
use serde::{Deserialize, Serialize};

use crate::controllers::cria_conn;

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::locatarios)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Locatario{
    pub idlocatario: String,
    pub idusuario: String,
    pub idendereco: String,
}

pub async fn cadastra_locatario(conn: &mut PgConnection, locatario: Locatario)
    -> Result<String, String>{
    use crate::schema::locatarios::dsl::*;
    let res: Result<Locatario, diesel::result::Error> = diesel::insert_into(locatarios)
        .values(locatario)
        .get_result(conn);
    match res{
        Ok(locatario) => {
            return Ok(locatario.idlocatario)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

pub async fn busca_locatario_idusuario(conn: &mut PgConnection, id: String)
    -> Result<Locatario, String>{
    use crate::schema::locatarios::dsl::*;

    let res: Result<Locatario, diesel::result::Error> = locatarios.filter(idusuario.eq(id))
        .first(conn);
    
    match res{
        Ok(locatario) => {
            return Ok(locatario)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

pub async fn busca_locatario_idlocatario(conn: &mut PgConnection, id: String)
     -> Result<Locatario, String>{
    use crate::schema::locatarios::dsl::*;

    let res: Result<Locatario, diesel::result::Error> = locatarios.filter(idlocatario.eq(id))
      .get_result(conn);

    match res{
      Ok(locatario) => {
        return Ok(locatario)
      },
      Err(e) => {
        return Err(e.to_string())
      }
    }
}

pub async fn deleta_locatario(id: String)
    -> Result<(), String>{
    use crate::schema::locatarios::dsl::*;

    let conn = &mut cria_conn().unwrap();

    let res = diesel::delete(locatarios)
        .filter(idlocatario.eq(id))
        .execute(conn);

    match res{
        Ok(_) => {
            Ok(())
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}