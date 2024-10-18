#[cfg(test)]

async fn cria_usuario(nome: String, email: String, senha: String, cpf: String, cnpj: String){
    use crate::test::usuario::cria_usuario_teste;
    assert!(cria_usuario_teste(&nome, &email, &senha, &cpf, &cnpj).await.is_ok());
}

