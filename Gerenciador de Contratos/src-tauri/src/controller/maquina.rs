use mysql_async::{params, prelude::Queryable};

use crate::{
    controller::{self, gera_hash},
    model,
};

#[tauri::command]
pub async fn estrutura_maquina(nomemaquina: String, valoraluguel: String, numserie: String) -> Result<serde_json::Value, String> {
    if nomemaquina.is_empty() || valoraluguel.is_empty() || numserie.is_empty(){
        return Err("Erro: Um ou mais campos estão vazios.".to_string())
    }
    let idmaquina = gera_hash(&numserie);
    let valoraluguel:f32 = valoraluguel.trim().parse().unwrap();
    let maquina: serde_json::Value = serde_json::json!({
        "idmaquina": idmaquina,
        "valoraluguel": valoraluguel,
        "numserie": numserie,
    });
    return Ok(maquina);
}

#[tauri::command]
pub async fn filtra_maquina_nome(nome_maquina: String) -> Result<Vec<model::maquina::Maquina>, String>{
    let resultado_busca: Result<Vec<model::maquina::Maquina>, mysql_async::Error> = _filtra_maquina_nome(nome_maquina).await;

    match resultado_busca{
        Ok(resultado) => {
            if !resultado.is_empty(){
                return Ok(resultado);
            }
            return Err("Erro: Máquina não encontrada".to_string());
        },
        Err(erro) => {
            return Err(erro.to_string());
        }
    }
}

pub async fn _filtra_maquina_nome(nome_maquina: String) -> Result<Vec<model::maquina::Maquina>, mysql_async::Error>{
    let pool = match controller::cria_pool().await {
        Ok(pool) => {
            pool
        }, 
        Err(e) =>{
            return Err(e)
        }
    };
    let mut conn = pool.get_conn().await?;
    let resultado_busca: Result<Vec<model::maquina::Maquina>, mysql_async::Error> = conn.exec_map(
        "SELECT idmaquina, nomemaquina, numserie, valoraluguel FROM maquina WHERE nomemaquina = :nome_maquina ORDER BY valoraluguel ".to_owned() + "DESC",
        params! { "nome_maquina" => nome_maquina },
        |(idmaquina, nomemaquina, numserie, valoraluguel)| model::maquina::Maquina {
            idmaquina,
            nomemaquina,
            numserie,
            valoraluguel,
        }
    ).await;
    
    match resultado_busca{
        Ok(valor_mensal) => {            
            if valor_mensal.is_empty(){
                return Ok(vec![]);
            }
            return Ok(valor_mensal)
        },
        Err(e) => {
            return Err(e);
        }
    }
}