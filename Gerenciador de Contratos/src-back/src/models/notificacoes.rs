use chrono::NaiveDateTime;
use diesel::{prelude::{Insertable, Queryable}, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::notificacoes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Notificacao{
    pub idnotificacao: String,
    pub idusuario: String,
    pub titulo: String,
    pub mensagem: String,
    pub onclick: String,
    pub status: String,
    pub datacriacao: NaiveDateTime
}

pub async fn cadastra_notificacao(conn: &mut PgConnection, notificacao: Notificacao)
    -> Result<Notificacao, String>{
    use crate::schema::notificacoes::dsl::*;

    let res: Result<Notificacao, diesel::result::Error> = diesel::insert_into(notificacoes)
      .values(notificacao)
      .get_result(conn);

    match res{
      Ok(notificacao) => {
        return Ok(notificacao)
      },
      Err(e) => {
        return Err(e.to_string())
      }
    }
}

pub async fn busca_notificacoes_idusuario(conn: &mut PgConnection, id: String)
    -> Result<Vec<Notificacao>, String>{
    use crate::schema::notificacoes::dsl::*;

    let res: Result<Vec<Notificacao>, diesel::result::Error> = notificacoes
      .filter(idusuario.eq(id))
      .order_by(datacriacao.desc())
      .get_results(conn);

    match res{
      Ok(res) => {
        return Ok(res)
      },
      Err(e) => {
        return Err(e.to_string())
      }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NotificaoStatusInput{
    pub id: String,
    pub novostatus: String
}

pub async fn atualiza_status_notificacao(conn: &mut PgConnection, atualizacao: NotificaoStatusInput)
    -> Result<String, String>{
    use crate::schema::notificacoes::dsl::*;

    let res: Result<Notificacao, diesel::result::Error> = diesel::update(notificacoes)
      .filter(idnotificacao.eq(atualizacao.id))
      .set(status.eq(atualizacao.novostatus))
      .get_result(conn);

    match res{
      Ok(notificacao) => {
        return Ok(notificacao.idnotificacao)
      },
      Err(e) => {
        return Err(e.to_string())
      }
    }
}