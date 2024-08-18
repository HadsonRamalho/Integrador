use std::ops::Deref;

use mysql_async::{params, prelude::Queryable};

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
pub fn estrutura_locadora(idendereco: String, cnpj: String, numerocontabanco: String, numeroagenciabanco: String, nomebanco: String) -> Result<serde_json::Value, bool>{
    let id = controller::enc_senha(&cnpj);
    let locadora = serde_json::json!({
        "idendereco": idendereco,
        "idlocadora": id,
        "cnpj": cnpj,
        "numerocontabanco": numerocontabanco,
        "numeroagenciabanco": numeroagenciabanco,
        "nomebanco": nomebanco,
    });
    return Ok(locadora)
}

#[tauri::command]
pub async fn busca_id_locadora() -> Result<String, bool>{
    let resultado = _busca_id_locadora("123456").await;
    match resultado{
        Ok(id) =>{
            return Ok(id);
        },
        Err(e) => {
            return Err(false); //Alterar para myqsl_async::Error
        }
    }
}

// mover para model
pub async fn _busca_id_locadora(cnpj: &str) -> Result<String, mysql_async::Error>{
    let pool = controller::cria_pool().await.unwrap();
    let mut conn = pool.get_conn().await?;
    let mut resultado_busca: Result<Option<String>, mysql_async::Error> = conn.exec_first("SELECT idlocadora FROM locadora WHERE cnpj = :cnpj",
     params!{"cnpj" => cnpj}).await;
    match resultado_busca{
        Ok(id) => {
            return Ok(id.unwrap());
        },
        Err(e) => {
            return Err(e);
        }
    }
}