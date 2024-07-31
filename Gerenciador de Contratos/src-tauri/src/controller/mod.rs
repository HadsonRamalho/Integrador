use crate::model;
use crate::model::Usuario;
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
pub async fn cria_conta(nome_completo: &str, email: &str, senha1: &str, senha2: &str) -> Result<bool, bool> { 
    let email:String = email.chars().filter(|c| !c.is_whitespace()).collect(); // Removendo todos os espaços em branco do email
    let validacao_email = valida_email(&email);
    if validacao_email == false{
        return Ok(false); // Conta não criada
    }
    if senha1 != senha2 {
        return Ok(false); // Conta não criada
    }
    let hash = enc_senha(senha1); // Criptografando a senha (Standard *BSD hash)
    let mut usuario = model::Usuario::novo_usuario(
        nome_completo.to_string(),
        email.to_string(), 
        hash); // Cria um novo usuário

    let _consome_result = save_data(
        nome_completo, 
        &email, 
        usuario.get_hash()
        ).await;
    if _consome_result.unwrap(){   
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
pub fn checa_email(email: &str) -> Result<bool, bool> { // Retorna um bool para o front, representando sucesso na validação do e-mail
    if !valida_email(email){
        return Ok(false)
    }
    return Ok(true)
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
pub async fn login_senha(email: &str, senha: &str) -> Result<bool, String>{ // Retorna uma mensagem para o front e um booleano
    let senha:String = senha.chars().filter(|c| !c.is_whitespace()).collect(); // Removendo todos os espaços em branco da senha
    if senha.is_empty(){ // Verificação caso o campo do front falhe
        return Ok(false)
    }
    let mut senha_correta:u32 = 0;
    let resultado_verificacao: Result<Usuario, String> = _verifica_senha(email, &senha, &mut senha_correta).await;
    let mut _usuario_autenticado= Default::default();
    match resultado_verificacao{
        Ok(_) => {
            _usuario_autenticado = resultado_verificacao.unwrap();
        },
        _ => {
            let erro =  resultado_verificacao.unwrap_err();
            println!("{erro}");
            let separacao = erro.find(")");
            let (_primeira_parte, segunda_parte) = erro.split_at(separacao.unwrap()+2);
            return Err(segunda_parte.to_string())
        }
    }
    let usuario_autenticado = _usuario_autenticado.get_all();
    println!("{}, {}, {}", usuario_autenticado.0, usuario_autenticado.1, usuario_autenticado.2);
    if senha_correta != 0 {
        return Ok(true)
    }
    return Err("Senha inválida".to_string())
    
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
pub async fn save_data(nome: &str, email: &str, senha: &str) -> Result<bool, String> {
    let pool = model::create_pool().await.map_err(|e| format!("{}", e))?;
    let resultado_criacao = model::save_data(
        &pool, 
        nome, 
        &email, 
        senha
        ).await.map_err(|e| format!("{}", e))?; // Usa o arquivo db.rs para salvar dados no banco
    Ok(resultado_criacao)
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
pub async fn _verifica_senha(email: &str, senha: &str, senha_correta: &mut u32) -> Result<Usuario, String> { // Parâmetros devem ser alterados conforme a necessidade posterior
    let pool = model::create_pool().await.map_err(|e| format!("{}", e))?;
    let usuario_autenticado = model::verifica_senha(&pool, &email, senha,senha_correta).await.map_err(|e| format!("{}", e))?; // Usa o arquivo db.rs para salvar dados no banco
    Ok(usuario_autenticado)
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
    let email:String = email.chars().filter(|c| !c.is_whitespace()).collect(); // Removendo todos os espaços em branco do email
    if email.contains("@") && email.contains(".") && !email.is_empty() {
        verificador = true;
    }
    verificador
}

/// Função Tauri para verificar a existência de um e-mail no banco de dados e enviar um e-mail de verificação se existir.
///
/// # Parâmetros
/// - email: Endereço de e-mail a ser verificado.
///
/// # Retornos
/// - Result<bool, bool>: Retorna Ok(true) se o e-mail for encontrado e o e-mail de verificação enviado com sucesso,
///   Ok(false) se o e-mail não for encontrado.
#[tauri::command]
pub async fn encontra_email_smtp(email: &str) -> Result<bool, bool>{
    if !valida_email(email){
        return Ok(false)
    }
    let pool = model::create_pool().await.map_err(|e| format!("{}", e)).unwrap();
    let _consome_result = model::busca_email(&pool, email).await;
    match _consome_result{
        Ok(_) => {
            model::envia_email(_consome_result.unwrap());
            return Ok(true)
        },
        _ => return Ok(false)
    }
}


// !!!! Funções a serem implementadas posteriormente !!!!
#[tauri::command]
pub async fn _altera_email(email: &str) -> Result<bool, bool>{
    let email = email.trim();
    let pool = model::create_pool().await.map_err(|e| format!("{}", e)).unwrap();
    let _consome_result = model::busca_email(&pool, email).await;
    if email.is_empty() || !valida_email(email) || _consome_result.unwrap() == ""{
        return Ok(false)
    }
    // chamada à função no model
    Ok(true)
}
// !!!!

#[tauri::command]
pub async fn gera_token(email: &str) -> Result<String, ()>{
    let token = enc_senha(email);
    Ok(token)
}

/// Função para criptografar uma senha usando o algoritmo bcrypt.
///
/// # Parâmetros
/// - senha: Senha a ser criptografada.
///
/// # Retorno
/// - String: Retorna o hash da senha criptografada.
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