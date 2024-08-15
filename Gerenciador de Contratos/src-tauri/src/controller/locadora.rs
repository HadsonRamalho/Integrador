use mysql_async::prelude::Queryable;

use crate::controller;

#[tauri::command]
pub fn estrutura_locadora(idendereco: String, cnpj: String, numerocontabanco: String, numeroagenciabanco: String, nomebanco: String) -> Result<serde_json::Value, bool>{
    let id = controller::enc_senha("cnpj");
    let locadora = serde_json::json!({
        "idendereco": idendereco,
        "idlocadora": id,
        "numerocontabanco": numerocontabanco,
        "numeroagenciabanco": numeroagenciabanco,
        "nomebanco": nomebanco,
    });
    return Ok(locadora)
}

pub async fn busca_id_locadora(cnpj: String) -> Result<String, mysql_async::Error>{
    let pool = controller::cria_pool().await.unwrap();
    let conn = pool.get_conn().await?;
  //  let mut resultado_busca = conn.exec_drop(stmt, params);
    return Ok("x".to_string())
}