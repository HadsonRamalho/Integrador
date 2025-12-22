pub mod codigos_recuperacao;
pub mod contas_bancarias;
pub mod enderecos;
pub mod enderecos_usuarios;
pub mod locadoras;
pub mod locatarios;
pub mod maquinas;
pub mod usuarios;

#[tokio::test]
async fn test_cria_conn_ok() {
    use crate::controllers::cria_conn;

    assert!(cria_conn().is_ok());
}
