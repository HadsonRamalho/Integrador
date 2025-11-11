use reqwest::Response;

use super::usuarios::{CredenciaisUsuario, UserId};

pub async fn obter_userid(res: Result<Response, reqwest::Error>)    
    -> Result<UserId, String>{
    let res = match res{
        Ok(res) => {
            res
        },
        Err(e) => {
            return Err(e.to_string())
        }
    };
    match res.json::<UserId>().await{
        Ok(res) => {
            return Ok(res)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}
pub async fn testa_api()
    -> Result<UserId, String>{
    let email = "email";
    let senha = "senha";
    let url = "https://g6v9psc0-3003.brs.devtunnels.ms/realiza_login";
    let client = reqwest::Client::new();
    let res: Result<Response, reqwest::Error> = client.post(url)
    .json(&CredenciaisUsuario{
        email: email.to_string(),
        senha: senha.to_string()
    }).send().await;    
    let id = obter_userid(res).await?;
    Ok(id)
}