use axum::Json;
use diesel::prelude::*;
use hyper::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::controllers::gera_hash;
use crate::models::imagens::recupera_imagem;
use crate::schema::imagens_maquinas::{self};
use diesel::Queryable;

#[derive(Queryable, Selectable, Serialize, Deserialize, Insertable)]
#[table_name = "imagens_maquinas"]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ImagemMaquina{
    pub idimagemmaquina: String,
    pub idimagem: String,
    pub idmaquina: String
}

pub async fn cadastra_imagem_maquina(conn: &mut PgConnection, idmaquina: String, idimagem: String)
    -> Result<String, String>{
    use crate::schema::imagens_maquinas;
    let id = gera_hash(&idmaquina);
    let nova_imagem = ImagemMaquina {
        idimagemmaquina: id.clone(),
        idimagem,
        idmaquina
    };

    let res = diesel::insert_into(imagens_maquinas::table)
        .values(&nova_imagem)
        .execute(conn);

    match res {
        Ok(_) => Ok(id),
        Err(e) => {
            println!("Erro [BACK/DB]: {}", e);
            return Err(e.to_string())
        },
    }
}

pub async fn recupera_imagem_maquina(conn: &mut PgConnection, id: String)
    -> Result<(HeaderMap, String), String> {
    use crate::schema::imagens_maquinas::dsl::*;

    let imagem: Result<ImagemMaquina, diesel::result::Error> = imagens_maquinas
        .filter(idmaquina.eq(id))
        .first(conn);

    let imagem = match imagem{
        Ok(imagem) => {imagem},
        Err(e) => {
            return Err(e.to_string())
        }
    };

    let res = recupera_imagem(conn, imagem.idimagem).await;

    match res{
        Ok(img) =>{
            return Ok(img)
        },
        Err(e) => {
            println!("Erro [BACK/DB]: {}", e);
            return Err(e)
        }
    }
}

pub async fn recupera_imagens_maquina(conn: &mut PgConnection, id: Json<String>) -> Result<Vec<String>, String> {
    use crate::schema::imagens_maquinas::dsl::*;

    let imagem: Result<Vec<ImagemMaquina>, diesel::result::Error> = imagens_maquinas
        .filter(idmaquina.eq(id.0))
        .get_results(conn);

    let imagem = match imagem{
        Ok(imagem) => {imagem},
        Err(e) => {
            return Err(e.to_string())
        }
    };

    let mut imagens = Vec::new();
    for img in imagem{
        let res = recupera_imagem(conn, img.idimagem).await;
        match res{
            Ok(img) => {
                imagens.push(img.1)
            },
            Err(e) => {
                println!("Erro [BACK/DB]: {}", e);
                return Err(e)
            }
        }
    }

    return Ok(imagens)
}

