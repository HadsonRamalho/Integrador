pub mod usuarios;
pub mod codigos_recuperacao;
pub mod maquinas;
pub mod enderecos_usuarios;
pub mod enderecos;
pub mod locatarios;
pub mod contas_bancarias;
pub mod locadoras;

#[tokio::test]
async fn test_cria_conn_ok(){
    use crate::controllers::cria_conn;

    assert!(cria_conn().is_ok());
}