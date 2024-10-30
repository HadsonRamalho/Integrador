use mysql_async::{params, prelude::Queryable};

use crate::{controller::{cria_pool, locatario::{cadastra_locatario, estrutura_locatario, LocatarioInput}, socioadm::estrutura_socio_adm}, test::{endereco::{_limpa_endereco, cria_endereco_teste}, socioadm::{_limpa_socio, cria_socio_teste}}};
#[cfg(test)]
use crate::model::locatario::Locatario;

pub async fn cria_locatario_teste(nomelocatario: &str, idendereco: &str, cnpj: &str, idsocio: &str, locatariostatus: i16) -> Result<String, String>{
    use axum::Json;
    use crate::{controller::locatario::{cadastra_locatario, estrutura_locatario, LocatarioInput}, model::locatario::Locatario};

    let locatario = LocatarioInput{
        nomelocatario: nomelocatario.to_string(),
        idendereco: idendereco.to_string(),
        cnpj: cnpj.to_string(),
        idsocio: idsocio.to_string(),
        locatariostatus
    };
    let resultado = estrutura_locatario(Json(locatario));
    let idlocatario;
    let locatario = match resultado{
        Ok(locatario) => {
            idlocatario = locatario.idlocatario.clone();
            locatario
        },
        Err(e) => {
            return Err(e)
        }
    };
    assert!(cadastra_locatario(locatario).await.is_ok());
    Ok(idlocatario)
}

#[tokio::test]
async fn cria_deleta_locatario(){
    let nomelocatario = "Locatario";
    let cnpj = "11.411.411/0004-01";

    let logradouro = "Rua T";
    let cep = "39600-000";
    let complemento = "A";
    let numeroendereco = "123";
    let cidade = "Araçuaí";
    let uf = "MG";
    let idendereco = cria_endereco_teste(logradouro, cep, complemento, numeroendereco, cidade, uf).await.unwrap();
    let idenderecosocio = cria_endereco_teste(logradouro, cep, complemento, numeroendereco, cidade, uf).await.unwrap();

    let nome = "SocioTeste";
    let cpf = "114.145.123-01";
    let orgaoemissor = "PCMG";
    let estadocivil = "Solteiro";
    let nacionalidade = "Brasileiro";
    let idsocio = cria_socio_teste(&idenderecosocio, nome, cpf, orgaoemissor, estadocivil, nacionalidade, cnpj).await;

    let idlocatario = cria_locatario_teste(nomelocatario, &idendereco, cnpj, &idsocio, 1).await.unwrap();

    assert!(_limpa_locatario(idlocatario).await.is_ok());
    assert!(_limpa_socio(idsocio).await.is_ok());
    assert!(_limpa_endereco(idendereco).await.is_ok());    
    assert!(_limpa_endereco(idenderecosocio).await.is_ok());    
}

pub async fn _limpa_locatario(idlocatario: String) -> Result<(), String>{
    match deleta_locatario(idlocatario).await{
        Ok(_) => {
            return Ok(())
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

async fn deleta_locatario(idlocatario: String) -> Result<(), mysql_async::Error>{
    let pool = cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let resultado = conn.exec_drop("DELETE FROM locatario WHERE idlocatario = :idlocatario;", 
        params! {"idlocatario" => idlocatario}).await;
    match resultado{
        Ok(_) =>{
            return Ok(())
        }, 
        Err(e) => {
            return Err(e)
        }
    }
}