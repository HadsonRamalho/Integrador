use axum::{body::Bytes, http::StatusCode, Json};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::{controllers::cria_conn, models::imagens::atualiza_link_imagem};

#[derive(TryFromMultipart)]
pub struct Img {
    #[form_data(limit = "16MiB")]
    pub file: FieldData<Bytes>,
}

#[derive(Serialize, Deserialize)]
pub struct ImgOutput {
    pub idimagem: String,
    pub link: String,
}

pub async fn cadastra_imagem(data: TypedMultipart<Img>) 
    -> Result<(StatusCode, Json<ImgOutput>), (StatusCode, Json<String>)> {
    
    let nomearquivo = data.file.metadata.file_name.clone().unwrap_or_else(|| "file_name_undefined".to_string());
    println!("Recebendo arquivo: {}", nomearquivo);

    println!("Tamanho do arquivo: {} bytes", data.file.contents.len());

    // Converte os bytes para Vec<u8>
    let bin = data.file.contents.to_vec();

    let conn = &mut cria_conn()?;

    // Insere os dados da imagem no banco de dados
    match crate::models::imagens::cadastra_imagem(conn, nomearquivo.clone()).await {
        Ok(idimagem) => {
            let nome_hash = format!("{}_{}", &idimagem[..20], nomearquivo);
            let file_path = format!("./images/{}", &nome_hash);
            println!("Salvando imagem em: {}", file_path);

            if let Err(e) = fs::write(&file_path, &data.file.contents).await {
                eprintln!("Erro ao salvar o arquivo: {}", e);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(format!("Erro ao salvar o arquivo: {}", e)),
                ));
            }

            // Atualiza o link da imagem no banco de dados
            let link = format!("/images/{}", &nome_hash);
            if let Err(e) = atualiza_link_imagem(conn, idimagem.clone(), link.clone()).await {
                eprintln!("Erro ao atualizar o link da imagem: {}", e);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(format!("Erro ao atualizar o link no banco: {}", e)),
                ));
            }
            
            Ok((
                StatusCode::OK,
                Json(ImgOutput {
                    idimagem,
                    link,
                }),
            ))
        }
        Err(e) => {
            eprintln!("Erro ao inserir a imagem no banco de dados: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(format!("Erro ao inserir a imagem no banco: {}", e)),
            ))
        }
    }
}
