use crate::model;
use pwhash::bcrypt;
use pwhash::unix;

/// Função para criar uma conta de usuário.
///
/// # Parâmetros
/// - nome_completo: Nome completo do usuário.
/// - email: Endereço de email do usuário.
/// - senha1: Primeira senha digitada.
/// - senha2: Segunda senha digitada para confirmação.
///
/// # Retornos
/// - Result<bool, bool>: Retorna Ok(true) se a conta for criada com sucesso, 
///   Ok(false) se houver um erro na criação da conta
#[tauri::command] 
pub async fn cria_conta(nomeCompleto: &str, email: &str, senha1: &str, senha2: &str) -> Result<bool, bool> { 
    let email:String = email.chars().filter(|c| !c.is_whitespace()).collect(); // Removendo todos os espaços em branco do email
    let validacao_email = valida_email(&email);
    if validacao_email == false{
        return Ok(false); // Conta não criada
    }
    if senha1 != senha2 {
        return Ok(false); // Conta não criada
    }
    let hash = enc_senha(senha1); // Criptografando a senha (Standard *BSD hash)
    let mut email_repetido:u32 = 0;
    let mut usuario = model::Usuario::novo_usuario(
        nomeCompleto.to_string(),
        email.to_string(), 
        hash); // Cria um novo usuário

    let _consome_result = save_data(
        nomeCompleto, 
        &email, 
        usuario.get_hash(), 
        &mut email_repetido).await;
    if email_repetido == 0{   
        return Ok(true); // Conta criada   
    }
    return Ok(false); // Conta não foi criada
}

/// Função para verificar o email inserido
///
/// # Parâmetros
/// - email: Endereço de email do usuário.
///
/// # Retornos
/// - Result<bool, bool>: Retorna Ok(true) se: o email não está vazio E possui os caracteres '@' e '.'
///   Ok(false) se o email não cumprir algum dos critérios de validação
#[tauri::command]
pub fn login_email(email: &str) -> Result<bool, bool> { // Retorna uma mensagem de sucesso ou falha para o front
    let vazio = ""; // String vazia a ser comparada caso a verificação no front falhe
    if email == vazio{
        return Err(false)
    }
    if valida_email(email){
        return Ok(true)
    }
    return Ok(false)
}

/// Função para realizar login com email e senha.
///
/// # Parâmetros
/// - email: Endereço de email do usuário.
/// - senha: Senha do usuário.
///
/// # Retornos
/// - Result<bool, bool>: Retorna Ok(true) se o login for bem-sucedido,
///   Ok(false) se a senha estiver vazia ou o login não for bem-sucedido.
#[tauri::command]
pub async fn login_senha(email: &str, senha: &str) -> Result<bool, bool>{ // Retorna uma mensagem para o front e um booleano
    let vazio = ""; 
    if senha == vazio{ // Verificação caso o campo do front falhe
        return Ok(false)
    }
    let mut senha_correta:u32 = 0;
    let _x = _verifica_senha(email, senha, &mut senha_correta).await;
    if senha_correta != 0 {
        return Ok(true)
    } else{
        return Ok(false)
    }
    
}

/// Função para salvar dados do usuário no banco de dados.
///
/// # Parâmetros
/// - nome: Nome do usuário.
/// - email: Endereço de email do usuário.
/// - senha: Senha do usuário.
/// - email_repetido: Referência mutável para o contador de emails repetidos.
///
/// # Retornos
/// - Result<u32, String>: Retorna Ok(email_repetido) se a operação for bem-sucedida,
///   Err(erro) se ocorrer um erro ao salvar os dados.
pub async fn save_data(nome: &str, email: &str, senha: &str, email_repetido: &mut u32) -> Result<u32, String> {
    let pool = model::create_pool().await.map_err(|e| format!("{}", e))?;
    model::save_data(
        &pool, 
        nome, 
        &email, 
        senha, 
        email_repetido).await.map_err(|e| format!("{}", e))?; // Usa o arquivo db.rs para salvar dados no banco
    Ok(*(email_repetido))
}

/// Função para verificar a senha do usuário.
///
/// # Parâmetros
/// - email: Endereço de email do usuário.
/// - senha: Senha do usuário.
/// - senha_correta: Referência mutável para o contador de senhas corretas.
///
/// # Retornos
/// - Result<u32, String>: Retorna Ok(senha_correta) se a operação for bem-sucedida,
///   Err(erro) se ocorrer um erro ao verificar a senha.
pub async fn _verifica_senha(email: &str, senha: &str, senha_correta: &mut u32) -> Result<u32, String> { // Parâmetros devem ser alterados conforme a necessidade posterior
    let pool = model::create_pool().await.map_err(|e| format!("{}", e))?;
    model::verifica_senha(&pool, &email, senha,senha_correta).await.map_err(|e| format!("{}", e))?; // Usa o arquivo db.rs para salvar dados no banco
    Ok(*(senha_correta))
}

/// Função para validar o formato do endereço de email.
///
/// # Parâmetros
/// - email: Endereço de email a ser validado.
///
/// # Retorno
/// - bool: Retorna true se o email tiver o formato válido (contém '@' e '.'), caso contrário, retorna false.
pub fn valida_email(email: &str) -> bool{
    let mut verificador = false;
    if email.contains("@") && email.contains(".") {
        verificador = true;
    }
    verificador
}

/// Função para criptografar uma senha usando o algoritmo bcrypt.
///
/// # Parâmetros
/// - senha: Senha a ser criptografada.
///
/// # Retorno
/// - String: Retorna a senha criptografada.
pub fn enc_senha(senha: &str) -> String{
    let enc = bcrypt::hash(senha).unwrap();
    return enc
}

/// Função para verificar se uma senha digitada corresponde a um hash de senha.
///
/// # Parâmetros
/// - senha_digitada: Senha digitada pelo usuário.
/// - hash: Hash da senha armazenada no banco de dados.
///
/// # Retorno
/// - bool: Retorna true se a senha digitada corresponder ao hash fornecido, caso contrário, retorna false.
pub fn dec_senha(senha_digitada: &str, hash: String) -> bool{
    let dec = unix::verify(senha_digitada, &hash);
    return dec
}