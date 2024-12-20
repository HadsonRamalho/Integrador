use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::{controller, model::{self, erro::MeuErro, locatario::Locatario}};

use super::locadora::formata_cnpj;

#[derive(Serialize, Deserialize)]
pub struct LocatarioInput{
    pub idendereco: String,
    pub cnpj: String,
    pub nomelocatario: String,
    pub idsocio: String,
    pub locatariostatus: i16
}


/// ## Recebe campos separados e os junta em um serde_json::Value que representa um Locatario
/// Primeiro, verifica se algum dos campos recebidos está vazio e retorna erro caso ao menos um esteja: 
/// ```
/// if idendereco.trim().is_empty() || cnpj.trim().is_empty() || 
/// nomelocatario.trim().is_empty() || idsocio.trim().is_empty(){
///    return Err(MeuErro::CamposVazios.to_string())
/// }
/// ```
/// Em seguida, faz a formatação do CNPJ que foi recebido como parâmetro:
/// ```
/// let cnpj = match controller::locadora::formata_cnpj(&cnpj)
/// ```
/// Caso a formatação seja bem-sucedida, usa o CNPJ formatado para gerar o hash do ID do Locatario:
/// ```
/// let id: String = controller::gera_hash(&cnpj);
/// ```
/// Após gerar o hash, junta todos os dados em um serde_json::Value e retorna o objeto:
/// ```
/// let locatario: serde_json::Value = serde_json::json!({
///     "idlocatario": id,
///     "idendereco": idendereco,
///     "cnpj": cnpj,
///     "nomelocatario": nomelocatario,
///     "idsocio": idsocio
/// });
/// return Ok(locatario)
/// ```
//#[tauri::command]
pub async fn estrutura_locatario(input: Json<LocatarioInput>) 
    -> Result<(StatusCode, Json<Locatario>), (StatusCode, Json<String>)>{
    let idendereco = input.idendereco.clone();
    let cnpj = input.cnpj.clone();
    let nomelocatario = input.nomelocatario.clone();
    let idsocio = input.idsocio.clone();
    
    if idendereco.trim().is_empty() || cnpj.trim().is_empty() || 
        nomelocatario.trim().is_empty() || idsocio.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json(MeuErro::CamposVazios.to_string())))
    }
    let cnpj = match controller::locadora::formata_cnpj(&cnpj){
        Ok(cnpj) => {
            cnpj
        },
        Err(e) => {
            return Err((StatusCode::BAD_REQUEST, Json(e)));
        }
    };
    let id: String = controller::gera_hash(&cnpj);

    let locatario = Locatario{
        idendereco,
        idlocatario: id,
        cnpj,
        nomelocatario,
        idsocio,
        locatariostatus: 1
    };
    return Ok((StatusCode::OK, Json(locatario)))
}

/// ## Recebe um serde_json::Value contendo dados de um Locatario e registra esses campos no banco após a conversão para o tipo `Locatario`
/// Primeiro, transforma o idlocatario do Value em uma String, corta essa String no índice 45 e faz mais uma cópia do resultado:
/// ```
/// let idlocatario: String = locatario["idlocatario"].as_str().unwrap_or("").to_string();
/// let idlocatario: (&str, &str) = idlocatario.split_at(45 as usize);
/// let idlocatario: String = idlocatario.0.to_string();
/// let idlocatario_cpy = idlocatario.clone();
/// ```
/// Em seguida, converte os demais campos em String e usa eles para criar um objeto do tipo `Locatario`:
/// ```
/// let idsocio: String = locatario["idsocio"].as_str().unwrap_or("").to_string();
/// let locatario: model::locatario::Locatario = model::locatario::Locatario {
///     idlocatario,
///     idendereco: locatario["idendereco"].as_str().unwrap_or("").to_string(),
///     [...]
/// };
/// ```
/// É verificado se algum dos campos está vazio, e é retornado um erro caso ao menos um esteja:
/// ```
/// if locatario.idlocatario.is_empty() || locatario.idsocio.is_empty() || 
///     locatario.cnpj.is_empty() || locatario.nomelocatario.is_empty() || 
///     locatario.idendereco.is_empty(){
/// return Err(MeuErro::CamposVazios.to_string())
/// }
/// ```
/// Faz a busca do ID do Locatario no banco de dados, e retorna erro caso a busca falhe ou caso o ID não esteja vazio (o que significa que o Locatario já está cadastrado).
/// ```
/// let resultado_busca: Result<String, mysql_async::Error> = model::locatario::busca_id_locatario(&locatario.cnpj).await;
/// ```
/// Caso a função não tenha retornado um erro, o `locatario` é cadastrado no banco de dados a partir da função responsável por isso:
/// ```
/// let resultado_cadastro = model::locatario::_cadastra_locatario(locatario).await;
/// ```
//#[tauri::command]
pub async fn cadastra_locatario(input: Json<Locatario>) 
    -> Result<(StatusCode, Json<String>), (StatusCode, Json<String>)>{
    let idlocatario: String = input.idlocatario.to_string();
    let idlocatario: (&str, &str) = idlocatario.split_at(45 as usize);
    let idlocatario: String = idlocatario.0.to_string();
    let idlocatario_cpy = idlocatario.clone();
    let idsocio: String = input.idsocio.to_string();
    let idendereco = input.idendereco.to_string();
    let cnpj = input.cnpj.to_string();
    let nomelocatario = input.nomelocatario.to_string();

    let locatario: model::locatario::Locatario = model::locatario::Locatario {
        idlocatario,
        idendereco,
        cnpj,
        nomelocatario,
        idsocio,
        locatariostatus: 1
    };

    if locatario.idlocatario.is_empty() || locatario.idsocio.is_empty() || 
        locatario.cnpj.is_empty() || locatario.nomelocatario.is_empty() || 
        locatario.idendereco.is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json(MeuErro::CamposVazios.to_string())))
    }

    let resultado_busca: Result<String, mysql_async::Error> = model::locatario::busca_id_locatario(&locatario.cnpj).await;

    let id = match resultado_busca{
        Ok(resultado) => {
            resultado
        },
        Err(e) => {
            return Err((StatusCode::BAD_REQUEST, Json(e.to_string())));
        }
    };
    if !id.is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("Locatario já cadastrado".to_string())));        
    }
    let resultado_cadastro = model::locatario::_cadastra_locatario(locatario).await;
    match resultado_cadastro{
        Ok(_) => {
            return Ok((StatusCode::BAD_REQUEST, Json(idlocatario_cpy.to_string())));
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())))
        }
    }
}

/// ## Recebe um CNPJ e busca pelo ID do Locatario no banco de dados
/// Primeiro, verifica se o CNPJ está vazio e retorna erro caso esteja.
///  Se não retornar um erro, faz a formatação do CNPJ e usa o resultado para chamar a função que faz a busca do ID no banco de dados:
/// ```
/// let resultado: Result<String, mysql_async::Error> = model::locatario::busca_id_locatario(cnpj).await;
/// ```
//#[tauri::command]
pub async fn busca_id_locatario(input: Json<String>) 
    -> Result<(StatusCode, Json<String>), (StatusCode, Json<String>)>{
    let cnpj = input.0;
    if cnpj.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json(MeuErro::CnpjVazio.to_string())))
    }
    let cnpj = match formata_cnpj(&cnpj){
        Ok(cnpj) => {
            cnpj
        },
        Err(e) => {
            return Err((StatusCode::BAD_REQUEST, Json(e)))
        }
    };

    let resultado: Result<String, mysql_async::Error> = model::locatario::busca_id_locatario(&cnpj).await;
    match resultado{
        Ok(id) =>{
            return Ok((StatusCode::OK, Json(id)));
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())));
        }
    }
}

/// ## Recebe o nome de um Locatario e retorna um vetor contendo todos os registros que contenham um nome parecido com o argumento recebido
/// Primeiro, verifica se o nome está vazio e retorna erro caso esteja.
/// Se a função não retornar erro, chama a função que faz a busca no banco de dados, armazenando o resultado dessa busca em um vetor de `Locatario`:
/// ```
/// let resultado_busca: Result<Vec<Locatario>, mysql_async::Error> = model::locatario::busca_locatario_nome(&nomelocatario).await;
/// ```
/// Por fim, verifica se o vetor recebido está vazio e retorna um erro caso esteja. Se o vetor possuir ao menos um registro, o vetor de objetos é retornado:
/// ```
/// if locatario.is_empty(){
///     return Err("Locatário não encontrado".to_string());
/// }
/// return Ok(locatario)
/// ```
//#[tauri::command]
pub async fn busca_locatario_nome(input: Json<String>) 
    -> Result<(StatusCode, Json<Vec<Locatario>>), (StatusCode, Json<String>)>{
    let nomelocatario = input.0;
    if nomelocatario.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json(MeuErro::NomeLocatarioVazio.to_string())))
    }
    let resultado_busca: Result<Vec<Locatario>, mysql_async::Error> = model::locatario::busca_locatario_nome(&nomelocatario).await;
    match resultado_busca {
        Ok(locatario) =>{
            if locatario.is_empty(){
                return Err((StatusCode::BAD_REQUEST, Json(MeuErro::NomeLocatarioNaoEncontrado.to_string())));
            }
            return Ok((StatusCode::OK, Json(locatario)))
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())));
        }
    }
}

/// ## Recebe um CNPJ e usa ele para buscar um Locatario no banco de dados
/// Primeiro, verifica se o parâmetro CNPJ está vazio, retornando um erro caso esteja.
/// Se o CNPJ não estiver vazio, ele é formatado e o valor resultante é armazenado em `cnpj`. 
/// `cnpj` é então utilizado para chamar a função que faz a busca do Locatario no banco de dados:
/// ```
/// let resultado_busca = model::locatario::busca_locatario_cnpj(&cnpj).await;
/// ```
/// Se a busca não falhar, um vetor é retornado e armazenado em `resultado_busca`. A função então retorna o vetor de `Locatario`:
/// ```
/// Ok(locatario) =>{
///     return Ok(locatario);
/// }
/// ```
//#[tauri::command]
pub async fn busca_locatario_cnpj(input: Json<String>) 
    -> Result<(StatusCode, Json<Vec<Locatario>>), (StatusCode, Json<String>)>{
    let cnpj = input.0;
    if cnpj.trim().is_empty(){
        println!("Erro na formatação do CNPJ do Locatario.");
        return Err((StatusCode::BAD_REQUEST, Json(MeuErro::CnpjVazio.to_string())))
    }
    let cnpj = match formata_cnpj(&cnpj){
        Ok(cnpj) => {cnpj},
        Err(e) => {
            println!("{:?}", e);
            return Err((StatusCode::BAD_REQUEST, Json(e)));
        }
    };
    let resultado_busca = model::locatario::busca_locatario_cnpj(&cnpj).await;
    match resultado_busca {
        Ok(locatario) =>{
            return Ok((StatusCode::OK, Json(locatario)));
        },
        Err(e) => {
            println!("{:?}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())));
        }
    }
}
