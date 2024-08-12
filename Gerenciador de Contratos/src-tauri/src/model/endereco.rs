use mysql_async::prelude::*;
use crate::model;
use crate::controller::endereco::Endereco;

pub async fn salva_endereco(endereco: serde_json::Value) -> Result<bool, mysql_async::Error> {
    let pool = model::create_pool().await?;
    let mut conn = pool.get_conn().await?;

    let endereco = Endereco {
        id: endereco["id"].as_str().unwrap_or("").to_string().split_off(15 as usize),
        logradouro: endereco["logradouro"].as_str().unwrap_or("").to_string(),
        cep: endereco["cep"].as_str().unwrap_or("").to_string(),
        complemento: endereco["complemento"].as_str().unwrap_or("").to_string(),
        numeroendereco: endereco["numeroendereco"].as_str().unwrap_or("").to_string(),
        cidade: endereco["cidade"].as_str().unwrap_or("").to_string(),
        uf: endereco["uf"].as_str().unwrap_or("").to_string(),
    };
    conn.exec_drop(
        "INSERT INTO endereco (idendereco, logradouro, cep, complemento, numeroendereco, cidade, uf) VALUES (?, ?, ?, ?, ?, ?, ?)",
        (
            endereco.id,
            endereco.logradouro,
            endereco.cep,
            endereco.complemento,
            endereco.numeroendereco,
            endereco.cidade,
            endereco.uf,
        ),
    ).await?;

    println!("Endereço salvo com sucesso");

    Ok(true)
}
