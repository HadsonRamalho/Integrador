use axum::{http::StatusCode, Json};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CodigoRecuperacaoInput{
    pub codigodigitado: String
}

pub async fn verifica_codigo_recuperacao(input: Json<CodigoRecuperacaoInput>)
    -> Result<(), (StatusCode, Json<String>)>{
    let codigodigitado = input.codigodigitado.to_string();
    if codigodigitado.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("O código não pode estar vazio.".to_string())))
    }

    // buscar um código de verificação no banco
    let exemplos = ["1234", "0000", "1111", "3333", "0169"];
    let exemplos = exemplos.to_vec();
    for codigo in exemplos{
        if codigo == codigodigitado{
            return Ok(())
        }
    }
    return Err((StatusCode::BAD_REQUEST, Json("Código não encontrado.".to_string())))
}