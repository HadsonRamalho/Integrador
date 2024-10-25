#[cfg(test)]

use crate::controller::cria_pool;
use crate::controller::{formata_cpf, locadora::formata_cnpj};

mod usuario;
mod e2e;
mod endereco;
mod maquina;
mod locatario;
mod socioadm;
mod locadora;

#[tokio::test]
async fn test_criacao_pool_ok(){
    let pool = cria_pool().await;
    assert!(pool.is_ok());
}

#[tokio::test]
async fn test_formata_cpf(){
    let cpf = "12345678901";
    assert_eq!(formata_cpf(cpf), Ok("123.456.789-01".to_string()));
    let cpf = "123456789-01";
    assert_eq!(formata_cpf(cpf), Ok("123.456.789-01".to_string()));
    let cpf = "123.45678901";
    assert_eq!(formata_cpf(cpf), Ok("123.456.789-01".to_string()));
    let cpf = "123.456.789-0000";
    assert!(formata_cpf(cpf).is_err());
    let cpf = "123.456.789";
    assert!(formata_cpf(cpf).is_err());
}

#[tokio::test]
async fn test_formata_cnpj(){
    let cnpj = "11123456000101";
    assert_eq!(formata_cnpj(cnpj), Ok("11.123.456/0001-01".to_string()));
    let cnpj = "11123456/000101";
    assert_eq!(formata_cnpj(cnpj), Ok("11.123.456/0001-01".to_string()));
    let cnpj = "11123456/0001-01";
    assert_eq!(formata_cnpj(cnpj), Ok("11.123.456/0001-01".to_string()));
    let cnpj = "11.123.456000101";
    assert_eq!(formata_cnpj(cnpj), Ok("11.123.456/0001-01".to_string()));
    let cnpj = "11%123A456a0001p01";
    assert_eq!(formata_cnpj(cnpj), Ok("11.123.456/0001-01".to_string()));
    let cnpj = "11.123.456/0001-010000";
    assert!(formata_cnpj(cnpj).is_err());
    let cnpj = "11.123.456/0001";
    assert!(formata_cnpj(cnpj).is_err());
}