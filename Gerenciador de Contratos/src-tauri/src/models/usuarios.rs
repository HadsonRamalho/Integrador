use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{controllers::cria_conn, schema::usuarios};

#[derive(Queryable, Selectable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::usuarios)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Usuario {
    pub nome: String,
    pub email: String,
    pub senha: String,
    pub documento: String,
    pub idusuario: String
}

pub async fn cadastra_usuario(usuario: Usuario) -> Result<(), String>{
    let conn = &mut cria_conn()?;

    let res: Result<Usuario, diesel::result::Error> = diesel::insert_into(usuarios::table)
        .values(usuario)
        .get_result(conn);
    match res{
        Ok(_res) => {
            return Ok(())
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}