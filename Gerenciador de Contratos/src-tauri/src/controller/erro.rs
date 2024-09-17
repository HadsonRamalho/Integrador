use::thiserror::Error;

#[derive(Error, Debug)]
pub enum MeuErro{
    #[error("CNPJ não encontrado")]
    CnpjNaoEncontrado,
    #[error("Erro na conexão com o banco de dados: {0}")]
    ConexaoBanco(#[from] mysql_async::Error),
}