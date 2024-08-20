use mysql_async::prelude::*;
use crate::model;
use crate::controller::endereco::Endereco;

/// Função para salvar um endereço no banco de dados.
///
/// Esta função recebe um objeto `serde_json::Value` contendo os dados de um endereço,
/// cria uma conexão com o banco de dados, e insere os dados na tabela `endereco`.
///
/// # Parâmetros
/// - `endereco`: Um objeto JSON contendo os dados do endereço, como logradouro, CEP, complemento, número, cidade e UF.
///
/// # Retornos
/// - `Result<bool, mysql_async::Error>`: Retorna `Ok(true)` se o endereço for salvo com sucesso.
///   Em caso de falha na conexão ou na execução da query, retorna um erro do tipo `mysql_async::Error`.
///
/// # Exceções
/// - Pode retornar um erro se houver problemas ao conectar ao banco de dados ou ao executar a query.
pub async fn salva_endereco(endereco: serde_json::Value) -> Result<String, mysql_async::Error> {
    // Cria uma conexão com o pool do banco de dados
    let pool = model::create_pool().await?;
    let mut conn = pool.get_conn().await?; 

    // Converte os dados recebidos em um objeto `Endereco`
    let endereco = Endereco {
        id: endereco["id"].as_str().unwrap_or("").to_string().split_off(15 as usize),
        logradouro: endereco["logradouro"].as_str().unwrap_or("").to_string(),
        cep: endereco["cep"].as_str().unwrap_or("").to_string(),
        complemento: endereco["complemento"].as_str().unwrap_or("").to_string(),
        numeroendereco: endereco["numeroendereco"].as_str().unwrap_or("").to_string(),
        cidade: endereco["cidade"].as_str().unwrap_or("").to_string(),
        uf: endereco["uf"].as_str().unwrap_or("").to_string(),
    };
    let id_retorno = endereco.id.to_string(); // faz uma cópia do id do endereço
    // Insere o endereço na tabela `endereco`
    conn.exec_drop(
        "INSERT INTO endereco (idendereco, logradouro, cep, complemento, numeroendereco, cidade, uf) VALUES (?, ?, ?, ?, ?, ?, ?)",
        params! {"idendereco" => endereco.id, "logradouro" => endereco.logradouro,
        "cep" => endereco.cep, "complemento" => endereco.complemento,
        "numeroendereco" => endereco.numeroendereco, "cidade" => endereco.cidade,
        "uf" => endereco.uf},
    ).await?;

    println!("Endereço salvo com sucesso");

    return Ok(id_retorno) // retorna o id do endereço
}
