use mysql_async::{params, prelude::Queryable};

use crate::{controller, model::{self, erro::MeuErro, locatario::Locatario}};

use super::locadora::formata_cnpj;

#[tauri::command]
pub fn estrutura_locatario(idendereco: String, cnpj: String, nomelocatario: String, idsocio: String) -> Result<serde_json::Value, String>{
    if idendereco.trim().is_empty() || cnpj.trim().is_empty() || 
        nomelocatario.trim().is_empty() || idsocio.trim().is_empty(){
        return Err(MeuErro::CamposVazios.to_string())
    }
    let cnpj = match controller::locadora::formata_cnpj(&cnpj){
        Ok(_) => {
            cnpj
        },
        Err(e) => {
            return Err(e);
        }
    };
    let id: String = controller::gera_hash(&cnpj);

    let locatario: serde_json::Value = serde_json::json!({
        "idlocatario": id,
        "idendereco": idendereco,
        "cnpj": cnpj,
        "nomelocatario": nomelocatario,
        "idsocio": idsocio
    });
    return Ok(locatario)
}

#[tauri::command]
pub async fn cadastra_locatario(locatario: serde_json::Value) -> Result<String, String>{
    let idlocatario: String = locatario["idlocatario"].as_str().unwrap_or("").to_string();
    let idlocatario: (&str, &str) = idlocatario.split_at(45 as usize);
    let idsocio: String = locatario["idsocio"].as_str().unwrap_or("").to_string();
    let idlocatario: String = idlocatario.0.to_string();
    let idlocatario_cpy = idlocatario.clone();
    let locatario: model::locatario::Locatario = model::locatario::Locatario {
        idlocatario: idlocatario,
        idendereco: locatario["idendereco"].as_str().unwrap_or("").to_string(),
        cnpj: locatario["cnpj"].as_str().unwrap_or("").to_string(),
        nomelocatario: locatario["nomelocatario"].as_str().unwrap_or("").to_string(),
        idsocio: idsocio,
        locatariostatus: 1
    };

    if locatario.idlocatario.is_empty() || locatario.idsocio.is_empty() || 
        locatario.cnpj.is_empty() || locatario.nomelocatario.is_empty() || 
        locatario.idendereco.is_empty(){
        return Err(MeuErro::CamposVazios.to_string())
    }

    let resultado_busca: Result<String, mysql_async::Error> = _busca_id_locatario(&locatario.cnpj).await;

    let id = match resultado_busca{
        Ok(resultado) => {
            resultado
        },
        Err(erro) => {
            return Err(erro.to_string());
        }
    };
    if !id.is_empty(){
        return Err("Erro: Locatario já cadastrado".to_string());        
    }
    let resultado_cadastro = model::locatario::_cadastra_locatario(locatario).await;
    match resultado_cadastro{
        Ok(_) => {
            println!("idlocatario: {}", idlocatario_cpy);
            return Ok(idlocatario_cpy.to_string());
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}


#[tauri::command]
pub async fn busca_id_locatario(cnpj: &str) -> Result<String, String>{
    if cnpj.trim().is_empty(){
        return Err(MeuErro::CnpjVazio.to_string())
    }
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
    let pool = controller::cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let resultado_busca: Result<Option<String>, mysql_async::Error> = conn.exec_first("SELECT idlocatario FROM locatario WHERE cnpj = :cnpj",
        params!{"cnpj" => cnpj}).await;
    let id = match resultado_busca{
        Ok(id) => {
            id
        },
        Err(e) => {
            return Err(e);
        }
    };
    match id {
        Some(id) => {
            return Ok(id);
        }, 
        None =>{
            return Ok("".to_string());
        }
    }
}

#[tauri::command]
pub async fn busca_locatario_nome(nomelocatario: String) -> Result<Vec<Locatario>, String>{
    if nomelocatario.trim().is_empty(){
        return Err("Erro: O nome do locatário não pode estar vazio.".to_string())
    }
    let resultado_busca = model::locatario::busca_locatario_nome(&nomelocatario).await;
    match resultado_busca {
        Ok(locatario) =>{
            if locatario.is_empty(){
                return Err("Locatário não encontrado".to_string());
            }
            return Ok(locatario)
        },
        Err(e) => {
            return Err(e.to_string());
        }
    }
}

#[tauri::command]
pub async fn busca_locatario_cnpj(cnpj: String) -> Result<Vec<Locatario>, String>{
    if cnpj.trim().is_empty(){
        return Err(MeuErro::CnpjVazio.to_string())
    }
    let cnpj = match formata_cnpj(&cnpj){
        Ok(cnpj) => {cnpj},
        Err(e) => {
            return Err(e);
        }
    };
    let resultado_busca = model::locatario::busca_locatario_cnpj(&cnpj).await;
    match resultado_busca {
        Ok(locatario) =>{
            return Ok(locatario);
        },
        Err(e) => {
            return Err(e.to_string());
        }
    }
}
