use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::{
    controller::gera_hash,
    model::{self, erro::MeuErro, maquina::Maquina},
};

/// ## Recebe dados referentes a uma Maquina e os converte em um objeto do tipo `serde_json::Value`
/// Primeiro, verifica se os dados recebidos estão vazios, retornando erro caso ao menos um esteja.
/// Em seguida, usa o `numserie` para gerar um hash que se tornará o ID da maquina, 
///  corta esse hash para que tenha no máximo 45 caracteres e o converte para String:
/// ```
/// let idmaquina = gera_hash(&numserie)
///     .split_at(45 as usize)
///     .0.to_string();
/// ```
/// Por fim, junta os dados em um serde_json::Value e retorna o valor final como um Value equivalente ao tipo `Maquina`:
/// ```
/// let maquina: serde_json::Value = serde_json::json!({
///     "nomemaquina": nomemaquina,
///     "idmaquina": idmaquina,
///     "valoraluguel": valoraluguel,
///     "numserie": numserie,
/// });
/// return Ok(maquina);
/// ```

#[derive(Serialize, Deserialize)]
pub struct EstruturaMaquinaInput{
    pub nomemaquina: String,
    pub valoraluguel: String,
    pub numserie: String
}

//#[tauri::command]
pub async fn estrutura_maquina(input: Json<EstruturaMaquinaInput>) -> Result<Json<Maquina>, (StatusCode, Json<String>)> {
    let nomemaquina = input.nomemaquina.to_string();
    let valoraluguel = input.valoraluguel.to_string();
    let numserie = input.numserie.to_string();
    if nomemaquina.is_empty() || valoraluguel.is_empty() || numserie.is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json(MeuErro::CamposVazios.to_string())))
    }
    let idmaquina = gera_hash(&numserie)
        .split_at(45 as usize).0
        .to_string();
    
    let maquina = Maquina{
        nomemaquina,
        idmaquina,
        valoraluguel: formata_valor_f32(&valoraluguel).unwrap(),
        numserie,
        disponibilidade: 1,
        maquinastatus: 1
    };
    return Ok(Json(maquina));
}

/// ## Recebe um serde_json::Value, que é convertido para o tipo `Maquina` e registrado no banco. No final, a função retorna o ID da Maquina.
/// Primeiro, converte o campo `valoraluguel` em uma string slice e então em uma String. Essa String é passada como string slice para a função
/// que faz a conversão do `valoraluguel` para um float(f32):
/// ```
/// let valoraluguel = maquina["valoraluguel"].as_str().unwrap_or("").to_string();
/// let valoraluguel = formata_valor_f32(&valoraluguel)?;
/// ```
/// Em seguida, é criado um objeto do tipo `Maquina`, que recebe os campos equivalentes do Value, convertidos para String quando necessário:
/// ```
///  let maquina: model::maquina::Maquina = model::maquina::Maquina {
///     nomemaquina: maquina["nomemaquina"].as_str().unwrap_or("").to_string(),
///     numserie: maquina["numserie"].as_str().unwrap_or("").to_string(),
///     [...]
/// ```
/// Depois, `maquina` tem os campos `disponibilidade` e `maquinastatus` inicializados em `1`.
/// Após a criação do objeto `maquina`, ele é passado como argumento para chamar a função responsável por cadastrar uma Maquina no banco de dados:
/// ```
/// let resultado_cadastro = model::maquina::cadastrar_maquina(maquina).await;
/// ```
/// Se o registro for bem-sucedido, a função retorna o ID da Maquina:
/// ```
/// Ok(idmaquina) => {
///     return Ok(idmaquina);
/// }
/// ```
//#[tauri::command]
pub async fn cadastra_maquina(input: Json<Maquina>) -> Result<(StatusCode, Json<String>), (StatusCode, Json<String>)>{
    
    let valoraluguel =  input.valoraluguel.to_string();
    let valoraluguel = match formata_valor_f32(&valoraluguel){
        Ok(valor) => {
            valor
        },
        Err(e) => {
            return Err((StatusCode::BAD_REQUEST, Json(e.to_string())))
        }
    };

    let maquina: model::maquina::Maquina = model::maquina::Maquina {
        nomemaquina: input.nomemaquina.to_string(),
        numserie: input.numserie.to_string(),
        valoraluguel,
        idmaquina: input.idmaquina.to_string(),
        disponibilidade: 1,
        maquinastatus: 1
    };

    let resultado_cadastro = model::maquina::cadastrar_maquina(maquina).await;
    match resultado_cadastro{
        Ok(idmaquina) => {
            return Ok((StatusCode::CREATED, Json(idmaquina)));
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())))
        }
    }
}

/// ## Recebe um nome que é usado para buscar uma Maquina no banco de dados, retornando um vetor com as máquinas que possuam nomes semelhantes
/// Primeiro, faz a cópia de `nome_maquina` e armazena em `nome_maquina_backup`. Em seguida, 
/// troca os espaços de `nome_maquina` por um caractere vazio e verifica se a String está vazia, retornando erro caso esteja:
/// ```
/// let nome_maquina_backup = nome_maquina.clone();
/// let nome_maquina = nome_maquina.replace(" ", "");
/// if nome_maquina.is_empty(){
///    return Err("Erro: O nome da máquina está vazio.".to_string());
/// }
/// ```
/// Após a verificação, chama a função que busca as máquinas com nomes semelhantes ao `nome_maquina_backup` no banco de dados:
/// ```
/// let resultado_busca: Result<Vec<model::maquina::Maquina>, mysql_async::Error> = model::maquina::buscar_maquina_nome(&nome_maquina_backup).await;
/// ```
/// Se a busca não falhar, verifica se o vetor resultante está vazio, retornando erro caso esteja,
///  ou um vetor de Maquina caso possua ao menos um registro:
/// ``` 
/// Ok(resultado) => {
///     if !resultado.is_empty(){
///         return Ok(resultado);
///     }
///    return Err(MeuErro::MaquinaNaoEncontrada.to_string());
/// }
/// ```
//#[tauri::command]
pub async fn busca_maquina_nome(nome_maquina: Json<String>) 
    -> Result<(StatusCode, Json<Vec<model::maquina::Maquina>>), (StatusCode, Json<String>)>{
    let nome_maquina_backup = nome_maquina.clone();
    let nome_maquina = nome_maquina.replace(" ", "");
    if nome_maquina.is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json(MeuErro::NomeMaquinaVazio.to_string())));
    }
    
    let resultado_busca: Result<Vec<model::maquina::Maquina>, mysql_async::Error> = model::maquina::buscar_maquina_nome(&nome_maquina_backup).await;

    match resultado_busca{
        Ok(resultado) => {
            if !resultado.is_empty(){
                return Ok((StatusCode::OK, Json(resultado)));
            }
            return Err((StatusCode::BAD_REQUEST, Json(MeuErro::MaquinaNaoEncontrada.to_string())));
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())));
        }
    }
}

/// ## Recebe um número de série e busca no banco de dados pela máquina ao qual ele pertence, retornando um vetor com um único registro
// Primeiro, faz a cópia de `numserie` e armazena em `numserie_backup`. Em seguida, 
/// troca os espaços de `numserie` por um caractere vazio e verifica se a String está vazia, retornando erro caso esteja:
/// ```
/// let numserie_backup = numserie.clone();
/// let numserie = numserie.replace(" ", "");
/// if numserie.is_empty(){
///        return Err("Erro: O número de série está vazio.".to_string());
/// }
/// ```
/// Depois, busca no banco de dados por uma máquina que tenha esse número de série:
/// ```
/// let resultado_busca: Result<Vec<model::maquina::Maquina>, mysql_async::Error> = model::maquina::busca_maquina_serie(&numserie_backup).await;
/// ```
/// Se o vetor resultante da busca não estiver vazio, ele é retornado. Caso contrário, a função retorna um erro:
/// ```
/// Ok(resultado) => {
///     if !resultado.is_empty(){
///     return Ok(resultado);
///     }
///     return Err(MeuErro::MaquinaNaoEncontrada.to_string());
/// }
/// ```
//#[tauri::command]
pub async fn busca_maquina_numserie(numserie: Json<String>) 
    -> Result<(StatusCode, Json<Vec<model::maquina::Maquina>>), (StatusCode, Json<String>)>{
    let numserie_backup = numserie.clone();
    let numserie = numserie.replace(" ", "");
    if numserie.is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("O número de série está vazio.".to_string())));
    }
    let resultado_busca: Result<Vec<model::maquina::Maquina>, mysql_async::Error> = model::maquina::busca_maquina_serie(&numserie_backup).await;

    match resultado_busca{
        Ok(resultado) => {
            if !resultado.is_empty(){
                return Ok((StatusCode::OK, Json(resultado)));
            }
            return Err((StatusCode::BAD_REQUEST, Json(MeuErro::MaquinaNaoEncontrada.to_string())));
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())));
        }
    }
}

/// ## Recebe o nome de uma máquina e retorna um vetor do tipo `EstoqueMaquina` após buscar no banco pela quantidade de máquinas disponíveis e que tenham nomes semelhantes.
/// Primeiro, faz a cópia de `nomemaquina` e armazena em `nomemaquina_backup`. Em seguida, 
/// troca os espaços de `nomemaquina` por um caractere vazio e verifica se a String está vazia, retornando erro caso esteja:
/// ```
/// let nomemaquina_backup = nomemaquina.clone();
/// let nomemaquina = nomemaquina.replace(" ", "");
/// if nomemaquina.is_empty(){
///    return Err("Erro: O nome da máquina está vazio.".to_string());
/// }
/// ```
/// Em seguida, chama a função que faz a busca no banco de dados, e o resultado da busca é armazenado em `estoque_maquina`. 
/// Se esse resultado não for um erro e `estoque_maquina` não estiver vazio, é retornado um vetor do tipo `EstoqueMaquina`:
/// ```
/// let estoque_maquina = match model::maquina::gera_estoque_por_nome(nomemaquina_backup).await{
///     Ok(maquina) => {maquina},
///     Err(e) => {return Err(e.to_string())}
/// };
/// if estoque_maquina.is_empty(){
///     return Err(MeuErro::MaquinaNaoEncontrada.to_string())
/// }
/// return Ok(estoque_maquina)
/// ```
//#[tauri::command]
pub async fn gera_estoque_por_nome(nomemaquina: Json<String>)
    -> Result<(StatusCode, Json<Vec<model::maquina::EstoqueMaquina>>), (StatusCode, Json<String>)>{
    let nomemaquina_backup = nomemaquina.clone();
    let nomemaquina = nomemaquina.replace(" ", "");
    if nomemaquina.is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("O nome da máquina está vazio.".to_string())));
    }

    let estoque_maquina = match model::maquina::gera_estoque_por_nome(nomemaquina_backup.0).await{
        Ok(maquina) => {maquina},
        Err(e) => {return Err((StatusCode::BAD_REQUEST, Json(e.to_string())))}
    };
    if estoque_maquina.is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json(MeuErro::MaquinaNaoEncontrada.to_string())))
    }
    return Ok((StatusCode::OK, Json(estoque_maquina)))
}

/// ## Gera o estoque total de todas as máquinas disponíveis atualmente no banco de dados, retornando um vetor do tipo `EstoqueMaquina`
//#[tauri::command]
pub async fn gera_estoque_total() 
    -> Result<(StatusCode, Json<Vec<model::maquina::EstoqueMaquina>>), (StatusCode, Json<String>)>{
    match model::maquina::gera_estoque_total().await{
        Ok(estoque_total) => {return Ok((StatusCode::OK, Json(estoque_total)))},
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())))
    };
}

/// ## Gera o estoque total de todas as máquinas alugadas atualmente no banco de dados, retornando um vetor do tipo `EstoqueMaquina`
//#[tauri::command]
pub async fn gera_estoque_total_alugadas() 
    -> Result<(StatusCode, Json<Vec<model::maquina::EstoqueMaquina>>), (StatusCode, Json<String>)>{
    match model::maquina::gera_estoque_total_alugadas().await{
        Ok(estoque_total) => {return Ok((StatusCode::OK, Json(estoque_total)))},
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())))
    };
}

/// ## Recebe uma string slice, converte para float(f32) e retorna o resultado da conversão
/// Primeiro, remove espaços e os caracteres 'R$' da string slice, armazenando o resultado em `valor`, que agora é do tipo String:
/// ```
/// let valor:String = valor.trim().split("R$").collect();
/// ```
/// Em seguida, remove todos os caracteres '.' da String e troca as vírgulas pelo caractere '.' novamente, 
/// convertendo o resultado em um float(f32):
/// ```
/// let valor: f32 = match valor.replace(".", "").replace(",", ".").parse(){
/// ```
pub fn formata_valor_f32(valor: &str) -> Result<f32, String>{
    let valor:String = valor.trim().split("R$").collect();
    let valor: f32 = match valor.trim().replace(".", "").replace(",", ".").parse(){
        Ok(valor) => {valor},
        Err(e) => {return Err(e.to_string())}
    };
    return Ok(valor);
}