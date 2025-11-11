use diesel::prelude::*;
use hyper::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::controllers::gera_hash;
use crate::schema::imagens::{self};
use diesel::Queryable;

#[derive(Queryable, Selectable, Serialize, Deserialize, Insertable)]
#[table_name = "imagens"]
pub struct Imagem { 
    pub idimagem: String,
    pub nome: String,
    pub link: String
}

pub async fn cadastra_imagem(conn: &mut PgConnection, nome: String) -> Result<String, String> {
    use crate::schema::imagens;
    let id = gera_hash(&nome);
    let nova_imagem = Imagem {
        idimagem: id.clone(),
        nome,
        link: "".to_string()
    };

    let res = diesel::insert_into(imagens::table)
        .values(&nova_imagem)
        .execute(conn);

    match res {
        Ok(_) => Ok(id),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn atualiza_link_imagem(conn: &mut PgConnection, id: String, novolink: String)
    -> Result<String, String>{
    use self::imagens::dsl::*;

    let res = diesel::update(imagens)
        .filter(idimagem.eq(id))
        .set(link.eq(novolink))
        .execute(conn);

    match res{
        Ok(qtd) => {
            println!("{} imagens atualizadas", qtd);
            return Ok(qtd.to_string())
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

pub async fn recupera_imagem(conn: &mut PgConnection, id: String) -> Result<(HeaderMap, String), String> {
    use crate::schema::imagens::dsl::*;

    let imagem: Result<Imagem, diesel::result::Error> = imagens.filter(idimagem.eq(id)).first(conn);

    let imagem = match imagem{
        Ok(imagem) => {imagem},
        Err(e) => {
            return Err(e.to_string())
        }
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::header::HeaderValue::from_static("image/jpeg"),
    );

    return Ok((headers, imagem.link))
}
