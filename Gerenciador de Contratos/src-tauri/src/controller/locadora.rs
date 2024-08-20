use mysql_async::{params, prelude::Queryable};

use crate::controller;

pub struct Locadora{
    pub idlocadora: String,
    pub idendereco: String,
    pub cnpj: String,
    pub numerocontabanco: String,
    pub numeroagenciabanco: String,
    pub nomebanco: String,
    pub nomelocadora: String
}

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
pub fn estrutura_locadora(idendereco: String, cnpj: String, numerocontabanco: String, numeroagenciabanco: String, nomebanco: String, nomelocadora: String) -> Result<serde_json::Value, bool>{
    let id = controller::enc_senha(&cnpj);
    let locadora = serde_json::json!({
        "idlocadora": id,
        "idendereco": idendereco,
        "cnpj": cnpj,
        "numerocontabanco": numerocontabanco,
        "numeroagenciabanco": numeroagenciabanco,
        "nomebanco": nomebanco,
        "nomelocadora": nomelocadora
    });
    return Ok(locadora)
}

#[tauri::command]
pub async fn cadastra_locadora(locadora: serde_json::Value) -> Result<String, String>{
    let idlocadora = locadora["idlocadora"].as_str().unwrap_or("").to_string();
    let idlocadora = idlocadora.split_at(45 as usize);
    let idlocadora = idlocadora.0.to_string();
    let locadora = Locadora {
        idlocadora: idlocadora,
        idendereco: locadora["idendereco"].as_str().unwrap_or("").to_string(),
        cnpj: locadora["cnpj"].as_str().unwrap_or("").to_string(),
        numerocontabanco: locadora["numerocontabanco"].as_str().unwrap_or("").to_string(),
        numeroagenciabanco: locadora["numeroagenciabanco"].as_str().unwrap_or("").to_string(),
        nomebanco: locadora["nomebanco"].as_str().unwrap_or("").to_string(),
        nomelocadora: locadora["nomelocadora"].as_str().unwrap_or("").to_string(),
    };

    let resultado_busca = _busca_id_locadora(&locadora.cnpj).await;

    match resultado_busca{
        Ok(resultado) => {
            if resultado == ""{
                let _resultado_cadastro = _cadastra_locadora(locadora).await;
                return Ok("Locadora cadastrada com sucesso".to_string());
            }
            return Err("Erro: Locadora já cadastrada".to_string());
        },
        Err(erro) => {
            return Err(erro.to_string());
        }
    }
}

pub async fn _cadastra_locadora(locadora: Locadora) -> Result<String, mysql_async::Error>{
    let pool = controller::cria_pool().await.unwrap();
    let mut conn = pool.get_conn().await?;
    let resultado_insert =
         conn.exec_drop("INSERT INTO locadora (idlocadora, idendereco, cnpj, numerocontabanco, numeroagenciabanco, nomebanco, nomelocadora)
          VALUES (:idlocadora, :idendereco, :cnpj, :numerocontabanco, :numeroagenciabanco, :nomebanco, :nomelocadora)", 
         params! {"idlocadora" =>  locadora.idlocadora, "idendereco" => locadora.idendereco, "cnpj" => locadora.cnpj, "numerocontabanco" => locadora.numerocontabanco,
            "numeroagenciabanco" => locadora.numeroagenciabanco, "nomebanco" => locadora.nomebanco, "nomelocadora" => locadora.nomelocadora}).await;
    match resultado_insert{
        Ok(_) => {
            println!("Locadora cadastrada");
        }, 
        Err(e) => {
            println!("{:?}", e);
            return Err(e);
        }
    }
    return Ok("er".to_string());
}

#[tauri::command]
pub async fn busca_id_locadora() -> Result<String, String>{
    let resultado = _busca_id_locadora("000123").await;
    match resultado{
        Ok(id) =>{
            return Ok(id);
        },
        Err(e) => {
            return Err(e.to_string());
        }
    }
}

// mover para model
pub async fn _busca_id_locadora(cnpj: &str) -> Result<String, mysql_async::Error>{

    let pool = controller::cria_pool().await.unwrap();
    let mut conn = pool.get_conn().await?;
    let resultado_busca: Result<Option<String>, mysql_async::Error> = conn.exec_first("SELECT idlocadora FROM locadora WHERE cnpj = :cnpj",
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