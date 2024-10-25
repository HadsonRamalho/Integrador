use axum::Json;
use mysql_async::{params, prelude::Queryable};

use crate::{controller::{cria_pool, socioadm::{cadastra_socio_adm, estrutura_socio_adm, SocioAdmInput}}, test::endereco::_limpa_endereco};

use super::endereco::cria_endereco_teste;

pub async fn cria_socio_teste(idendereco: &str, nome: &str, cpf: &str, orgaoemissor: &str, estadocivil: &str, nacionalidade: &str, cnpj: &str) -> String{
    let socioadm = SocioAdmInput{
        idendereco: idendereco.to_string(),
        nome: nome.to_string(),
        cpf: cpf.to_string(),
        orgaoemissor: orgaoemissor.to_string(),
        estadocivil: estadocivil.to_string(),
        nacionalidade: nacionalidade.to_string(),
        cnpj: cnpj.to_string(),
    };
    let socioadm = estrutura_socio_adm(Json(socioadm)).unwrap().1;
    let idsocioadm = cadastra_socio_adm(socioadm).await.unwrap();
    return idsocioadm
}

pub async fn _limpa_socio(idsocio: String) -> Result<(), String>{
    match deleta_socio(idsocio).await{
        Ok(_) => {
            return Ok(())
        },
        Err(e) => {
            println!("{:?}", e);
            return Err(e.to_string())
        }
    }
}

async fn deleta_socio(idsocio: String) -> Result<(), mysql_async::Error>{
    let pool = cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let resultado = conn.exec_drop("DELETE FROM socioadm WHERE idsocio = :idsocio;",
        params! {"idsocio" => idsocio}).await;
    match resultado{
        Ok(_) => {
            return Ok(())
        },
        Err(e) => {
            return Err(e)
        }
    }
}

#[tokio::test]
async fn test_cria_deleta_socio(){
    let logradouro = "Rua T";
    let cep = "39600-000";
    let complemento = "B";
    let numeroendereco = "123";
    let cidade = "Araçuaí";
    let uf = "MG";
    let idendereco = cria_endereco_teste(logradouro, cep, complemento, numeroendereco, cidade, uf).await.unwrap();

    let nome = "Socioadm".to_string();
    let cpf = "123.456.111-01".to_string();
    let orgaoemissor = "PCMG".to_string();
    let estadocivil = "Solteiro".to_string();
    let nacionalidade = "Brasileiro".to_string();
    let cnpj = "11.123.411/0001-01".to_string();

    let socio = SocioAdmInput{
        idendereco: idendereco.clone(),
        nome,
        cpf,
        orgaoemissor,
        estadocivil,
        nacionalidade,
        cnpj,
    };

    let socio = estrutura_socio_adm(Json(socio)).unwrap().1;
    let idsocio = cadastra_socio_adm(socio).await.unwrap();

    assert!(_limpa_socio(idsocio).await.is_ok());
    assert!(_limpa_endereco(idendereco).await.is_ok())
}