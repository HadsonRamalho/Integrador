use axum::Json;
use mysql_async::{params, prelude::Queryable};

use crate::{controller::{cria_pool, endereco::{self, busca_endereco_id, estrutura_endereco}}, model::endereco::salva_endereco};


pub async fn cria_endereco_teste(logradouro: &str, cep: &str, complemento: &str, numeroendereco: &str, cidade: &str, uf: &str) -> Result<String, String>{
    use crate::controller::endereco::{EnderecoInput, _salva_endereco};

    let enderecoinput = EnderecoInput{
        logradouro: logradouro.to_string(),
        cep: cep.to_string(),
        complemento: complemento.to_string(),
        numeroendereco: numeroendereco.to_string(),
        cidade: cidade.to_string(),
        uf: uf.to_string(),
    };
    let endereco = estrutura_endereco(Json(enderecoinput.clone())).unwrap().1;
    let resultado = _salva_endereco(Json(enderecoinput)).await;
    match resultado{
        Ok(idendereco) => {
            return Ok(idendereco.to_string())
        },
        Err(e) => {
            return Err(e.1.to_string())
        }
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

pub async fn _limpa_endereco(idendereco: String) -> Result<(), String>{
    match deleta_endereco(idendereco).await{
        Ok(_) => {
            return Ok(())
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

pub async fn deleta_endereco(idendereco: String) -> Result<(), mysql_async::Error>{
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

    assert!(_limpa_endereco(idendereco).await.is_ok())

}

#[tokio::test]
async fn test_busca_endereco_id(){
    let endereco = crate::controller::endereco::EnderecoInput{
        logradouro: "Rua TESTE".to_string(),
        cep: "39600-000".to_string(),
        complemento: "T".to_string(),
        numeroendereco: "Teste0".to_string(),
        cidade: "Araçuaí".to_string(),
        uf: "MG".to_string(),
    };
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
    assert!(_limpa_endereco(idendereco).await.is_ok())

}