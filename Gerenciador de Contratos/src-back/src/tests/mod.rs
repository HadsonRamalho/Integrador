pub mod usuarios;
pub mod codigos_recuperacao;
pub mod maquinas;

#[tokio::test]
async fn test_cria_conn_ok(){
    use crate::controllers::cria_conn;

    assert!(cria_conn().is_ok());
}