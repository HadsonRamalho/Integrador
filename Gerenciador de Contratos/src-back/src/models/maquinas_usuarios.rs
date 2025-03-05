use diesel::{prelude::{Insertable, Queryable}, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::maquinas_usuarios;


#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Debug)]
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

pub async fn busca_maquinas_usuario_idusuario(conn: &mut PgConnection, id: String)
    -> Result<Vec<MaquinaUsuario>, String>{
    use crate::schema::maquinas_usuarios::dsl::*;

    let res: Result<Vec<MaquinaUsuario>, diesel::result::Error> = maquinas_usuarios.
        filter(idusuario.eq(id))
        .get_results(conn);

    match res{
        Ok(maqs) => {
            if !maqs.is_empty(){
                return Ok(maqs)
            }
            return Err("Este usuário ainda não cadastrou uma máquina.".to_string())
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}