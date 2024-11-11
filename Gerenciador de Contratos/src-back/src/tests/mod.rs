#[cfg(test)]
mod usuarios;
#[tokio::test]
async fn test_cria_conn_ok(){
    use crate::controllers::cria_conn;

    assert!(cria_conn().is_ok());
}