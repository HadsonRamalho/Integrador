use axum::Json;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct LocatarioInput{
  idusuario: String,
  idendereco: String
}

pub async fn cadastra_locatario(input: Json<LocatarioInput>){

}