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
    #[error("O endereço não foi salvo")]
    SalvarEndereco,
    #[error("O contrato não foi salvo")]
    SalvarContrato,
    #[error("O nome do usuário não foi atualizado")]
    AtualizarNomeUsuario,
    #[error("A senha do usuário não foi atualizada")]
    AtualizarSenhaUsuario,
    #[error("O e-mail do usuário não foi atualizado")]
    AtualizarEmailUsuario,
    #[error("O usuário não foi salvo")]
    SalvarUsuario,
    #[error("O sócio não foi salvo")]
    SalvarSocio,
    #[error("O hash não foi encontrado")]
    HashNaoEncontrado,
    #[error("Não foi encontrada uma máquina com esse nome")]
    NomeMaquinaNaoEncontrado,
    #[error("A máquina não foi salva")]
    SalvarMaquina,
    #[error("Não foi encontrado um locatário com esse nome")]
    NomeLocatarioNaoEncontrado,
    #[error("O locatário não foi salvo")]
    SalvarLocatario,
    #[error("Não foi encontrada uma locadora com esse CNPJ")]
    CnpjLocadoraNaoEncontrado,
    #[error("A locadora não foi salva")]
    SalvarLocadora
}