use axum::Json;
use mysql_async::{params, prelude::Queryable};

use crate::{controller::{cria_pool, endereco::{self, busca_endereco_id, estrutura_endereco}}, model::endereco::salva_endereco};

#[cfg(test)]

fn cria_endereco_teste(logradouro: &str, cep: &str, complemento: &str, numeroendereco: &str, cidade: &str, uf: &str) -> crate::controller::endereco::EnderecoInput{
    use crate::controller::endereco::EnderecoInput;

    EnderecoInput{
        logradouro: logradouro.to_string(),
        cep: cep.to_string(),
        complemento: complemento.to_string(),
        numeroendereco: numeroendereco.to_string(),
        cidade: cidade.to_string(),
        uf: uf.to_string(),
    }
}

#[tokio::test]
async fn test_estrutura_endereco_ok(){
    use axum::Json;

    let input = crate::controller::endereco::EnderecoInput{
        logradouro: "Rua TESTE".to_string(),
        cep: "39600-000".to_string(),
        complemento: "T".to_string(),
        numeroendereco: "123A".to_string(),
        cidade: "Araçuaí".to_string(),
        uf: "MG".to_string(),
    };
    assert!(crate::controller::endereco::estrutura_endereco(Json(input)).is_ok());
}

async fn deleta_endereco(idendereco: String) -> Result<(), mysql_async::Error>{
    let pool = cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let resultado_delete = conn.exec_drop("DELETE FROM endereco WHERE idendereco = :id", 
        params! {"id" => idendereco}).await;
    match resultado_delete{
        Ok(_) => {return Ok(())},
        Err(e) => {
            return Err(e)
        }
    }
}

#[tokio::test]
async fn test_cria_deleta_endereco_ok(){
    let endereco = crate::controller::endereco::EnderecoInput{
        logradouro: "Rua TESTE".to_string(),
        cep: "39600-000".to_string(),
        complemento: "T".to_string(),
        numeroendereco: "Teste0".to_string(),
        cidade: "Araçuaí".to_string(),
        uf: "MG".to_string(),
    };
    let endereco: crate::controller::endereco::EnderecoInput = endereco.into();
    let idendereco = match crate::controller::endereco::_salva_endereco(Json(endereco)).await{
        Ok(idendereco) => {idendereco.0},
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
    };

    assert!(deleta_endereco(idendereco).await.is_ok())

}

#[tokio::test]
async fn test_busca_endereco_id(){
    let endereco = cria_endereco_teste(
        "Rua TESTE",
        "39600-000",
        "T",
        "123A",
        "Araçuaí",
        "MG",
    );
    let endereco = match estrutura_endereco(Json(endereco)){
        Ok(endereco) => {endereco},
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
    };
    let idendereco = endereco.1.idendereco.clone();
    assert!(salva_endereco(endereco.1.into()).await.is_ok());
    assert!(busca_endereco_id(Json(idendereco.clone())).await.is_ok());
    assert!(deleta_endereco(idendereco).await.is_ok())

}