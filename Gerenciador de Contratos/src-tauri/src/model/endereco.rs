use diesel::sql_types::Cidr;
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

impl From<axum::Json<Endereco>> for Endereco{
    fn from(value: axum::Json<Endereco>) -> Self {
        Endereco{
            idendereco: value.idendereco.clone(),
            logradouro: value.logradouro.clone(),
            cep: value.cep.clone(),
            complemento: value.cep.clone(),
            numeroendereco: value.numeroendereco.clone(),
            cidade: value.cidade.clone(),
            uf: value.uf.clone()
        }
    }
}

pub async fn salva_endereco(endereco: Endereco) -> Result<String, mysql_async::Error> {
    // Cria uma conexão com o pool do banco de dados
    let pool = controller::cria_pool().await?;
    let mut conn = pool.get_conn().await?; 
    
    let id_retorno = endereco.idendereco.to_string(); // faz uma cópia do id do endereço
    // Insere o endereço na tabela `endereco`
    let resultado_insert: Result<(), mysql_async::Error> = conn.exec_drop(
        "INSERT INTO endereco (idendereco, logradouro, cep, complemento, numeroendereco, cidade, uf)
         VALUES (:idendereco, :logradouro, :cep, :complemento, :numeroendereco, :cidade, :uf)",
        params! {"idendereco" => endereco.idendereco, "logradouro" => endereco.logradouro,
        "cep" => endereco.cep, "complemento" => endereco.complemento,
        "numeroendereco" => endereco.numeroendereco, "cidade" => endereco.cidade,
        "uf" => endereco.uf},
    ).await;

    match resultado_insert{
        Ok(_) => {
            println!("Endereço salvo com sucesso");
            return Ok(id_retorno)
        },
        Err(e) => {
            println!("{:?}", e);
            return Err(mysql_async::Error::Other(Box::new(MeuErro::SalvarEndereco)))
        }
    }
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