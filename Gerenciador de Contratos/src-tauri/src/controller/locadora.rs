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
pub fn estrutura_locadora(idendereco: String, cnpj: String, numerocontabanco: String, numeroagenciabanco: String, nomebanco: String, nomelocadora: String, idsocio: String) -> Result<serde_json::Value, String>{
    if nomebanco.trim().len() < 4{
        return Err("Erro: Nome do banco é inválido.".to_string());
    }
    if idendereco.trim().is_empty() || cnpj.trim().is_empty() || numerocontabanco.trim().is_empty()
        || numeroagenciabanco.trim().is_empty() || nomebanco.trim().is_empty() || nomelocadora.trim().is_empty(){
        return Err("Erro: Um ou mais campos estão vazios.".to_string());
    }
    let cnpj = match formata_cnpj(&cnpj){
        Ok(_) =>{
            cnpj
        },
        Err(e) => {
            return Err(e);
        }
    };
    let id: String = controller::gera_hash(&cnpj);
    let locadora: serde_json::Value = serde_json::json!({
        "idlocadora": id,
        "idendereco": idendereco,
        "cnpj": cnpj,
        "numerocontabanco": numerocontabanco,
        "numeroagenciabanco": numeroagenciabanco,
        "nomebanco": nomebanco,
        "nomelocadora": nomelocadora,
        "idsocio": idsocio
    });
    return Ok(locadora);
}

/// Função para cadastrar uma locadora no banco de dados.
///
/// Esta função valida os dados da locadora fornecidos em formato JSON e tenta cadastrar
/// a locadora no banco de dados, se ainda não existir um registro com o mesmo CNPJ.
///
/// # Parâmetros
/// - `locadora`: Um objeto `serde_json::Value` contendo as informações da locadora a ser cadastrada.
///
/// # Retornos
/// - `Ok(())`: Se a locadora for cadastrada com sucesso ou já existir.
/// - `Err(String)`: Se ocorrer um erro durante a validação ou no processo de busca/cadastro.
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

/// Função assíncrona para buscar o ID de uma locadora pelo seu CNPJ.
///
/// Esta função verifica se o CNPJ fornecido não está vazio e, em seguida,
/// realiza uma busca no banco de dados para encontrar o ID da locadora correspondente.
///
/// # Parâmetros
/// - `cnpj`: Uma referência para uma string que representa o CNPJ da locadora.
///
/// # Retornos
/// - `Ok(String)`: O ID da locadora se encontrado.
/// - `Err(String)`: Uma mensagem de erro se o CNPJ estiver vazio ou se ocorrer um erro durante a busca.
#[tauri::command]
pub async fn busca_id_locadora(cnpj: &str) -> Result<String, String>{
    if cnpj.trim().is_empty(){
        return Err("Erro: O campo CNPJ está vazio.".to_string());
    }
    let cnpj = formata_cnpj(cnpj);
    let cnpj = match cnpj{
        Ok(_) => {
            cnpj.unwrap()
        },
         Err(e) => {
            return Err(e);
        }
    };
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

/// Função para validar e criar uma instância de `Locadora` a partir de um JSON.
///
/// A função extrai e verifica os campos necessários de um objeto JSON para criar uma instância
/// de `Locadora`. Se algum dos campos obrigatórios estiver vazio, retorna um erro.
///
/// # Parâmetros
/// - `locadora`: Um objeto JSON contendo os dados da locadora.
///
/// # Retornos
/// - `Ok(Locadora)`: Retorna uma instância válida de `Locadora` se todos os campos estiverem preenchidos.
/// - `Err(String)`: Retorna uma mensagem de erro se um ou mais campos obrigatórios estiverem vazios.
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
        idsocio: locadora["idsocio"].as_str().unwrap_or("").to_string(),
        locadorastatus: 1
    };
    if locadora.idendereco.trim().is_empty() || locadora.cnpj.trim().is_empty() || 
        locadora.numerocontabanco.trim().is_empty()
        || locadora.numeroagenciabanco.trim().is_empty() || 
        locadora.nomebanco.trim().is_empty() || locadora.nomelocadora.trim().is_empty() || locadora.idsocio.trim().is_empty(){
            return Err("Erro: Um ou mais campos estão vazios.".to_string());
    }
    return Ok(locadora);
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
