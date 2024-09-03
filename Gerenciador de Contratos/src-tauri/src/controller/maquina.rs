use mysql_async::{params, prelude::Queryable};

use crate::{
    controller::{self, gera_hash},
    model,
};

#[tauri::command]
pub async fn estrutura_maquina(nomemaquina: String, valoraluguel: String, numserie: String) {
    let idmaquina = gera_hash(&numserie);
    let valoraluguel = valoraluguel.trim().parse().unwrap();
    let maquina = model::maquina::Maquina {
        idmaquina,
        nomemaquina,
        valoraluguel,
        numserie,
    };
}

#[tauri::command]
pub async fn busca_nome_maquina(nome_maquina: String) -> Result<String, String>{
    let resultado_busca: Result<String, mysql_async::Error> = _busca_nome_maquina(nome_maquina).await;

    match resultado_busca{
        Ok(resultado) => {
            if resultado != ""{
                return Ok(resultado);
            }
            return Err("Erro: Máquina não encontrada".to_string());
        },
        Err(erro) => {
            return Err(erro.to_string());
        }
    }
}

// Recebe nome da maquina, busca o valor mensal do aluguel e retorna o primeiro registro encontrado
pub async fn _busca_nome_maquina(nome_maquina: String) -> Result<String, mysql_async::Error>{
    let pool = controller::cria_pool().await.unwrap();
    let mut conn = pool.get_conn().await?;
    let resultado_busca: Result<Option<f32>, mysql_async::Error> = conn.exec_first(
        "SELECT contrato_aluguel.valormensal FROM contrato_aluguel 
                INNER JOIN maquina 
                    ON contrato_aluguel.idaluguelmaquina = maquina.idmaquina
                WHERE maquina.nomemaquina = :nome_maquina",
        params! { "nome_maquina" => nome_maquina }
    ).await;    
    match resultado_busca{
        Ok(valor_mensal) => {
            match valor_mensal {
                Some(valor_mensal) => {
                    let valor_mensal = valor_mensal.to_string();
                    return Ok(format!("Valor mensal do aluguel: R${}", valor_mensal));
                }, None =>{
                    return Ok("Máquina não encontrada".to_string());
                }
            }
        },
        Err(e) => {
            return Err(e);
        }
    }
}