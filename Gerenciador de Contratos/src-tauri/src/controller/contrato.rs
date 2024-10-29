use axum::Json;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use mysql_async::Row;
use mysql_async::Value;

use crate::model::erro::MeuErro;
use crate::model::{self, contrato::Contrato};
use crate::controller;

use super::gera_hash;
use super::locadora::formata_cnpj;
use super::maquina::formata_valor_f32;
use super::usuario::busca_cnpj_usuario;

/// ## Busca um contrato pelo nome da máquina, selecionando apenas os que pertencerem à empresa do usuário.
/// 
    /// Primeiro, faz verificações nos parâmetros nome_maquina e idusuario, retornando um erro caso essas verificações falhem.
    /// ```
    /// let idusuario = idusuario.trim();
    /// if nome_maquina.trim().is_empty() || idusuario.is_empty(){
    ///     return Err("Erro: Um ou mais campos estão vazios".to_string())
    /// }
    /// ```
    /// Em seguida, cria uma pool de conexões (normalmente isso é feito no banco), e então chama uma função que busca o CNPJ do usuário
    /// usando o idusuario.
    /// ``` 
    /// model::usuario::busca_cnpj_usuario(&pool, &idusuario).await;
    /// ```
    /// Caso a função de busca retorne um CNPJ, ele é atribuído à variável cnpj. Caso contrário, a função retorna um erro.
    /// 
    /// Finalmente, passa o nome_maquina e o CNPJ para a função
    /// ```
    /// model::contrato::busca_contrato_nome_maquina(nome_maquina, cnpj).await;
    /// ```
    /// Caso a função não retorne um erro, é feita uma verificação no vetor retornado:
    /// ```
    /// Ok(resultado) => {
    ///     if !resultado.is_empty(){
    ///         return Ok(resultado);
    ///     }
    ///     return Err("Erro: Máquina não encontrada".to_string());
    /// }
    /// ```
    /// Se essa última verificação não retornar um erro, o vetor de Contrato é retornado para o front.
#[tauri::command]
pub async fn busca_contrato_nome_maquina(nome_maquina: String, idusuario: String) -> Result<Vec<model::contrato::Contrato>, String>{
    let idusuario = idusuario.trim();
    if nome_maquina.trim().is_empty() || idusuario.is_empty(){
        return Err("Erro: Um ou mais campos estão vazios".to_string())
    }
    let pool = match controller::cria_pool().await {
        Ok(pool) => {
            pool
        }, 
        Err(e) =>{
            return Err(e.to_string())
        }
    };
    let cnpj = model::usuario::busca_cnpj_usuario(&pool, &idusuario).await;
    let cnpj = match cnpj{
        Ok(cnpj) => {
            cnpj
        }, Err(_) => {
            return Err("Erro: O usuário não tem um CNPJ cadastrado.".to_string())
        }
    };

    let resultado_busca: Result<Vec<model::contrato::Contrato>, mysql_async::Error> = model::contrato::busca_contrato_nome_maquina(nome_maquina, cnpj).await;

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

/// ## Recebe um Value e formata ele para um NaiveDate ou NaiveDateTime
/// Primeiro, verifica se o Value é do tipo Value::Date, que possua ano, mês, dia, horas, minutos, segundos e microssegundos (cujo valor não é utilizado).
/// ```
/// Value::Date(ano, mes, dia, hora, minuto, segundo, _microssegundo)
/// ```
/// Se as horas, minutos e segundos forem iguais a 0(zero), cria um novo NaiveDate usando o ano, mês e dia.
///  Após essa verificação, a data é transformada na String que é retornada.
/// ```
/// if hora == 0 && minuto == 0 && segundo == 0 {
///     let data = NaiveDate::from_ymd_opt(ano as i32, mes as u32, dia as u32).unwrap()
///         .format("%Y-%m-%d")
///         .to_string();
///      return data;
/// }
/// ```
/// Se o Value possuir horas, minutos ou segundos diferentes de 0(zero), a data é transformada em um NaiveDateTime, que também
/// passa pelo processo de formatação e retorno como String
/// ```
///  let data_precisa = NaiveDateTime::new(
///     NaiveDate::from_ymd_opt(ano as i32, mes as u32, dia as u32).unwrap(),
///     NaiveTime::from_hms_opt(hora as u32, minuto as u32, segundo as u32).unwrap()
///  )
///     .format("%Y-%m-%d %H:%M:%S")
///     .to_string();
///  return data_precisa;
/// ```
/// Se o Value não for do tipo Date, retorna uma String com erro.
fn formata_data(value: Value) -> String {
    match value {
        Value::Date(ano, mes, dia, hora, minuto, segundo, _microssegundo) => {
            if hora == 0 && minuto == 0 && segundo == 0 {
                // se o horário for 00:00:00, trata como Date
                let data = NaiveDate::from_ymd_opt(ano as i32, mes as u32, dia as u32).unwrap()
                    .format("%Y-%m-%d")
                    .to_string();
                return data;
            }
            // trata como DateTime
            let data_precisa = NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(ano as i32, mes as u32, dia as u32).unwrap(),
                    NaiveTime::from_hms_opt(hora as u32, minuto as u32, segundo as u32).unwrap()
                )
                .format("%Y-%m-%d %H:%M:%S")
                .to_string();
            return data_precisa;            
        },
        _ => "Erro: Formato de data inválido".to_string(),
    }
}
/// ## Estrutura um contrato, transformando campos separados em um valor do tipo serde_json::Value
/// Primeiro, verifica se algum dos campos necessários está vazio, retornando um erro caso a verificação detecte campos inválidos.
/// Em seguida, remove espaços de campos.
/// ```
/// let idlocatario = idlocatario.trim().to_string();
/// let idlocador = idlocador.trim().to_string();
/// let idmaquina = idmaquina.trim().to_string();
/// let enderecoretirada = enderecoretirada.trim().to_string();
/// ```
/// 
/// Gera um hash aleatório usando o ID do endereço de retirada como parâmetro.
/// 
/// ```
/// let idcontrato = gera_hash(&enderecoretirada); 
/// ```
/// 
/// #### Após tratar os campos, cria um novo objeto do tipo `serde_json::Value`, atribuindo os valores dos campos às chaves adequadas
#[tauri::command]
pub async fn estrutura_contrato(
        idlocatario: String, 
        idlocador: String, 
        idmaquina: String, 
        enderecoretirada: String,
        prazolocacao: String,
        avisotransferencia: String,
        cidadeforo: String,
        datacontrato: String,
        dataretirada: String,
        valormensal: String,
        vencimento: String,
        multaatraso: String,
        jurosatraso: String,
        prazodevolucao: String) -> Result<serde_json::Value, String>{

    if idlocatario.trim().is_empty() || idlocador.trim().is_empty()
     || idlocatario.trim().is_empty() || idmaquina.trim().is_empty() || enderecoretirada.trim().is_empty() ||
    prazolocacao.trim().is_empty() || avisotransferencia.trim().is_empty() || cidadeforo.trim().is_empty() || 
    datacontrato.trim().is_empty() || dataretirada.trim().is_empty() ||
    valormensal.trim().is_empty() || vencimento.trim().is_empty() || multaatraso.trim().is_empty()
     || jurosatraso.trim().is_empty() || prazodevolucao.trim().is_empty(){
        return Err("Erro: Um ou mais campos estão vazios.".to_string())
    }

    let idlocatario = idlocatario.trim().to_string();
    let idlocador = idlocador.trim().to_string();
    let idmaquina = idmaquina.trim().to_string();
    let enderecoretirada = enderecoretirada.trim().to_string();

    let idcontrato = gera_hash(&enderecoretirada);

    let contrato =  serde_json::json!({
        "idcontrato": idcontrato,
        "idlocador": idlocador, 
        "idlocatario": idlocatario,
        "idmaquina": idmaquina,
        "enderecoretirada": enderecoretirada,
        "prazolocacao": prazolocacao,
        "avisotransferencia": avisotransferencia, 
        "cidadeforo": cidadeforo, 
        "datacontrato": datacontrato, 
        "dataretirada": dataretirada,
        "valormensal": valormensal, 
        "vencimento": vencimento, 
        "multaatraso": multaatraso, 
        "jurosatraso": jurosatraso, 
        "prazodevolucao": prazodevolucao
    });

    return Ok(contrato)
}

/// ## Recebe um contrato no formato serde_json::Value e cadastra os dados no banco de dados
/// Adquire todos os dados do parâmetro contrato, e faz a conversão necessária para cada um, convertendo para uma String vazia,
/// quando necessário. Também corta o tamanho dos IDs, pra que sejam compatíveis com os campos no banco de dados.
/// #### Exemplo:
/// ```
/// let idlocatario: String = contrato["idlocatario"].as_str().unwrap_or("").to_string();
/// let idlocatario: (&str, &str) = idlocatario.split_at(45 as usize);
/// let idlocatario: String = idlocatario.0.to_string();
/// [...]
/// ```
/// Além de converter Strings, também faz a conversão para Float(f32):
/// ```
/// let prazolocacao:f32 = match contrato["prazolocacao"].as_str().unwrap_or("").to_string().trim().parse(){
///        Ok(prazolocacao) => {prazolocacao},
///        Err(e) => {return Err(format!("Erro ao converter prazo de locação: {}", e))}
///    };
/// ```
/// Após essas conversões, verifica se algum dos campos convertidos para Strings está vazio:
/// ```
/// if idlocatario.is_empty() || idlocador.is_empty() || idmaquina.is_empty() || enderecoretirada.is_empty() ||
/// dataretirada.is_empty() || vencimento.is_empty() || avisotransferencia.is_empty() || 
/// prazodevolucao.is_empty() || cidadeforo.is_empty() || datacontrato.is_empty(){
///    return Err(MeuErro::CamposVazios.to_string())
/// }
/// ```
/// Gera um hash que se tornará o ID do Contrato que será registrado no banco:
/// ```
/// let idcontrato = gera_hash(&idlocatario);
/// let idcontrato: (&str, &str) = idcontrato.split_at(45 as usize);
/// let idcontrato: String = idcontrato.0.to_string();
/// ```
/// Para construir o objeto do tipo Contrato, converte os demais campos e atribui os já convertidos aos equivalentes da struct:
/// ```
/// let contrato: Contrato = Contrato {idcontrato,
///     prazolocacao, 
///     dataretirada: contrato["dataretirada"].as_str().unwrap_or("").to_string(), 
///     valormensal, 
///     [...]
/// ```
/// Finalmente, chama a função que tenta alugar a máquina ao atualizar o campo Disponibilidade no banco de dados:
/// ```
/// let resultado_aluguel = model::maquina::aluga_maquina(&idmaquina_cpy).await;
/// ```
/// Após verificações, o contrato pode ser registrado no banco:
/// ```
/// let resultado_cadastro = model::contrato::registra_contrato(contrato).await;
/// ```
#[tauri::command]
pub async fn cadastra_contrato(contrato: serde_json::Value) -> Result<(), String>{
    let idlocatario: String = contrato["idlocatario"].as_str().unwrap_or("").to_string();
    let idlocatario: (&str, &str) = idlocatario.split_at(45 as usize);
    let idlocatario: String = idlocatario.0.to_string();

    let idlocador: String = contrato["idlocador"].as_str().unwrap_or("").to_string();
    let idlocador: (&str, &str) = idlocador.split_at(45 as usize);
    let idlocador: String = idlocador.0.to_string();

    let idmaquina: String = contrato["idmaquina"].as_str().unwrap_or("").to_string();
    let idmaquina: (&str, &str) = idmaquina.split_at(45 as usize);
    let idmaquina: String = idmaquina.0.to_string();

    let enderecoretirada: String = contrato["enderecoretirada"].as_str().unwrap_or("").to_string();
    let enderecoretirada: (&str, &str) = enderecoretirada.split_at(45 as usize);
    let enderecoretirada: String = enderecoretirada.0.to_string();

    let prazolocacao:f32 = match contrato["prazolocacao"].as_str().unwrap_or("").to_string().trim().parse(){
        Ok(prazolocacao) => {prazolocacao},
        Err(e) => {return Err(format!("Erro ao converter prazo de locação: {}", e))}
    };

    let valormensal= contrato["valormensal"].as_str().unwrap_or("").to_string();
    let valormensal = formata_valor_f32(&valormensal)?;
    let multaatraso:f32 = match contrato["multaatraso"].as_str().unwrap_or("").to_string().trim().parse(){
        Ok(multaatraso) => {multaatraso},
        Err(e) => {return Err(format!("Erro ao converter multa de atraso: {}", e))}
    };

    let jurosatraso:f32 = match contrato["jurosatraso"].as_str().unwrap_or("").to_string().trim().parse(){
        Ok(jurosatraso) => {jurosatraso},
        Err(e) => {return Err(format!("Erro ao converter juros de atraso: {}", e))}
    };

    let dataretirada = contrato["dataretirada"].as_str().unwrap_or("").to_string();
    let vencimento = contrato["vencimento"].as_str().unwrap_or("").to_string();
    let avisotransferencia = contrato["avisotransferencia"].as_str().unwrap_or("").to_string();
    let prazodevolucao = contrato["prazodevolucao"].as_str().unwrap_or("").to_string();
    let cidadeforo = contrato["cidadeforo"].as_str().unwrap_or("").to_string();
    let datacontrato = contrato["datacontrato"].as_str().unwrap_or("").to_string();

    if idlocatario.is_empty() || idlocador.is_empty() || idmaquina.is_empty() || enderecoretirada.is_empty() ||
    dataretirada.is_empty() || vencimento.is_empty() || avisotransferencia.is_empty() || 
    prazodevolucao.is_empty() || cidadeforo.is_empty() || datacontrato.is_empty(){
        return Err(MeuErro::CamposVazios.to_string())
    }

    let idmaquina_cpy = idmaquina.clone();

    let idcontrato = gera_hash(&idlocatario);
    let idcontrato: (&str, &str) = idcontrato.split_at(45 as usize);
    let idcontrato: String = idcontrato.0.to_string();

    let contrato: Contrato = Contrato {idcontrato,
        prazolocacao, 
        dataretirada, 
        valormensal, 
        vencimento, 
        multaatraso, 
        jurosatraso, 
        avisotransferencia, 
        prazodevolucao, 
        cidadeforo, 
        datacontrato, 
        idlocatario, 
        idlocador, 
        idmaquina, 
        enderecoretirada        
    };
    let resultado_aluguel = model::maquina::aluga_maquina(&idmaquina_cpy).await;
    match resultado_aluguel{
        Ok(_) => {},
        Err(e) => {
            println!("{:?}", e);
            return Err(e.to_string())
        }
    }
    let resultado_cadastro = model::contrato::registra_contrato(contrato).await;
    match resultado_cadastro{
        Ok(_) => {
            return Ok(());
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

/// ## Busca contratos que ainda não venceram 
/// 
/// Recebe o idusuario como parâmetro e verifica se ele está vazio:
/// ```
/// let idusuario = idusuario.trim();
/// if idusuario.is_empty(){
///     return Err("Erro: Um ou mais campos estão vazios".to_string())
/// }
/// ```
/// Chama a função de busca CNPJ do usuário, recebendo de volta um CNPJ.
/// ```
/// let cnpj = model::usuario::busca_cnpj_usuario(&pool, &idusuario).await;
/// ```
/// Caso a busca retorne erro, a função retorna um erro em formato de String para o chamador.
/// Caso contrário, a função chama a função que busca os contratos a vencer que estejam relacionados ao CNPJ do usuário:
/// ```
/// let resultado_busca: Result<Vec<model::contrato::Contrato>, mysql_async::Error> = model::contrato::busca_contratos_a_vencer(cnpj).await;
/// ```
/// #### Enfim, a função verifica se o vetor de Contrato está vazio, e retorna um erro caso esteja. Se o vetor possuir ao menos um registro, retorna esse registro dentro do vetor.
#[tauri::command]
pub async fn busca_contratos_a_vencer(idusuario: String) -> Result<Vec<model::contrato::Contrato>, String>{
    let idusuario = idusuario.trim();
    if idusuario.is_empty(){
        return Err("Erro: Um ou mais campos estão vazios".to_string())
    }
    let pool = match controller::cria_pool().await {
        Ok(pool) => {
            pool
        }, 
        Err(e) =>{
            return Err(e.to_string())
        }
    };
    let cnpj = model::usuario::busca_cnpj_usuario(&pool, &idusuario).await;
    let cnpj = match cnpj{
        Ok(cnpj) => {
            cnpj
        }, Err(_) => {
            return Err("Erro: O usuário não tem um CNPJ cadastrado.".to_string())
        }
    };
    let resultado_busca: Result<Vec<model::contrato::Contrato>, mysql_async::Error> = model::contrato::busca_contratos_a_vencer(cnpj).await;

    match resultado_busca{
        Ok(resultado) => {
            if !resultado.is_empty(){
                return Ok(resultado);
            }
            return Err("Erro: Não existem contratos a vencer OU não há um contrato cadastrado".to_string());
        },
        Err(erro) => {
            return Err(erro.to_string());
        }
    }
}

/// ## Converte um vetor do tipo Row para um vetor do tipo Contrato
/// Converte os campos para String ou f32, a depender das especificações da struct Contrato:
/// ```
/// let idcontrato = row.get::<String, _>("idcontrato").unwrap_or_default();
/// let prazolocacao = row.get::<f32, _>("prazolocacao").unwrap_or_default();
/// ```
/// Ao final, estrutura esses campos em um vetor de Contrato e retorna para o chamador.
pub fn cria_vetor_contratos(row: Vec<Row>) -> Vec<Contrato>{
    let contratos: Vec<Contrato> =  row.into_iter().map(|row| {
        let idcontrato = row.get::<String, _>("idcontrato").unwrap_or_default();
        let prazolocacao = row.get::<f32, _>("prazolocacao").unwrap_or_default();
        let dataretirada = formata_data(row.get::<Value, _>("dataretirada").unwrap());
        let valormensal = row.get::<f32, _>("valormensal").unwrap_or_default();
        let vencimento = formata_data(row.get::<Value, _>("vencimento").unwrap());
        let multaatraso = row.get::<f32, _>("multaatraso").unwrap_or_default();
        let jurosatraso = row.get::<f32, _>("jurosatraso").unwrap_or_default();
        let avisotransferencia = row.get::<String, _>("avisotransferencia").unwrap();
        let prazodevolucao = formata_data(row.get::<Value, _>("prazodevolucao").unwrap());
        let cidadeforo = row.get::<String, _>("cidadeforo").unwrap_or_default();
        let datacontrato = formata_data(row.get::<Value, _>("datacontrato").unwrap());
        let idlocatario = row.get::<String, _>("idlocatario").unwrap_or_default();
        let idlocador = row.get::<String, _>("idlocador").unwrap_or_default();
        let idmaquina = row.get::<String, _>("idmaquina").unwrap_or_default();
        let enderecoretirada = row.get::<String, _>("enderecoretirada").unwrap_or_default();

        Contrato {
            idcontrato,
            prazolocacao,
            dataretirada,
            valormensal,
            vencimento,
            multaatraso,
            jurosatraso,
            avisotransferencia,
            prazodevolucao,
            cidadeforo,
            datacontrato,
            idlocatario,
            idlocador,
            idmaquina,
            enderecoretirada,
        }
    }).collect();
    return contratos
}


/// ## Busca contratos pelo número de série da máquina e pelo CNPJ da empresa do usuário.
/// Verifica se os parâmetros estão vazios e retorna um erro caso estejam.
/// ```
/// if numserie.trim().is_empty() || idusuario.trim().is_empty(){
///     return Err(MeuErro::CamposVazios.to_string())
/// }
/// ```
/// Busca o CNPJ do usuário e o formata novamente:
/// ```
/// let cnpj = busca_cnpj_usuario(idusuario).await?;
/// let cnpj = formata_cnpj(&cnpj)?;
/// ```
/// Busca contratos que possuam o número de série da máquina e que estejam relacionados ao CNPJ escolhido, retornando um vetor de Contrato caso a busca seja bem sucedida:
/// ```
/// let resultado_busca = model::contrato::busca_contrato_numserie_maquina(numserie, cnpj).await;
/// match resultado_busca{
///    Ok(contratos) => {return Ok(contratos)},
///    Err(e) => return Err(e.to_string())
/// }
/// ```
#[tauri::command]
pub async fn busca_contrato_numserie_maquina(numserie: String, idusuario: String) -> Result<Vec<Contrato>, String>{
    if numserie.trim().is_empty() || idusuario.trim().is_empty(){
        return Err(MeuErro::CamposVazios.to_string())
    }
    let numserie = numserie.trim().to_string();
    let cnpj = match busca_cnpj_usuario(Json(idusuario)).await{
        Ok(cnpj) => {cnpj.1},
        Err(e) => {
            return Err(e.1.0);
        }
    };
    let cnpj = formata_cnpj(&cnpj)?;
    let resultado_busca = model::contrato::busca_contrato_numserie_maquina(numserie, cnpj).await;
    match resultado_busca{
        Ok(contratos) => {return Ok(contratos)},
        Err(e) => return Err(e.to_string())
    }
}

/// ## Busca contratos pelo nome do locatário/empresa cliente.
/// Verifica se o nome do locatário ou ID do usuário estão vazios, retornando um erro caso estejam:
/// ```
/// if nomelocatario.trim().is_empty() || idusuario.trim().is_empty(){
///     return Err(MeuErro::CamposVazios.to_string())
/// }
/// ```
/// /// Busca o CNPJ do usuário e o formata novamente:
/// ```
/// let cnpj = busca_cnpj_usuario(idusuario).await?;
/// let cnpj = formata_cnpj(&cnpj)?;
/// ```
/// Busca contratos que possuam um nome semelhante ao nome do cliente e que estejam relacionados ao CNPJ escolhido, retornando um vetor de Contrato caso a busca seja bem sucedida:
/// ```
/// let resultado_busca = model::contrato::busca_contrato_nome_locatario(nomelocatario, cnpj).await;
/// match resultado_busca{
///    Ok(contratos) => {
///        if contratos.is_empty(){
///            return Err(MeuErro::ContratoNaoEncontrado.to_string())
///        }
///        return Ok(contratos)
///    },
///    Err(e) => {
///        return Err(e.to_string())
///    }
/// }
/// ```
#[tauri::command]
pub async fn busca_contrato_nome_locatario(nomelocatario: String, idusuario: String) -> Result<Vec<Contrato>, String>{
    if nomelocatario.trim().is_empty() || idusuario.trim().is_empty(){
        return Err(MeuErro::CamposVazios.to_string())
    }
    let cnpj = match busca_cnpj_usuario(Json(idusuario)).await{
        Ok(cnpj) => {cnpj.1},
        Err(e) => {
            return Err(e.1.0);
        }
    };
    let cnpj = formata_cnpj(&cnpj)?;
    let resultado_busca = model::contrato::busca_contrato_nome_locatario(nomelocatario, cnpj).await;
    match resultado_busca{
        Ok(contratos) => {
            if contratos.is_empty(){
                return Err(MeuErro::ContratoNaoEncontrado.to_string())
            }
            return Ok(contratos)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

/// ## Busca contratos pelo CNPJ do locatário/empresa cliente
/// Primeiro, verifica se os parâmetros CNPJ do locatário ou ID do usuário estão vazios, retornando um erro caso estejam:
/// ```
/// if cnpjlocatario.trim().is_empty() || idusuario.trim().is_empty(){
///     return Err(MeuErro::CamposVazios.to_string())
/// }
/// ```
/// Busca o CNPJ do usuário e o formata novamente.
/// A função então busca no banco por contratos que possuam o CNPJ do cliente e da empresa do usuário, retornando um vetor de Contrato caso a busca seja bem sucedida, ou um erro convertido para String:
/// ```
/// let resultado_busca = model::contrato::busca_contrato_cnpj_locatario(cnpjlocatario, cnpj).await;
/// match resultado_busca{
///    Ok(contratos) => {
///        if contratos.is_empty(){
///            return Err(MeuErro::ContratoNaoEncontrado.to_string())
///        }
///        return Ok(contratos)
///    },
///    Err(e) => { return Err(e.to_string()) }
/// ```
#[tauri::command]
pub async fn busca_contrato_cnpj_locatario(cnpjlocatario: String, idusuario: String) -> Result<Vec<Contrato>, String>{
    if cnpjlocatario.trim().is_empty() || idusuario.trim().is_empty(){
        return Err(MeuErro::CamposVazios.to_string())
    }
    let cnpj = match busca_cnpj_usuario(Json(idusuario)).await{
        Ok(cnpj) => {cnpj},
        Err(e) => {
            return Err(e.1.0);
        }
    };
    let cnpj = formata_cnpj(&cnpj.1)?;
    let cnpjlocatario = formata_cnpj(&cnpjlocatario)?;
    let resultado_busca = model::contrato::busca_contrato_cnpj_locatario(cnpjlocatario, cnpj).await;
    match resultado_busca{
        Ok(contratos) => {
            if contratos.is_empty(){
                return Err(MeuErro::ContratoNaoEncontrado.to_string())
            }
            return Ok(contratos)
        },
        Err(e) => { return Err(e.to_string()) }
    }
}