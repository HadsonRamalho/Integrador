#[cfg(test)]
mod tests {
    use crate::controller::{self, cria_pool};

    use super::*;
    use controller::usuario::_busca_nome_usuario;
    use mysql_async::Pool;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_criacao_pool_ok(){
        let mut pool = cria_pool().await;
        assert!(pool.is_ok());
    }

    #[tokio::test]
    async fn test_busca_nome_usuario_ok() {
        let mut mock_pool = match cria_pool().await{
            Ok(pool) => {pool},
            Err(e) => {
                println!("{:?}", e);
                return;
            }
        };
        let nome_esperado = "Usuario 1".to_string();
        let resultado_busca = _busca_nome_usuario(&mock_pool, "$2b$10$/2s6.dhOc8KzUm0o7YfAr.jfkEtcXxRB/Y6rSVSuqHZCrz1dc9l5q").await;
        assert_eq!(resultado_busca.unwrap(), nome_esperado);
    }

    #[tokio::test]
    async fn test_busca_nome_usuario_id_invalido() {
        let mut mock_pool = match cria_pool().await{
            Ok(pool) => {pool},
            Err(e) => {
                println!("{:?}", e);
                return;
            }
        };
        let resultado_busca = _busca_nome_usuario(&mock_pool, "$2b$10$").await;
        assert!(resultado_busca.is_err());
    }
}
