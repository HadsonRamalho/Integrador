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
    ImpossivelApagar,
    #[error("CPF não encontrado")]
    CpfNaoEncontrado,
    #[error("Contrato não encontrado")]
    ContratoNaoEncontrado,
    #[error("E-mail não encontrado")]
    EmailNaoEncontrado,
    #[error("Senha incorreta")]
    SenhaIncorreta,
    #[error("Número de série não encontrado")]
    NumeroSerieNaoEncontrado,
    #[error("Não há maquinas em estoque")]
    SemMaquinasNoEstoque,
    #[error("Não há maquinas alugadas no momento")]
    SemMaquinasAlugadas,
    #[error("Máquina não encontrada no estoque")]
    MaquinaNaoEncontradaNoEstoque,
    #[error("Erro ao salvar o endereço")]
    ErroSalvarEndereco,
    #[error("Erro ao salvar o contrato")]
    ErroSalvarContrato
}