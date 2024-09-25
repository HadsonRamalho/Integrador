use mysql_async::prelude::*;
use serde::{Deserialize, Serialize};
use crate::model::erro::MeuErro;
use crate::controller::{self, cria_pool};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Endereco{
    pub idendereco: String,
    pub logradouro: String,
    pub cep: String,
    pub complemento: String,
    pub numeroendereco: String,
    pub cidade: String,
    pub uf: String,
}


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
pub async fn salva_endereco(endereco: Endereco) -> Result<String, mysql_async::Error> {
    // Cria uma conexão com o pool do banco de dados
    let pool = controller::cria_pool().await?;
    let mut conn = pool.get_conn().await?; 
    
    let id_retorno = endereco.idendereco.to_string(); // faz uma cópia do id do endereço
    // Insere o endereço na tabela `endereco`
    conn.exec_drop(
        "INSERT INTO endereco (idendereco, logradouro, cep, complemento, numeroendereco, cidade, uf)
         VALUES (:idendereco, :logradouro, :cep, :complemento, :numeroendereco, :cidade, :uf)",
        params! {"idendereco" => endereco.idendereco, "logradouro" => endereco.logradouro,
        "cep" => endereco.cep, "complemento" => endereco.complemento,
        "numeroendereco" => endereco.numeroendereco, "cidade" => endereco.cidade,
        "uf" => endereco.uf},
    ).await?;

    println!("Endereço salvo com sucesso");

    return Ok(id_retorno) // retorna o id do endereço
}

pub async fn busca_endereco_id(id: &str) -> Result<Endereco, mysql_async::Error>{
    let pool = cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let resultado_busca = 
        conn.exec_first("SELECT * FROM endereco WHERE idendereco = :id",
        params! {"id" => id} ).await?;
    match resultado_busca{
        None => {return Err(mysql_async::Error::Other(Box::new(MeuErro::EnderecoNaoEncontrado)))},
        Some(endereco) => {return Ok(endereco)}
    }
}