use chrono::format;
use mysql_async::{params, prelude::Queryable, Pool, ServerError};

use crate::{controller, model::{self, locatario::Locatario}};

use super::{cria_pool, locadora::formata_cnpj};

#[tauri::command]
pub fn estrutura_locatario(idendereco: String, cnpj: String, nomelocatario: String) -> Result<serde_json::Value, String>{
    let cnpj = match controller::locadora::formata_cnpj(&cnpj){
        Ok(_) => {
            cnpj
        },
        Err(e) => {
            return Err(e);
        }
    };
    let id: String = controller::gera_hash(&cnpj);

    if nomelocatario.trim().len() < 5{
        return Err("Erro: Nome do locatário é muito curto.".to_string());
    }

    let locatario: serde_json::Value = serde_json::json!({
        "idlocatario": id,
        "idendereco": idendereco,
        "cnpj": cnpj,
        "nomelocatario": nomelocatario
    });
    return Ok(locatario)
}

#[tauri::command]
pub async fn cadastra_locatario(locatario: serde_json::Value) -> Result<String, String>{
    let idlocatario: String = locatario["idlocatario"].as_str().unwrap_or("").to_string();
    let idlocatario: (&str, &str) = idlocatario.split_at(45 as usize);
    let idsocio = idlocatario.0.to_string();
    let idlocatario: String = idlocatario.0.to_string();
    let locatario: model::locatario::Locatario = model::locatario::Locatario {
        idlocatario: idlocatario,
        idendereco: locatario["idendereco"].as_str().unwrap_or("").to_string(),
        cnpj: locatario["cnpj"].as_str().unwrap_or("").to_string(),
        nomelocatario: locatario["nomelocatario"].as_str().unwrap_or("").to_string(),
        idsocio: idsocio
    };

    let resultado_busca: Result<String, mysql_async::Error> = _busca_id_locatario(&locatario.cnpj).await;

    match resultado_busca{
        Ok(resultado) => {
            if resultado == ""{
                let _resultado_cadastro = _cadastra_locatario(locatario).await;
                return Ok("Locatario cadastrado com sucesso".to_string());
            }
            return Err("Erro: Locatario já cadastrado".to_string());
        },
        Err(erro) => {
            return Err(erro.to_string());
        }
    }
}


#[tauri::command]
pub async fn busca_id_locatario(cnpj: &str) -> Result<String, String>{
    let resultado: Result<String, mysql_async::Error> = _busca_id_locatario(cnpj).await;
    match resultado{
        Ok(id) =>{
            return Ok(id);
        },
        Err(e) => {
            return Err(e.to_string());
        }
    }
}

pub async fn _busca_id_locatario(cnpj: &str) -> Result<String, mysql_async::Error>{
    let pool = match controller::cria_pool().await {
        Ok(pool) => {
            pool
        }, 
        Err(e) =>{
            return Err(e)
        }
    };
    let mut conn = pool.get_conn().await?;
    let resultado_busca: Result<Option<String>, mysql_async::Error> = conn.exec_first("SELECT idlocatario FROM locatario WHERE cnpj = :cnpj",
        params!{"cnpj" => cnpj}).await;
    match resultado_busca{
        Ok(id) => {
            match id {
                Some(id) => {
                    return Ok(id);
                }, None =>{
                    return Ok("".to_string());
                }
            }
        },
        Err(e) => {
            return Err(e);
        }
    }
}

pub async fn _cadastra_locatario(locatario: Locatario) -> Result<(), mysql_async::Error>{
    let pool = match controller::cria_pool().await {
        Ok(pool) => {
            pool
        }, 
        Err(e) =>{
            return Err(e)
        }
    };
    let mut conn = pool.get_conn().await?;
    let resultado_insert =
         conn.exec_drop("INSERT INTO locatario (idlocatario, idendereco, cnpj, 
         numerocontabanco, numeroagenciabanco, nomebanco, nomelocatario, idsocio)
          VALUES (:idlocatario, :idendereco, :cnpj, :numerocontabanco, :numeroagenciabanco, :nomebanco, :nomelocatario, :idsocio);", 
         params! {"idlocatario" =>  locatario.idlocatario, "idendereco" => locatario.idendereco, "cnpj" => locatario.cnpj,
            "nomelocatario" => locatario.nomelocatario, "idsocio" => locatario.idsocio}).await;
    match resultado_insert{
        Ok(_) => {
            println!("Locatario cadastrado");
        }, 
        Err(e) => {
            println!("{:?}", e);
            return Err(e);
        }
    }
    return Ok(());
}

#[tauri::command]
pub async fn busca_nome_locatario(cnpjlocatario: String) -> Result<Vec<Locatario>, String>{
    let resultado_busca = model::locatario::busca_locatario_nome(&cnpjlocatario).await;
    match resultado_busca {
        Ok(locatario) =>{
            return Ok(locatario);
        },
        Err(e) => {
            return Err(e.to_string());
        }
    }
}

pub async fn _busca_nome_locatario(cnpjlocatario: String) ->Result<String, mysql_async::Error>{

    let server_error = mysql_async::ServerError{
        code: 1045, //Código de erro
        message: "Erro: Não foi encontado um cliente com este CNPJ.".to_string(),
        state: "28000".to_string()
    };

    let pool = match cria_pool().await{
        Ok(pool) => {
            pool
        },
        Err(e) => {
            return Err(e);
        }
    };
    let mut conn = pool.get_conn().await?;

    let resultado_busca: Option<String> = 
        conn.exec_first("SELECT nomelocatario FROM locatario WHERE cnpj = :cnpjlocatario;", 
        params! {"cnpjlocatario" => cnpjlocatario}).await?;

    match resultado_busca{
        Some(nomelocatario) => {
            return Ok(nomelocatario);
        }, 
        None => {
            return Err(mysql_async::Error::Server(server_error))
        }
    }
}