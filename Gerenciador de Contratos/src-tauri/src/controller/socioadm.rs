use crate::{controller, model::{self, erro::MeuErro, socioadm::{SocioADM, _cadastra_socio_adm}}};

use super::formata_cpf;

#[tauri::command]
pub fn estrutura_socio_adm(idendereco: String, nome: String, cpf: String, orgaoemissor: String, estadocivil: String, nacionalidade: String, cnpj: String) -> Result<serde_json::Value, String>{
    
    if idendereco.trim().is_empty() || nome.trim().is_empty() || cpf.trim().is_empty()
        || orgaoemissor.trim().is_empty() || estadocivil.trim().is_empty() 
        || nacionalidade.trim().is_empty() || cnpj.trim().is_empty(){
            return Err(MeuErro::CamposVazios.to_string());
    }
    
    let id: String = controller::gera_hash(&cpf);
    let cpf = formata_cpf(&cpf)?;
    let socioadm: serde_json::Value = serde_json::json!({
        "idsocio": id,
        "idendereco": idendereco,
        "nome": nome,
        "cpf": cpf,
        "orgaoemissor": orgaoemissor,
        "estadocivil": estadocivil,
        "nacionalidade": nacionalidade,
        "cnpj": cnpj
    });
    return Ok(socioadm);
}

#[tauri::command]
pub async fn cadastra_socio_adm(socioadm: serde_json::Value) -> Result<String, String> {
    let idsocio: String = socioadm["idsocio"].as_str().unwrap_or("").to_string();
    //let idsocio: (&str, &str) = idsocio.split_at(45 as usize);
    //let idsocio: String = idsocio.0.to_string();
    let idsocio_cpy = idsocio.clone();
    let cpf = formata_cpf(socioadm["cpf"].as_str().unwrap_or(""))?;
    let socioadm: model::socioadm::SocioADM = model::socioadm::SocioADM {
        idsocio,
        idendereco: socioadm["idendereco"].as_str().unwrap_or("").to_string(),
        nome: socioadm["nome"].as_str().unwrap_or("").to_string(),
        cpf: cpf,
        orgaoemissor: socioadm["orgaoemissor"]
            .as_str()
            .unwrap_or("")
            .to_string(),
        estadocivil: socioadm["estadocivil"]
            .as_str()
            .unwrap_or("")
            .to_string(),
        nacionalidade: socioadm["nacionalidade"].as_str().unwrap_or("").to_string(),
        sociostatus: 1,
        cnpj: socioadm["cnpj"].as_str().unwrap_or("").to_string()
    };

    // buscar o CPF do socio para não permitir entrada duplicada
    let cpf_cpy = socioadm.cpf.clone();
    let resultado_busca: Result<String, mysql_async::Error> = model::socioadm::busca_id_socio_adm(cpf_cpy).await;
    match resultado_busca{
        Ok(idsocio) => {
            println!("IDSOCIO: {}", idsocio);
            if idsocio != ""{
                return Err("Sócio já está cadastrado".to_string())
            }
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }

    let resultado_cadastro = _cadastra_socio_adm(socioadm).await;
    match resultado_cadastro{
        Ok(_) =>{
            println!("Socio cadastrado");
            return Ok(idsocio_cpy);
        }, 
        Err(e) => {
            println!("Erro ao cadastrar o socio adm: {:?}", e); 
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub fn estrutura_primeiro_socio(idendereco: String, nome: String, cpf: String, orgaoemissor: String, estadocivil: String, nacionalidade: String, idsocio: String, cnpj: String) -> Result<serde_json::Value, String>{
    
    if idendereco.trim().is_empty() || nome.trim().is_empty() || cpf.trim().is_empty()
        || orgaoemissor.trim().is_empty() || estadocivil.trim().is_empty() 
        || nacionalidade.trim().is_empty() || cnpj.trim().is_empty(){
            return Err(MeuErro::CamposVazios.to_string());
    }
    
    let cpf = formata_cpf(&cpf)?;
    let socioadm: serde_json::Value = serde_json::json!({
        "idsocio": idsocio,
        "idendereco": idendereco,
        "nome": nome,
        "cpf": cpf,
        "orgaoemissor": orgaoemissor,
        "estadocivil": estadocivil,
        "nacionalidade": nacionalidade,
        "cnpj": cnpj
    });
    return Ok(socioadm);
}

#[tauri::command]
pub async fn busca_socio_adm_id(idsocio: String) -> Result<Vec<SocioADM>, String>{
    if idsocio.trim().is_empty(){
        return Err("ID do sócio está vazio".to_string())
    }
    let resultado_busca: Result<Vec<SocioADM>, mysql_async::Error> = model::socioadm::busca_socio_adm_id(idsocio).await;
    match resultado_busca{
        Ok(socioadm) => {
            return Ok(socioadm)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn socio_adm_existente(cpf: String) -> Result<serde_json::Value, String>{
    println!("{}", cpf);
    let cpf= formata_cpf(&cpf)?;
    let resultado_busca = model::socioadm::socio_adm_existente(&cpf).await;
    let socioadm = match resultado_busca{
        Ok(socioadm) => {socioadm},
        Err(e) => return Err(e.to_string())
    };
    let socioadm = serde_json::json!({
        "idsocio": socioadm.idsocio ,
        "idendereco": socioadm.idendereco,
        "nome":socioadm.nome ,
        "cpf": socioadm.cpf ,
        "orgaoemissor": socioadm.orgaoemissor ,
        "estadocivil": socioadm.estadocivil ,
        "nacionalidade": socioadm.nacionalidade ,
        "sociostatus": socioadm.sociostatus ,
        "cnpj": socioadm.cnpj 
    });
    return Ok(socioadm)
}