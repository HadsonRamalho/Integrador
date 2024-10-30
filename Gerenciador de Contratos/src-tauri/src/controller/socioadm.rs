use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::{controller, model::{self, erro::MeuErro, socioadm::{SocioADM, _cadastra_socio_adm}}};

use super::{formata_cpf, gera_hash};

#[derive(Deserialize, Serialize)]
pub struct SocioAdmInput{
    pub idendereco: String,
    pub nome: String,
    pub cpf: String,
    pub orgaoemissor: String,
    pub estadocivil: String,
    pub nacionalidade: String,
    pub cnpj: String
}

/// ## Recebe os campos necessários para criar um objeto do tipo `serde_json::Value` que seja equivalente ao tipo `SocioADM`, retornando o objeto resultante
/// Primeiro, verifica se algum dos campos está vazio, retornando erro caso ao menos um deles esteja:
/// ```
/// if idendereco.trim().is_empty() || nome.trim().is_empty() || cpf.trim().is_empty()
///     || orgaoemissor.trim().is_empty() || estadocivil.trim().is_empty() 
///     || nacionalidade.trim().is_empty() || cnpj.trim().is_empty(){
///    return Err(MeuErro::CamposVazios.to_string());
/// }
/// ```
/// Em seguida, usa o CPF para criar o ID do SocioADM. Após gerar o ID, faz a formatação do CPF e cria o objeto Value que será retornado:
/// ```
/// let id: String = controller::gera_hash(&cpf);
/// let cpf = formata_cpf(&cpf)?;
/// let socioadm: serde_json::Value = serde_json::json!({
///    "idsocio": id,
///    "idendereco": idendereco,
///     [...]
/// ```
//#[tauri::command]
pub async fn estrutura_socio_adm(input: Json<SocioAdmInput>) -> Result<(StatusCode, Json<SocioADM>), (StatusCode, Json<String>)>{
    let idendereco = input.idendereco.to_string();
    let nome = input.nome.to_string();
    let cpf = input.cpf.to_string();
    let orgaoemissor = input.orgaoemissor.to_string();
    let estadocivil = input.estadocivil.to_string();
    let nacionalidade = input.nacionalidade.to_string();
    let cnpj = input.cnpj.to_string();
    if idendereco.trim().is_empty() || nome.trim().is_empty() || cpf.trim().is_empty()
        || orgaoemissor.trim().is_empty() || estadocivil.trim().is_empty() 
        || nacionalidade.trim().is_empty() || cnpj.trim().is_empty(){
            return Err((StatusCode::BAD_REQUEST, Json(MeuErro::CamposVazios.to_string())));
    }
    let idsocio = gera_hash(&cpf);

    let cpf = match formata_cpf(&cpf){
        Ok(cpf) => {
            cpf
        },
        Err(e) => {
            return Err((StatusCode::BAD_REQUEST, Json(e)))
        }
    };
    let socioadm = SocioADM{
        nome,
        nacionalidade,
        idendereco,
        idsocio,
        cpf,
        cnpj,
        orgaoemissor,
        estadocivil,
        sociostatus: 1
    };
    return Ok((StatusCode::OK, Json(socioadm)));
}

/// ## Recebe um `serde_json::Value` contendo campos equivalentes ao tipo `SocioADM`, 
/// verifica se o Socio está cadastrado no banco e prossegue com o cadastro no banco de dados caso não esteja.
/// Primeiro, converte os campos do `Value` em um objeto `SocioADM`: 
/// ```
/// let cpf = formata_cpf(socioadm["cpf"].as_str().unwrap_or(""))?;
/// let socioadm: model::socioadm::SocioADM = model::socioadm::SocioADM {
///    idsocio,
///    idendereco: socioadm["idendereco"].as_str().unwrap_or("").to_string(),
///    [...]
/// ```
/// Em seguida, faz a cópia do CPF do Socio e usa essa cópia para buscar um registro no banco de dados:
/// ```
/// let cpf_cpy = socioadm.cpf.clone();
/// let resultado_busca: Result<String, mysql_async::Error> = model::socioadm::busca_id_socio_adm(cpf_cpy).await;
/// ```
/// Se a busca retornar um registro, a função retorna um erro, pois não podem ser registrados dois sócios com o mesmo CPF:
/// ```
/// Ok(idsocio) => {
///     println!("IDSOCIO: {}", idsocio);
///     if idsocio != ""{
///         return Err("Sócio já está cadastrado".to_string())
///     }
/// }
/// ```
/// Caso não exista um Socio com o mesmo CPF registrado, chama a função responsável por cadastrar o SocioADM no banco de dados, retornando o ID no final:
/// ```
/// let resultado_cadastro = _cadastra_socio_adm(socioadm).await;
/// match resultado_cadastro{
/// Ok(_) =>{
///     println!("Socio cadastrado");
///     return Ok(idsocio_cpy);
/// }
/// ```
//#[tauri::command]
pub async fn cadastra_socio_adm(input: Json<SocioADM>) -> Result<(StatusCode, Json<String>), (StatusCode, Json<String>)> {
    let idsocio: String = input.idsocio.to_string();
    //let idsocio: (&str, &str) = idsocio.split_at(45 as usize);
    //let idsocio: String = idsocio.0.to_string();
    let idsocio_cpy = idsocio.clone();
    let cpf = match formata_cpf(&input.cpf){
        Ok(cpf) => {
            cpf
        },
        Err(e) => {
            return Err((StatusCode::BAD_REQUEST, Json(e)))
        }
    };
    let idendereco = input.idendereco.to_string();
    let nome = input.nome.to_string();
    let orgaoemissor = input.orgaoemissor.to_string();
    let estadocivil = input.estadocivil.to_string();
    let nacionalidade = input.nacionalidade.to_string();
    let cnpj = input.cnpj.to_string();
    let socioadm: model::socioadm::SocioADM = model::socioadm::SocioADM {
        idsocio,
        idendereco,
        nome,
        cpf: cpf,
        orgaoemissor,
        estadocivil,
        nacionalidade,
        sociostatus: 1,
        cnpj
    };

    // buscar o CPF do socio para não permitir entrada duplicada
    let cpf_cpy = socioadm.cpf.clone();
    let resultado_busca: Result<String, mysql_async::Error> = model::socioadm::busca_id_socio_adm(cpf_cpy).await;
    match resultado_busca{
        Ok(idsocio) => {
            println!("IDSOCIO: {}", idsocio);
            if idsocio != ""{
                return Err((StatusCode::BAD_REQUEST, Json("Este sócio já está cadastrado.".to_string())))
            }
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())))
        }
    }

    let resultado_cadastro = _cadastra_socio_adm(socioadm).await;
    match resultado_cadastro{
        Ok(_) =>{
            println!("Socio cadastrado");
            return Ok((StatusCode::CREATED, Json(idsocio_cpy)));
        }, 
        Err(e) => {
            println!("Erro ao cadastrar o socio adm: {:?}", e); 
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())))
        }
    }
}



/// ## Recebe o ID de um Socio, busca pelo registro no banco de dados e retorna um vetor de `SocioADM` contendo um único valor
/// Primeiro, verifica se `idsocio` está vazio, retornando erro caso esteja:
/// ```
/// if idsocio.trim().is_empty(){
///     return Err("ID do sócio está vazio".to_string())
/// }
/// ```
/// Se a função de busca não retornar erro, é retornado um vetor de `SocioADM`:
/// ```
/// Ok(socioadm) => {
///     return Ok(socioadm)
/// }
/// ```
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

/// ## Recebe o CPF de um SocioADM, busca o registro no banco de dados retorna um `serde_json::Value` contendo os dados recuperados
/// Primeiro, faz a formatação do CPF e usa o resultado para buscar um registro no banco de dados:
/// ```
/// let cpf= formata_cpf(&cpf)?;
/// let resultado_busca = model::socioadm::socio_adm_existente(&cpf).await;
/// ```
/// Se a busca não resultar em um erro, é criado um `Value` com os dados adquiridos, e então o objeto é retornado:
/// ```
/// let socioadm = serde_json::json!({
///     "idsocio": socioadm.idsocio ,
///     "idendereco": socioadm.idendereco,
///     [...]
/// )};
/// return Ok(socioadm)
/// ```
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