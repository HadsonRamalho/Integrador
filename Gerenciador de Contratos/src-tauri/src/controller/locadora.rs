use axum::Json;

use crate::model::erro::MeuErro;
use crate::model::locadora::Locadora;
use crate::model::locadora::_cadastra_locadora;
use crate::model;
use crate::controller;

/// ## Recebe campos separados e transforma em um serde_json::Value contendo dados de uma Locadora
/// Primeiro, verifica se algum dos parâmetros está vazio, retornando erro caso ao menos um esteja.
/// Chama a função responsável por formatar o CNPJ que foi recebido como parâmetro e armazena o valor em `cnpjalterado`:
/// ```
/// let cnpjalterado = formata_cnpj(&cnpj)?;
/// ```
/// Em seguida, usa `cnpjalterado` para gerar o ID da locadora:
/// ```
/// let id: String = controller::gera_hash(&cnpjalterado);
/// ```
/// Usa os campos recebidos para montar e retornar um objeto serde_json::Value que contém dados da Locadora:
/// ```
/// let locadora: serde_json::Value = serde_json::json!({
///     "idlocadora": id,
///     "idendereco": idendereco,
///     "cnpj": cnpjalterado,
///     "numerocontabanco": numerocontabanco,
///     "numeroagenciabanco": numeroagenciabanco,
///     "nomebanco": nomebanco,
///     "nomelocadora": nomelocadora,
///     "idsocio": idsocio
/// });
/// ```

pub struct LocadoraInput{
    pub idendereco: String,
    pub cnpj: String,
    pub numerocontabanco: String,
    pub numeroagenciabanco: String,
    pub nomebanco: String,
    pub nomelocadora: String,
    pub idsocio: String
}

#[tauri::command]
pub fn estrutura_locadora(input: Json<LocadoraInput>) -> Result<Json<Locadora>, String>{
    let idendereco = input.idendereco.to_string();
    let cnpj = input.cnpj.to_string();
    let numerocontabanco = input.numerocontabanco.to_string();
    let numeroagenciabanco = input.numeroagenciabanco.to_string();
    let nomebanco = input.nomebanco.to_string();
    let nomelocadora = input.nomelocadora.to_string();
    let idsocio = input.idsocio.to_string();

    if idendereco.trim().is_empty() || cnpj.trim().is_empty() || numerocontabanco.trim().is_empty()
        || numeroagenciabanco.trim().is_empty() || nomebanco.trim().is_empty() || nomelocadora.trim().is_empty(){
        return Err("Erro: Um ou mais campos estão vazios.".to_string());
    }

    let cnpjalterado = formata_cnpj(&cnpj)?;
    let idlocadora: String = controller::gera_hash(&cnpjalterado);

    let locadora = Locadora{
        idendereco,
        cnpj,
        idlocadora,
        numeroagenciabanco,
        nomebanco,
        nomelocadora,
        numerocontabanco,
        idsocio,
        locadorastatus: 1
    };

    return Ok(Json(locadora));
}

/// ## Recebe um serde_json::Value contendo dados de uma Locadora e retorna o ID após o cadastro dela no banco de dados
/// Primeiro, chama a função que faz a conversão do Value para Locadora:
/// ```
/// let locadora: Result<Locadora, String> = valida_locadora(locadora);
/// match locadora{
///    Ok(_) => {},
///    Err(e) => {return Err(e)}
/// }
/// ```
/// Faz uma cópia do ID da Locadora que foi criada:
/// ```
/// let idlocadora = locadora.idlocadora.clone();
/// ```
/// Busca no banco pelo ID da Locadora usando o CNPJ:
/// ```
/// let resultado_busca: Result<String, mysql_async::Error> =
///     model::locadora::_busca_id_locadora(&locadora.cnpj).await;
/// ```
/// Se essa busca não der erro e retornar um resultado vazio, chama a função que faz o cadastro da Locadora no banco:
/// ```
/// let _resultado_cadastro = _cadastra_locadora(locadora).await;
/// ```
/// Finalmente, retorna a cópia do ID da Locadora:
/// ```
/// return Ok(idlocadora);
/// ```

#[tauri::command]
pub async fn cadastra_locadora(input: Json<Locadora>) -> Result<String, String> {
    let locadora = match valida_locadora(input){
        Ok(locadora) => {
            locadora
        },
        Err(e) => {
            return Err(e)
        }
    };
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

/// ## Recebe um CNPJ, que é formatado e usado para buscar o ID da Locadora ao qual pertence, se ela existir no banco de dados
/// Primeiro, verifica se o CNPJ está vazio e retorna um erro caso esteja.
/// Em seguida, formata o CNPJ para corrigir erros de digitação. Após essa formatação, o CNPJ é passado para a função que faz a busca no banco de dados:
/// ```
/// let cnpj = formata_cnpj(cnpj)?;
/// let resultado: Result<String, mysql_async::Error> = model::locadora::_busca_id_locadora(&cnpj).await;
/// ```
/// Se `resultado` não retornar um erro, o ID que foi encontrado é retornado para o chamador:
/// ```
/// Ok(id: String) =>{
///     return Ok(id);
/// }
/// ```
#[tauri::command]
pub async fn busca_id_locadora(cnpj: &str) -> Result<String, String>{
    if cnpj.trim().is_empty(){
        return Err(MeuErro::CnpjVazio.to_string());
    }
    let cnpj = formata_cnpj(cnpj)?;
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

/// ## Recebe um serde_json::Value contendo os dados de uma Locadora e converte para o tipo `Locadora`  
/// Os primeiros passos são manipulações do campo `idlocadora`, que é convertido para String e então cortado para ter no máximo 45 caracteres (o limite atual desse ID no banco):
/// ```
/// let idlocadora: String = locadora["idlocadora"].as_str().unwrap_or("").to_string();
/// let idlocadora: (&str, &str) = idlocadora.split_at(45 as usize);
/// let idlocadora: String = idlocadora.0.to_string();
/// ```
/// Em seguida, os demais campos são convertidos em String e o campo `locadorastatus` é definido como `1`: 
/// ```
/// let locadora: model::locadora::Locadora = model::locadora::Locadora {
///     [...]
///     idsocio: locadora["idsocio"].as_str().unwrap_or("").to_string(),
///     locadorastatus: 1
/// };
/// ```
/// Finalmente, a função verifica se algum dos campos convertidos está vazio e retorna um erro caso ao menos um esteja.
/// Se a verificação não detectar campos vazios, retorna um objeto do tipo `Locadora`.
/// ```
/// if locadora.idendereco.trim().is_empty() || locadora.cnpj.trim().is_empty() || 
///     locadora.numerocontabanco.trim().is_empty()
///     || locadora.numeroagenciabanco.trim().is_empty() || 
///     locadora.nomebanco.trim().is_empty() || locadora.nomelocadora.trim().is_empty() || 
///     locadora.idsocio.trim().is_empty(){
///     return Err(MeuErro::CamposVazios.to_string());
/// }
/// return Ok(locadora);
/// ```
fn valida_locadora(input: Json<Locadora>) -> Result<Locadora, String>{
    let idlocadora: String = input.idlocadora.to_string();
    let idlocadora: (&str, &str) = idlocadora.split_at(45 as usize);
    let idlocadora: String = idlocadora.0.to_string();

    let cnpj = input.cnpj.to_string();
    let cnpj = formata_cnpj(&cnpj)?;

    let idendereco = input.idendereco.to_string();
    let numerocontabanco = input.numerocontabanco.to_string();
    let numeroagenciabanco = input.numeroagenciabanco.to_string();
    let nomebanco = input.nomebanco.to_string();
    let nomelocadora = input.nomelocadora.to_string();
    let idsocio = input.idsocio.to_string();

    let locadora: model::locadora::Locadora = model::locadora::Locadora {
        idlocadora,
        idendereco,
        cnpj,
        numerocontabanco,
        numeroagenciabanco,
        nomebanco,
        nomelocadora,
        idsocio,
        locadorastatus: 1
    };
    if locadora.idendereco.trim().is_empty() || locadora.cnpj.trim().is_empty() || 
        locadora.numerocontabanco.trim().is_empty()
        || locadora.numeroagenciabanco.trim().is_empty() || 
        locadora.nomebanco.trim().is_empty() || locadora.nomelocadora.trim().is_empty() ||
        locadora.idsocio.trim().is_empty(){
            return Err(MeuErro::CamposVazios.to_string());
    }
    return Ok(locadora);
}

/// ## Recebe um CNPJ, que é formatado e usado para buscar dados de uma Locadora no banco de dados
/// Primeiro, faz a formatação do parâmetro CNPJ e usa o resultado para buscar uma Locadora no banco:
/// ```
/// let cnpj = formata_cnpj(cnpj)?;
/// let locadora = match model::locadora::locadora_existente(&cnpj).await{
///    Ok(locadora) => {locadora},
///    Err(e) => {return Err(e.to_string())}
/// };
/// ```
/// Após uma busca bem-sucedida, converte os dados recebidos para um serde_json::Value, que é retornado para o chamador:
/// ```
/// let locadora = serde_json::json!({
///     "idlocadora": locadora.idlocadora,
///     "idendereco": locadora.idendereco,
///     "cnpj": locadora.cnpj,
///     "numerocontabanco": locadora.numerocontabanco,
///     "numeroagenciabanco": locadora.numeroagenciabanco,
///     "nomebanco": locadora.nomebanco,
///     "nomelocadora": locadora.nomelocadora,
///     "idsocio": locadora.idsocio,
///     "locadorastatus": locadora.locadorastatus
/// });
/// return Ok(locadora)
/// ```
#[tauri::command]
pub async fn locadora_existente(cnpj: &str) -> Result<serde_json::Value, String>{
    let cnpj = formata_cnpj(cnpj)?;
    let locadora = match model::locadora::locadora_existente(&cnpj).await{
        Ok(locadora) => {locadora},
        Err(e) => {return Err(e.to_string())}
    };
    let locadora = serde_json::json!({
        "idlocadora": locadora.idlocadora,
        "idendereco": locadora.idendereco,
        "cnpj": locadora.cnpj,
        "numerocontabanco": locadora.numerocontabanco,
        "numeroagenciabanco": locadora.numeroagenciabanco,
        "nomebanco": locadora.nomebanco,
        "nomelocadora": locadora.nomelocadora,
        "idsocio": locadora.idsocio,
        "locadorastatus": locadora.locadorastatus
    });
    return Ok(locadora)
}

/// ## Recebe um CNPJ e faz a formatação adequada para ele
/// Primeiro, remove todos os caracteres não-numéricos presentes na string slice:
/// ```
/// let cnpj_numeros: Vec<char> = cnpj
///     .chars()
///     .filter(|c: &char| c.is_digit(10))
///     .collect();
/// ```
/// Se a quantidade de números resultante for diferente de 14, retorna um erro.
///  Se a quantidade for exatamente 14, converte a string slice em um vetor de caracteres:
/// ```
/// let mut cnpj: Vec<char> = cnpj_numeros;
/// ```
/// A função insere pontos e traços nas posições adequadas do vetor, criando um CNPJ formatado:
/// ```
/// cnpj.insert(2, '.');
/// cnpj.insert(6, '.');
/// cnpj.insert(10, '/');
/// cnpj.insert(15, '-');
/// ```
/// É criada uma nova String mutável vazia, que receberá cada caractere do vetor durante uma iteração, e que por fim será retornada para o chamador:
/// ```
/// let mut cnpjfinal: String = "".to_string();
/// for u in cnpj{
///    cnpjfinal.push(u);
/// }
/// return Ok(cnpjfinal);
/// ```
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

/// ## Recebe um idusuario e um CNPJ, que são usados para verificar no banco se o Usuário é o sócio atual de uma Locadora
/// Verifica se os campos idusuario ou CNPJ estão vazios, e retorna erro caso estejam.
/// Caso a verificação não faça a função retornar um erro, formata o CNPJ recebido e usa ele e o idusuario para verificar no banco se o usuário é o sócio atual da Locadora. Note que a função não retorna nada após essa verificação:
/// ```
/// let cnpj = formata_cnpj(&cnpj)?;
/// let resultado_verificacao = model::locadora::verifica_usuario_socio_locadora(idusuario, cnpj).await;
/// ```
#[tauri::command]
pub async fn verifica_usuario_socio_locadora(idusuario: String, cnpj: String) -> Result<(), String>{
    if idusuario.trim().is_empty() || cnpj.trim().is_empty(){
       return Err(MeuErro::CamposVazios.to_string())
    }
    let cnpj = formata_cnpj(&cnpj)?;
    let resultado_verificacao = model::locadora::verifica_usuario_socio_locadora(idusuario, cnpj).await;
    match resultado_verificacao{
        Ok(_) => {return Ok(())},
        Err(e) => {return Err(e.to_string())}
    }
}

