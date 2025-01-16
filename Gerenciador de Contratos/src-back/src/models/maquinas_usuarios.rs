use chrono::NaiveDateTime;
use diesel::{prelude::{Insertable, Queryable}, BoolExpressionMethods, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::maquinas_usuarios;


#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::maquinas_usuarios)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MaquinaUsuario{
    pub idmaquinausuario: String,
    pub idmaquina: String,
    pub idusuario: String
}

pub async fn cadastra_maquina_usuario(conn: &mut PgConnection, dados: MaquinaUsuario)
    -> Result<String, String>{
    let res: Result<MaquinaUsuario, diesel::result::Error> = diesel::insert_into(maquinas_usuarios::table)
        .values(dados)
        .get_result(conn);
    match res{
        Ok(maq) =>{
            return Ok(maq.idmaquinausuario)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}