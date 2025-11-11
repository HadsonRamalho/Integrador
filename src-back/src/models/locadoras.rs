use diesel::{prelude::{Insertable, Queryable}, query_dsl::methods::FilterDsl, ExpressionMethods, PgConnection, RunQueryDsl, Selectable};
use serde::{Deserialize, Serialize};

use crate::controllers::cria_conn;

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::locadoras)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Locadora{
    pub idlocadora: String,
    pub idusuario: String,
    pub idendereco: String,
    pub idconta: String
}

pub async fn cadastra_locadora(conn: &mut PgConnection, locadora: Locadora)
    -> Result<String, String>{
    use crate::schema::locadoras::dsl::*;
    let res: Result<Locadora, diesel::result::Error> = diesel::insert_into(locadoras)
        .values(locadora)
        .get_result(conn);
    match res{
        Ok(locadora) => {
            return Ok(locadora.idlocadora)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

pub async fn busca_locadora_idusuario(conn: &mut PgConnection, id: String)
    -> Result<Locadora, String>{
    use crate::schema::locadoras::dsl::*;

    let res: Result<Locadora, diesel::result::Error> = locadoras.filter(idusuario.eq(id))
        .first(conn);
    
    match res{
        Ok(locadora) => {
            return Ok(locadora)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

pub async fn busca_locadora_idlocadora(conn: &mut PgConnection, id: String) 
    -> Result<Locadora, String>{
    use crate::schema::locadoras::dsl::*;

    let res: Result<Locadora, diesel::result::Error> = locadoras.filter(idlocadora.eq(id))
      .get_result(conn);

    match res{
      Ok(locadora) => {
        return Ok(locadora)
      },
      Err(e) => {
        return Err(e.to_string())
      }
    }
}


pub async fn deleta_locadora(id: String)
    -> Result<(), String>{
    use crate::schema::locadoras::dsl::*;

    let conn = &mut cria_conn().unwrap();

    let res = diesel::delete(locadoras)
        .filter(idlocadora.eq(id))
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