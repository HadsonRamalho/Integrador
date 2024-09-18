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
    let idmaquina = gera_hash(&numserie).split_at(45 as usize).0.to_string();
    let maquina: serde_json::Value = serde_json::json!({
        "nomemaquina": nomemaquina,
        "idmaquina": idmaquina,
        "valoraluguel": valoraluguel,
        "numserie": numserie,
    });
    return Ok(maquina);
}

#[tauri::command]
pub async fn cadastra_maquina(maquina: serde_json::Value) -> Result<String, String>{
    
    let valoraluguel = maquina["valoraluguel"].as_str().unwrap_or("").to_string();
    let valoraluguel: f32 = match valoraluguel.trim().parse(){
        Ok(valoraluguel) => {
            valoraluguel
        },
        Err(e) => {
            return Err(e.to_string())
        }
    };

    let maquina: model::maquina::Maquina = model::maquina::Maquina {
        nomemaquina: maquina["nomemaquina"].as_str().unwrap_or("").to_string(),
        numserie: maquina["numserie"].as_str().unwrap_or("").to_string(),
        valoraluguel,
        idmaquina: maquina["idmaquina"].as_str().unwrap_or("").to_string(),
        disponibilidade: 1,
        maquinastatus: 1
    };

    let resultado_cadastro = model::maquina::cadastrar_maquina(maquina).await;
    match resultado_cadastro{
        Ok(idmaquina) => {
            return Ok(idmaquina);
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn busca_maquina_nome(nome_maquina: String) -> Result<Vec<model::maquina::Maquina>, String>{
    let nome_maquina_backup = nome_maquina.clone();
    let nome_maquina = nome_maquina.replace(" ", "");
    if nome_maquina.is_empty(){
        return Err("Erro: O nome da máquina está vazio.".to_string());
    }
    let nome_maquina = nome_maquina_backup;
    let resultado_busca: Result<Vec<model::maquina::Maquina>, mysql_async::Error> = model::maquina::buscar_maquina_nome(&nome_maquina).await;

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

pub async fn _busca_maquina_nome(nome_maquina: String) -> Result<Vec<model::maquina::Maquina>, mysql_async::Error>{
    let pool = controller::cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let nome_like = format!("%{}%", nome_maquina);
    let resultado_busca: Result<Vec<model::maquina::Maquina>, mysql_async::Error> = conn.exec_map(
        "SELECT idmaquina, nomemaquina, numserie, valoraluguel, maquinastatus, disponibilidade FROM maquina WHERE nomemaquina LIKE :nome_maquina ORDER BY valoraluguel ".to_owned() + "DESC",
        params! { "nome_maquina" => nome_like },
        |(idmaquina, nomemaquina, numserie, valoraluguel, maquinastatus, disponibilidade)| model::maquina::Maquina {
            idmaquina,
            nomemaquina,
            numserie,
            valoraluguel,
            maquinastatus,
            disponibilidade
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

#[tauri::command]
pub async fn busca_maquina_numserie(numserie: String) -> Result<Vec<model::maquina::Maquina>, String>{
    let numserie_backup = numserie.clone();
    let numserie = numserie.replace(" ", "");
    if numserie.is_empty(){
        return Err("Erro: O nome da máquina está vazio.".to_string());
    }
    let numserie = numserie_backup;
    let resultado_busca: Result<Vec<model::maquina::Maquina>, mysql_async::Error> = model::maquina::busca_maquina_serie(&numserie).await;

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

#[tauri::command]
pub async fn gera_estoque_por_nome(nomemaquina: &str) -> Result<model::maquina::EstoqueMaquina, String>{
    let estoque_maquina = match model::maquina::gera_estoque_por_nome(nomemaquina.to_string()).await{
        Ok(maquina) => {maquina},
        Err(e) => {return Err(e.to_string())}
    };
    return Ok(estoque_maquina)
}