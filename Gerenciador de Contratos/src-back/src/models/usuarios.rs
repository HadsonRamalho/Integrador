use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{controllers::cria_conn, schema::usuarios::{self, idusuario}};

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

// Só utilizar em testes
pub async fn deleta_usuario(id: String) -> Result<(), String>{
    let conn = &mut cria_conn()?;

    let res: Result<Usuario, diesel::result::Error> = diesel::delete(usuarios::table)
        .filter(idusuario.eq(id))
        .get_result(conn);
    match res{
        Ok(_) => {
            return Ok(())
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

pub async fn busca_email_usuario(id: String) -> Result<String, String>{
    use self::usuarios::dsl::*;
    let conn = &mut cria_conn()?;

    let res = usuarios.filter(idusuario.eq(id)).select(Usuario::as_select()).first(conn);
    match res{
        Ok(usuario) => {
            return Ok(usuario.email)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

pub async fn atualiza_email_usuario(email_antigo: String, email_novo: String) -> Result<String, String>{
 use self::usuarios::dsl::*;
    let conn = &mut cria_conn()?;

    let res = usuarios.filter(email.eq(email_antigo)).select(Usuario::as_select()).first(conn);
    let id = match res{
        Ok(usuario) => {
            usuario.idusuario
        },
        Err(e) => {
            return Err(e.to_string())
        }
    };

    let usuario_atualizado = diesel::update(usuarios.find(id))
        .set(email.eq(email_novo))
        .returning(Usuario::as_returning())
        .get_result(conn);

    match usuario_atualizado{
        Ok(usuario_atualizado) => {
            return Ok(usuario_atualizado.idusuario)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }

}

pub async fn busca_usuario_email(email_: String) -> Result<String, String>{
    use self::usuarios::dsl::*;

    let conn = &mut cria_conn()?;

    let res: Result<Usuario, diesel::result::Error> = usuarios.filter(email.eq(email_)).first(conn);
    match res{
        Ok(usuario) => {
            return Ok(usuario.idusuario)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}