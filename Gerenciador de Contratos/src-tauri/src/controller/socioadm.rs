use crate::{controller, model::{self, erro::MeuErro, socioadm::_cadastra_socio_adm}};

use super::formata_cpf;

#[tauri::command]
pub fn estrutura_socio_adm(idendereco: String, nome: String, cpf: String, orgaoemissor: String, estadocivil: String, nacionalidade: String) -> Result<serde_json::Value, String>{
    
    if idendereco.is_empty() || nome.is_empty() || cpf.is_empty()
        || orgaoemissor.is_empty() || estadocivil.is_empty() 
        || nacionalidade.is_empty(){
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
        "nacionalidade": nacionalidade
    });
    return Ok(socioadm);
}

#[tauri::command]
pub async fn cadastra_socio_adm(socioadm: serde_json::Value) -> Result<String, String> {
    let idsocio: String = socioadm["idsocio"].as_str().unwrap_or("").to_string();
    let idsocio: (&str, &str) = idsocio.split_at(45 as usize);
    let idsocio: String = idsocio.0.to_string();
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
        sociostatus: 1
    };

    // buscar o id do socio para não permitir entrada duplicada
    // let resultado_busca: Result<String, mysql_async::Error> =
    // model::socioadm::_busca_id_socio_adm(&socioadm.cpf).await;

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