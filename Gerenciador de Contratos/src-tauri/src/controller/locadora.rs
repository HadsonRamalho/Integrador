use crate::model::locadora::Locadora;
use crate::model::locadora::_cadastra_locadora;
use crate::model;
use crate::controller;

/// Função para criar uma estrutura de dados para uma locadora.
///
/// # Parâmetros
/// - idendereco: Identificador do endereço associado à locadora.
/// - cnpj: Cadastro Nacional da Pessoa Jurídica da locadora.
/// - numerocontabanco: Número da conta bancária da locadora.
/// - numeroagenciabanco: Número da agência bancária da locadora.
/// - nomebanco: Nome do banco onde a locadora possui conta.
///
/// # Processo
/// 1. Criptografa o CNPJ da locadora para criar um identificador único.
/// 2. Cria um objeto JSON com os seguintes dados:
///    - `idendereco`: Identificador do endereço.
///    - `idlocadora`: Identificador único criptografado para a locadora.
///    - `cnpj`: CNPJ da locadora.
///    - `numerocontabanco`: Número da conta bancária.
///    - `numeroagenciabanco`: Número da agência bancária.
///    - `nomebanco`: Nome do banco.
///
/// # Retornos
/// - Result<serde_json::Value, bool>: Retorna `Ok(locadora)` contendo os dados da locadora em formato JSON.
///   Retorna `Ok(false)` se houver algum problema na criação do objeto JSON (o que não é esperado neste caso).

#[tauri::command]
pub fn estrutura_locadora(idendereco: String, cnpj: String, numerocontabanco: String, numeroagenciabanco: String, nomebanco: String, nomelocadora: String) -> Result<serde_json::Value, String>{
    if idendereco.is_empty() || cnpj.is_empty() || numerocontabanco.is_empty()
        || numeroagenciabanco.is_empty() || nomebanco.is_empty() || nomelocadora.is_empty(){
            return Err("Erro: Um ou mais campos estão vazios.".to_string());
    }
    let id: String = controller::gera_hash(&cnpj);
    let locadora: serde_json::Value = serde_json::json!({
        "idlocadora": id,
        "idendereco": idendereco,
        "cnpj": cnpj,
        "numerocontabanco": numerocontabanco,
        "numeroagenciabanco": numeroagenciabanco,
        "nomebanco": nomebanco,
        "nomelocadora": nomelocadora
    });
    return Ok(locadora);
}

#[tauri::command]
pub async fn cadastra_locadora(locadora: serde_json::Value) -> Result<(), String> {
    let locadora = valida_locadora(locadora);
    match locadora{
        Ok(_) => {},
        Err(e) => {return Err(e)}
    }
    let locadora = locadora.unwrap();
    let resultado_busca: Result<String, mysql_async::Error> =
        model::locadora::_busca_id_locadora(&locadora.cnpj).await;

    match resultado_busca {
        Ok(resultado) => {
            if resultado == "" {
                let _resultado_cadastro = _cadastra_locadora(locadora).await;
                return Ok(());
            }
            return Err("Erro: Locadora já cadastrada".to_string());
        }
        Err(erro) => {
            return Err(erro.to_string());
        }
    }
}

#[tauri::command]
pub async fn busca_id_locadora(cnpj: &str) -> Result<String, String>{
    if cnpj.is_empty(){
        return Err("Erro: O parâmetro CNPJ está vazio".to_string())
    }
    let cnpj = cnpj.trim(); // remover traços e pontos
    let resultado: Result<String, mysql_async::Error> = model::locadora::_busca_id_locadora(cnpj).await;
    match resultado{
        Ok(id) =>{
            return Ok(id);
        }
        Err(e) => {
            return Err(e.to_string());
        }
    }
}

fn valida_locadora(locadora: serde_json::Value) -> Result<Locadora, String>{
    let idlocadora: String = locadora["idlocadora"].as_str().unwrap_or("").to_string();
    let idlocadora: (&str, &str) = idlocadora.split_at(45 as usize);
    let idlocadora: String = idlocadora.0.to_string();
    let locadora: model::locadora::Locadora = model::locadora::Locadora {
        idlocadora: idlocadora,
        idendereco: locadora["idendereco"].as_str().unwrap_or("").to_string(),
        cnpj: locadora["cnpj"].as_str().unwrap_or("").to_string(),
        numerocontabanco: locadora["numerocontabanco"]
            .as_str()
            .unwrap_or("")
            .to_string(),
        numeroagenciabanco: locadora["numeroagenciabanco"]
            .as_str()
            .unwrap_or("")
            .to_string(),
        nomebanco: locadora["nomebanco"].as_str().unwrap_or("").to_string(),
        nomelocadora: locadora["nomelocadora"].as_str().unwrap_or("").to_string(),
    };
    if locadora.idendereco.is_empty() || locadora.cnpj.is_empty() || locadora.numerocontabanco.is_empty()
        || locadora.numeroagenciabanco.is_empty() || locadora.nomebanco.is_empty() || locadora.nomelocadora.is_empty(){
            return Err("Erro: Um ou mais campos estão vazios.".to_string());
    }
    return Ok(locadora);
}