#[cfg(test)]

use crate::controller::cria_pool;

mod usuario;

#[tokio::test]
async fn test_criacao_pool_ok(){
    let pool = cria_pool().await;
    assert!(pool.is_ok());
}