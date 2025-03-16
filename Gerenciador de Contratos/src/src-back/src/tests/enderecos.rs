use axum::{extract::Query, Json};

use crate::{controllers::{enderecos::{busca_endereco_id, busca_endereco_idusuario, cadastra_endereco, EnderecoInput}, enderecos_usuarios::{busca_enderecousuario_idusuario, cadastra_endereco_usuario, EnderecoUsuarioInput}, gera_hash, usuarios::cadastra_usuario}, models::{enderecos::{deleta_endereco, Endereco}, enderecos_usuarios::deleta_endereco_usuario, usuarios::deleta_usuario}, tests::usuarios::usuario_padrao};

pub fn endereco_padrao(numeroteste: &str) -> EnderecoInput{
    let pais = format!("País Teste {}", numeroteste);
    let estado = format!("Estado Teste {}", numeroteste);
    let cidade = format!("Cidade Teste {}", numeroteste);
    let cep = format!("39606-{}", numeroteste);
    let bairro =  format!("Bairro Teste {}", numeroteste);
    let logradouro = format!("Logradouro Teste {}", numeroteste);
    let numero = format!("Numero Teste {}", numeroteste);
    let complemento = Some(format!("Complemento Teste {}", numeroteste));

    EnderecoInput{
        pais,
        estado,
        cidade,
        cep,
        bairro,
        logradouro,
        numero,
        complemento,
    }
}


#[tokio::test]
async fn test_cadastra_endereco_ok(){
    let endereco = endereco_padrao("300");
    
    let id = cadastra_endereco(Json(endereco)).await.unwrap().1.idendereco.clone();

    assert!(deleta_endereco(id).await.is_ok());
}

#[tokio::test]
async fn test_cadastra_endereco_err(){
    let pais = format!("País Teste {}", "301");
    let estado = format!("Estado Teste {}",  "301");
    let cidade = "".to_string();
    let cep = format!("39606-{}",  "301");
    let bairro =  format!("Bairro Teste {}",  "301");
    let logradouro = format!("Logradouro Teste {}",  "301");
    let numero = format!("Numero Teste {}",  "301");
    let complemento = Some(format!("Complemento Teste {}",  "301"));

    let endereco = EnderecoInput{
        pais,
        estado,
        cidade,
        cep,
        bairro,
        logradouro,
        numero,
        complemento,
    };

    assert!(cadastra_endereco(Json(endereco)).await.is_err());
}

#[tokio::test]
async fn test_cadastra_enderecousuario_ok(){
    let usuario = usuario_padrao("302");
    let endereco = endereco_padrao("302");
    
    let idusuario = cadastra_usuario(Json(usuario)).await.unwrap().1.0.idusuario;
    let idendereco = cadastra_endereco(Json(endereco)).await.unwrap().1.idendereco.clone();

    let enderecousuario = EnderecoUsuarioInput{
        idendereco: idendereco.clone(),
        idusuario: idusuario.clone()
    };

    let id = cadastra_endereco_usuario(Json(enderecousuario)).await.unwrap().1.0;

    assert!(deleta_usuario(idusuario).await.is_ok());
    assert!(deleta_endereco(idendereco).await.is_ok());
    assert!(deleta_endereco_usuario(id).await.is_ok());
}

#[tokio::test]
async fn test_busca_endereco_id_ok(){
    let endereco = endereco_padrao("303");
    
    let id = cadastra_endereco(Json(endereco)).await.unwrap().1.idendereco.clone();
    
    assert!(busca_endereco_id(Query(id.clone())).await.is_ok());

    assert!(deleta_endereco(id).await.is_ok());
}

#[tokio::test]
async fn test_busca_enderecousuario_idusuario_ok(){
    let usuario = usuario_padrao("304");
    let endereco = endereco_padrao("304");
    
    let idusuario = cadastra_usuario(Json(usuario)).await.unwrap().1.0.idusuario;
    let idendereco = cadastra_endereco(Json(endereco)).await.unwrap().1.idendereco.clone();

    let enderecousuario = EnderecoUsuarioInput{
        idendereco: idendereco.clone(),
        idusuario: idusuario.clone()
    };

    let id = cadastra_endereco_usuario(Json(enderecousuario)).await.unwrap().1.0;

    assert!(busca_enderecousuario_idusuario(Query(idusuario.clone())).await.is_ok());

    assert!(deleta_usuario(idusuario).await.is_ok());
    assert!(deleta_endereco(idendereco).await.is_ok());
    assert!(deleta_endereco_usuario(id).await.is_ok());
}

#[tokio::test]
async fn test_busca_endereco_idusuario_ok(){
    let usuario = usuario_padrao("305");
    let endereco = endereco_padrao("305");
    
    let idusuario = cadastra_usuario(Json(usuario)).await.unwrap().1.0.idusuario;
    let idendereco = cadastra_endereco(Json(endereco)).await.unwrap().1.idendereco.clone();

    let enderecousuario = EnderecoUsuarioInput{
        idendereco: idendereco.clone(),
        idusuario: idusuario.clone()
    };

    let id = cadastra_endereco_usuario(Json(enderecousuario)).await.unwrap().1.0;

    assert!(busca_endereco_idusuario(Query(idusuario.clone())).await.is_ok());

    assert!(deleta_usuario(idusuario).await.is_ok());
    assert!(deleta_endereco(idendereco).await.is_ok());
    assert!(deleta_endereco_usuario(id).await.is_ok());
}