use axum::Json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CodigoRecuperacaoInput{
    pub codigodigitado: String
}

pub async fn verifica_codigos_recuperacao(input: Json<CodigoRecuperacaoInput>)
    -> Result<(), String>{
    let codigodigitado = input.codigodigitado.to_string();

    // buscar um código de verificação no banco
    let exemplos = ["1234", "0000", "1111", "3333", "0169"];
    let exemplos = exemplos.to_vec();
    for codigo in exemplos{
        if codigo == codigodigitado{
            return Ok(())
        }
    }
    return Err("Código não encontrado.".to_string())
}