use crate::model::erro::MeuErro;
use crate::model::locadora::Locadora;
use crate::model::locadora::_cadastra_locadora;
use crate::model;
use crate::controller;

/// ## Recebe campos separados e transforma em um serde_json::Value contendo dados de uma Locadora
/// Primeiro, verifica se algum dos parâmetros está vazio, retornando erro caso ao menos um esteja.
/// Chama a função responsável por formatar o CNPJ que foi recebido como parâmetro e armazena o valor em `cnpjalterado`:
/// ```
/// let cnpjalterado = formata_cnpj(&cnpj)?;
/// ```
/// Em seguida, usa `cnpjalterado` para gerar o ID da locadora:
/// ```
/// let id: String = controller::gera_hash(&cnpjalterado);
/// ```
/// Usa os campos recebidos para montar e retornar um objeto serde_json::Value que contém dados da Locadora:
/// ```
/// let locadora: serde_json::Value = serde_json::json!({
///     "idlocadora": id,
///     "idendereco": idendereco,
///     "cnpj": cnpjalterado,
///     "numerocontabanco": numerocontabanco,
///     "numeroagenciabanco": numeroagenciabanco,
///     "nomebanco": nomebanco,
///     "nomelocadora": nomelocadora,
///     "idsocio": idsocio
/// });
/// ```
#[tauri::command]
pub fn estrutura_locadora(idendereco: String, cnpj: String, numerocontabanco: String, numeroagenciabanco: String, nomebanco: String, nomelocadora: String, idsocio: String) -> Result<serde_json::Value, String>{
    if idendereco.trim().is_empty() || cnpj.trim().is_empty() || numerocontabanco.trim().is_empty()
        || numeroagenciabanco.trim().is_empty() || nomebanco.trim().is_empty() || nomelocadora.trim().is_empty(){
        return Err("Erro: Um ou mais campos estão vazios.".to_string());
    }
    let cnpjalterado = formata_cnpj(&cnpj)?;
    let id: String = controller::gera_hash(&cnpjalterado);
    let locadora: serde_json::Value = serde_json::json!({
        "idlocadora": id,
        "idendereco": idendereco,
        "cnpj": cnpjalterado,
        "numerocontabanco": numerocontabanco,
        "numeroagenciabanco": numeroagenciabanco,
        "nomebanco": nomebanco,
        "nomelocadora": nomelocadora,
        "idsocio": idsocio
    });
    return Ok(locadora);
}

#[tauri::command]
pub async fn cadastra_locadora(locadora: serde_json::Value) -> Result<String, String> {
    let locadora: Result<Locadora, String> = valida_locadora(locadora);
    match locadora{
        Ok(_) => {},
        Err(e) => {return Err(e)}
    }
    let locadora: Locadora = locadora.unwrap();
    let idlocadora = locadora.idlocadora.clone();
    let resultado_busca: Result<String, mysql_async::Error> =
        model::locadora::_busca_id_locadora(&locadora.cnpj).await;
    match resultado_busca {
        Ok(resultado) => {
            if resultado.is_empty() {
                let _resultado_cadastro = _cadastra_locadora(locadora).await;
                return Ok(idlocadora);
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
    if cnpj.trim().is_empty(){
        return Err(MeuErro::CnpjVazio.to_string());
    }
    let cnpj = formata_cnpj(cnpj)?;
    let resultado: Result<String, mysql_async::Error> = model::locadora::_busca_id_locadora(&cnpj).await;
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
    let cnpj = locadora["cnpj"].as_str().unwrap_or("").to_string();
    let cnpj = formata_cnpj(&cnpj)?;
    let locadora: model::locadora::Locadora = model::locadora::Locadora {
        idlocadora: idlocadora,
        idendereco: locadora["idendereco"].as_str().unwrap_or("").to_string(),
        cnpj: cnpj,
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
        idsocio: locadora["idsocio"].as_str().unwrap_or("").to_string(),
        locadorastatus: 1
    };
    if locadora.idendereco.trim().is_empty() || locadora.cnpj.trim().is_empty() || 
        locadora.numerocontabanco.trim().is_empty()
        || locadora.numeroagenciabanco.trim().is_empty() || 
        locadora.nomebanco.trim().is_empty() || locadora.nomelocadora.trim().is_empty() || locadora.idsocio.trim().is_empty(){
            return Err(MeuErro::CamposVazios.to_string());
    }
    return Ok(locadora);
}

#[tauri::command]
pub async fn locadora_existente(cnpj: &str) -> Result<serde_json::Value, String>{
    let cnpj = formata_cnpj(cnpj)?;
    let locadora = match model::locadora::locadora_existente(&cnpj).await{
        Ok(locadora) => {locadora},
        Err(e) => {return Err(e.to_string())}
    };
    let locadora = serde_json::json!({
        "idlocadora": locadora.idlocadora,
        "idendereco": locadora.idendereco,
        "cnpj": locadora.cnpj,
        "numerocontabanco": locadora.numerocontabanco,
        "numeroagenciabanco": locadora.numeroagenciabanco,
        "nomebanco": locadora.nomebanco,
        "nomelocadora": locadora.nomelocadora,
        "idsocio": locadora.idsocio,
        "locadorastatus": locadora.locadorastatus
    });
    return Ok(locadora)
}

pub fn formata_cnpj(cnpj: &str) -> Result<String, String>{
    let cnpj_numeros: Vec<char> = cnpj
        .chars()
        .filter(|c: &char| c.is_digit(10))
        .collect();
    if cnpj_numeros.len() != 14{
        return Err("Erro: CNPJ de tamanho inválido.".to_string())
    }
    let mut cnpj: Vec<char> = cnpj_numeros;
    cnpj.insert(2, '.');
    cnpj.insert(6, '.');
    cnpj.insert(10, '/');
    cnpj.insert(15, '-');
    let mut cnpjfinal: String = "".to_string();
    for u in cnpj{
        cnpjfinal.push(u);
    }
    return Ok(cnpjfinal);
}

#[tauri::command]
pub async fn verifica_usuario_socio_locadora(idusuario: String, cnpj: String) -> Result<(), String>{
    if idusuario.trim().is_empty() || cnpj.trim().is_empty(){
       return Err(MeuErro::CamposVazios.to_string())
    }
    let cnpj = formata_cnpj(&cnpj)?;
    let resultado_verificacao = model::locadora::verifica_usuario_socio_locadora(idusuario, cnpj).await;
    match resultado_verificacao{
        Ok(_) => {return Ok(())},
        Err(e) => {return Err(e.to_string())}
    }
}

