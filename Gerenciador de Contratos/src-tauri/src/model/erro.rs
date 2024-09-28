use::thiserror::Error;

#[derive(Error, Debug)]
pub enum MeuErro{
    #[error("CNPJ não encontrado")]
    CnpjNaoEncontrado,
    #[error("Máquina não encontrada")]
    MaquinaNaoEncontrada,
    #[error("Erro na conexão com o banco de dados: {0}")]
    ConexaoBanco(#[from] mysql_async::Error),
    #[error("Endereço não encontrado")]
    EnderecoNaoEncontrado,
    #[error("O E-mail está vazio")]
    EmailVazio, 
    #[error("O CNPJ está vazio")]
    CnpjVazio,
    #[error("Um ou mais campos estão vazios")]
    CamposVazios,
    #[error("ID não encontrado")]
    IdNaoEncontrado,
    #[error("Credenciais inválidas")]
    CredenciaisInvalidas,
    #[error("Não foi possível apagar o registro")]
    ImpossivelApagar
}