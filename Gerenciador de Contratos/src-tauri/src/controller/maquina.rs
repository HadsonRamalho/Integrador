use crate::{
    controller::gera_hash,
    model::{self, erro::MeuErro},
};

#[tauri::command]
pub async fn estrutura_maquina(nomemaquina: String, valoraluguel: String, numserie: String) -> Result<serde_json::Value, String> {
    if nomemaquina.is_empty() || valoraluguel.is_empty() || numserie.is_empty(){
        return Err(MeuErro::CamposVazios.to_string())
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
    let valoraluguel = formata_valor_f32(&valoraluguel)?;

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
            return Err(MeuErro::MaquinaNaoEncontrada.to_string());
        },
        Err(erro) => {
            return Err(erro.to_string());
        }
    }
}



#[tauri::command]
pub async fn busca_maquina_numserie(numserie: String) -> Result<Vec<model::maquina::Maquina>, String>{
    let numserie_backup = numserie.clone();
    let numserie = numserie.replace(" ", "");
    if numserie.is_empty(){
        return Err("Erro: O número de série está vazio.".to_string());
    }
    let numserie = numserie_backup;
    let resultado_busca: Result<Vec<model::maquina::Maquina>, mysql_async::Error> = model::maquina::busca_maquina_serie(&numserie).await;

    match resultado_busca{
        Ok(resultado) => {
            if !resultado.is_empty(){
                return Ok(resultado);
            }
            return Err(MeuErro::MaquinaNaoEncontrada.to_string());
        },
        Err(erro) => {
            return Err(erro.to_string());
        }
    }
}

#[tauri::command]
pub async fn gera_estoque_por_nome(nomemaquina: &str) -> Result<Vec<model::maquina::EstoqueMaquina>, String>{
    let estoque_maquina = match model::maquina::gera_estoque_por_nome(nomemaquina.to_string()).await{
        Ok(maquina) => {maquina},
        Err(e) => {return Err(e.to_string())}
    };
    if estoque_maquina.is_empty(){
        return Err(MeuErro::MaquinaNaoEncontrada.to_string())
    }
    return Ok(estoque_maquina)
}

#[tauri::command]
pub async fn gera_estoque_total() -> Result<Vec<model::maquina::EstoqueMaquina>, String>{
    match model::maquina::gera_estoque_total().await{
        Ok(estoque_total) => {return Ok(estoque_total)},
        Err(e) => return Err(e.to_string())
    };
}

#[tauri::command]
pub async fn gera_estoque_total_alugadas() -> Result<Vec<model::maquina::EstoqueMaquina>, String>{
    match model::maquina::gera_estoque_total_alugadas().await{
        Ok(estoque_total) => {return Ok(estoque_total)},
        Err(e) => return Err(e.to_string())
    };
}

pub fn formata_valor_f32(valor: &str) -> Result<f32, String>{
    let valor:String = valor.trim().split("R$").collect();
    let valor: f32 = match valor.trim().replace(".", "").replace(",", ".").parse(){
        Ok(valor) => {valor},
        Err(e) => {return Err(e.to_string())}
    };
    return Ok(valor);
}