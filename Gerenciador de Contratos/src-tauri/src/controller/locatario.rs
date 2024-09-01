use crate::{controller, model};

#[tauri::command]
pub fn estrutura_locatario(idendereco: String, cnpj: String, nomelocatario: String) -> Result<serde_json::Value, bool>{
    let id: String = controller::gera_hash(&cnpj);
    let locatario: serde_json::Value = serde_json::json!({
        "idlocatario": id,
        "idendereco": idendereco,
        "cnpj": cnpj,
        "nomelocadora": nomelocatario
    });
    return Ok(locatario)
}

#[tauri::command]
pub async fn cadastra_locatario(locatario: serde_json::Value) -> Result<String, String>{
    let idlocatario: String = locatario["idlocatario"].as_str().unwrap_or("").to_string();
    let idlocatario: (&str, &str) = idlocatario.split_at(45 as usize);
    let idsocio = idlocatario.0.to_string();
    let idlocatario: String = idlocatario.0.to_string();
    let _locatario: model::locatario::Locatario = model::locatario::Locatario {
        idlocatario: idlocatario,
        idendereco: locatario["idendereco"].as_str().unwrap_or("").to_string(),
        cnpj: locatario["cnpj"].as_str().unwrap_or("").to_string(),
        nomelocatario: locatario["nomelocatario"].as_str().unwrap_or("").to_string(),
        idsocio: idsocio
    };

    return Ok("".to_string())

    /*let resultado_busca: Result<String, mysql_async::Error> = model::locatario::_busca_id_locatario(&locatario.cnpj).await;

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
    }*/
}

/*
#[tauri::command]
pub async fn busca_id_locatario() -> Result<String, String>{
    let resultado: Result<String, mysql_async::Error> = model::locadora::_busca_id_locatario("000123").await;
    match resultado{
        Ok(id) =>{
            return Ok(id);
        },
        Err(e) => {
            return Err(e.to_string());
        }
    }
}*/