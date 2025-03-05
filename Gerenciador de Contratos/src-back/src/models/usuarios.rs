use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::{controllers::{cria_conn, usuarios::AtualizaUsuarioInput}, schema::usuarios::{self, idusuario}};

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::usuarios)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Usuario {
    pub nome: String,
    pub email: String,
    pub senha: String,
    pub documento: String,
    pub datacadastro: NaiveDateTime,
    pub idusuario: String,
    pub origemconta: Option<String>
}

pub async fn cadastra_usuario(conn: &mut PgConnection, usuario: Usuario) 
    -> Result<(), String>{
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
pub async fn deleta_usuario(id: String) 
    -> Result<(), String>{

    let conn = &mut cria_conn().unwrap();

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

pub async fn busca_email_usuario(conn: &mut PgConnection, id: String) 
    -> Result<String, String>{
    use self::usuarios::dsl::*;

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

pub async fn atualiza_email_usuario(conn: &mut PgConnection, email_antigo: String, email_novo: String)
     -> Result<String, String>{
    use self::usuarios::dsl::*;

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

pub async fn busca_usuario_email(conn: &mut PgConnection, email_: String)
    -> Result<String, String>{
    use self::usuarios::dsl::*;

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

pub async fn busca_usuario_email_oauth(conn: &mut PgConnection, email_: String)
    -> Result<String, String>{
    use self::usuarios::dsl::*;

    let res: Result<Usuario, diesel::result::Error> = usuarios.filter(email.eq(email_).and(origemconta.eq("Google"))).first(conn);
    match res{
        Ok(usuario) => {
            return Ok(usuario.idusuario)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

pub async fn busca_senha_usuario(conn: &mut PgConnection, email_: String) 
    -> Result<String, String>{
    use self::usuarios::dsl::*;

    let res: Result<Usuario, diesel::result::Error> = usuarios.filter(email.eq(email_)).first(conn);
    match res{
        Ok(usuario) => {
            return Ok(usuario.senha)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

pub async fn atualiza_senha_usuario(conn: &mut PgConnection, email_: String, senha_nova: String) 
    -> Result<String, String>{
    use self::usuarios::dsl::*;

    let res = usuarios.filter(email.eq(email_)).select(Usuario::as_select()).first(conn);
    let id = match res{
        Ok(usuario) => {
            usuario.idusuario
        },
        Err(e) => {
           return Err(e.to_string())
        }
    };
   
    let usuario_atualizado = diesel::update(usuarios.find(id))
        .set(senha.eq(senha_nova))
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
pub async fn busca_usuario_id(conn: &mut PgConnection, id: String)
    -> Result<Usuario, String>{
    use self::usuarios::dsl::*;

    let res: Result<Usuario, diesel::result::Error> = usuarios.filter(idusuario.eq(id)).first(conn);
    match res{
        Ok(usuario) => {
            return Ok(usuario)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

pub async fn atualiza_usuario(conn: &mut PgConnection, usuario: AtualizaUsuarioInput)
    -> Result<String, String>{
    use self::usuarios::dsl::*;

    let res: Result<Usuario, diesel::result::Error> = usuarios
        .filter(email.eq(usuario.email_antigo))
        .get_result(conn);

    let usuario_banco = match res{
        Ok(usuario) => {
            usuario
        },
        Err(e) => {
            return Err(e.to_string())
        }
    };

    let res: Result<Usuario, diesel::result::Error> = usuarios.filter(documento.eq(usuario.documento_novo.clone()))
        .get_result(conn);

    match res{
        Ok(user) => {
            if user.documento == usuario.documento_novo && usuario_banco.idusuario != user.idusuario{
                return Err("Esse documento já pertence a outra pessoa.".to_string())
            }
        },
        Err(_) => {}
    }

    let res: Result<Usuario, diesel::result::Error> = diesel::update(usuarios)
        .filter(idusuario.eq(usuario_banco.idusuario))
        .set((
            email.eq(usuario.email_novo),
            documento.eq(usuario.documento_novo),
            nome.eq(usuario.nome_novo)
        )).get_result(conn);
        
    match res{
        Ok(usuario ) =>{
            return Ok(usuario.idusuario)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

pub async fn busca_usuario_documento(conn: &mut PgConnection, doc: String)
    -> Result<Usuario, String>{
    use self::usuarios::dsl::*;

    let res: Result<Usuario, diesel::result::Error> = usuarios.filter(documento.eq(doc)).first(conn);
    match res{
        Ok(usuario) => {
            return Ok(usuario)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}