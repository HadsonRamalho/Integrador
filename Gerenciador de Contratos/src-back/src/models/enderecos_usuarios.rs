use chrono::NaiveDateTime;
use diesel::{prelude::{Insertable, Queryable}, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, Selectable, SelectableHelper};
use serde::{Deserialize, Serialize};

use crate::controllers::cria_conn;

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::enderecos_usuarios)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EnderecoUsuario{
    pub idenderecousuario: String,
    pub idendereco: String,
    pub idusuario: String
}

pub async fn cadastra_endereco_usuario(conn: &mut PgConnection, dados: EnderecoUsuario)
    -> Result<String, String>{
    use crate::schema::enderecos_usuarios::dsl::*;

    let res: Result<EnderecoUsuario, diesel::result::Error> = diesel::insert_into(enderecos_usuarios)
        .values(dados)
        .get_result(conn);
    
    match res{
        Ok(dados) => {
            return Ok(dados.idenderecousuario)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

pub async fn busca_enderecousuario_idusuario(conn: &mut PgConnection, id: String)
    -> Result<EnderecoUsuario, String>{
    use crate::schema::enderecos_usuarios::dsl::*;

    let res: Result<EnderecoUsuario, diesel::result::Error> = enderecos_usuarios
        .filter(idusuario.eq(id))
        .get_result(conn);

    match res{
        Ok(endereco ) => {
            return Ok(endereco)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

pub async fn deleta_endereco_usuario(id: String)
    -> Result<String, String>{
    // Só utilizar em testes
    use crate::schema::enderecos_usuarios::dsl::*;

    let conn = &mut cria_conn().unwrap();

    let res: Result<EnderecoUsuario, diesel::result::Error> = diesel::delete(enderecos_usuarios)
        .filter(idenderecousuario.eq(id))
        .get_result(conn);

    match res{
        Ok(enderecoapagado) => {
            return Ok(enderecoapagado.idenderecousuario)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}