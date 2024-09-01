use crate::model;
use crate::model::Usuario;
use pwhash::bcrypt;
use pwhash::unix;
pub mod endereco;
pub mod locadora;
pub mod locatario;
pub mod maquina;
pub mod socioadm;
pub mod usuario;

#[tauri::command]
pub async fn cria_conta(
    nome_completo: &str,
    email: &str,
    senha1: &str,
    senha2: &str,
) -> Result<(), String> {
    let email: String = email.chars().filter(|c| !c.is_whitespace()).collect(); // Removendo todos os espaços em branco do email
    if !valida_email(&email) {
        return Err("E-mail inválido. Deve conter '@' e '.'".to_string());
    }
    if senha1 != senha2 {
        return Err("As senhas são diferentes".to_string()); // Conta não criada
    }
    match usuario::valida_senha(senha1) {
        Ok(_) => {}
        Err(e) => {
            return Err(e);
        }
    }
    let hash = gera_hash(senha1); // Criptografando a senha (Standard *BSD hash)
    let mut usuario =
        model::Usuario::novo_usuario(nome_completo.to_string(), email.to_string(), hash); // Cria um novo usuário
    if usuario.ja_cadastrado().await {
        return Err("Usuário já cadastrado".to_string());
    }
    let resultado_cadastro = save_data(nome_completo, &email, usuario.get_hash()).await;
    match resultado_cadastro {
        Ok(_) => return Ok(()),
        Err(_) => return Err("Erro no cadastro do usuário.".to_string()),
    }
}

#[tauri::command]
pub fn checa_email(email: &str) -> Result<(), String> {
    if !valida_email(email) {
        return Err("E-mail inválido. Deve conter '@' e '.'".to_string());
    }
    return Ok(());
}

#[tauri::command]
pub async fn realiza_login(email: &str, senha: &str) -> Result<(), String> {
    // Retorna uma mensagem para o front e um booleano
    let senha: String = senha.chars().filter(|c| !c.is_whitespace()).collect(); // Removendo todos os espaços em branco da senha
    if senha.is_empty() {
        // Verificação caso o campo do front falhe
        return Err("A senha não pode estar vazia".to_string());
    }
    let resultado_verificacao: Result<Usuario, String> = _verifica_senha(email, &senha).await;
    match resultado_verificacao {
        Ok(_) => return Ok(()),
        Err(e) => {
            return Err(e.to_string());
        }
    }
}

pub async fn save_data(nome: &str, email: &str, senha: &str) -> Result<(), String> {
    let pool = model::create_pool().await.map_err(|e| format!("{}", e))?;
    let _resultado_criacao = model::save_data(&pool, nome, &email, senha)
        .await
        .map_err(|e| format!("{}", e))?;
    Ok(())
}

pub async fn cria_pool() -> Result<mysql_async::Pool, String> {
    let pool = model::create_pool().await.map_err(|e| format!("{}", e))?;
    Ok(pool)
}

pub async fn _verifica_senha(email: &str, senha: &str) -> Result<Usuario, String> {
    // Parâmetros devem ser alterados conforme a necessidade posterior
    let pool = model::create_pool().await.map_err(|e| format!("{}", e))?;
    let usuario_autenticado = model::verifica_senha(&pool, &email, senha)
        .await
        .map_err(|e| format!("{}", e))?; // Usa o arquivo db.rs para salvar dados no banco
    Ok(usuario_autenticado)
}

pub fn valida_email(email: &str) -> bool {
    let mut verificador = false;
    let email: String = email.chars().filter(|c| !c.is_whitespace()).collect(); // Removendo todos os espaços em branco do email
    if email.contains("@") && email.contains(".") && !email.is_empty() {
        verificador = true;
    }
    verificador
}

#[tauri::command]
pub async fn encontra_email_smtp(email: &str) -> Result<(), String> {
    if !valida_email(email) {
        return Err("E-mail inválido. Deve conter '@' e '.'".to_string());
    }
    let pool = model::create_pool()
        .await
        .map_err(|e| format!("{}", e))
        .unwrap();
    let _consome_result = model::busca_email(&pool, email).await;
    match _consome_result {
        Ok(_) => {
            model::envia_email(_consome_result.unwrap());
            return Ok(());
        }
        _ => return Err("Erro: O e-mail não é válido ou pode não estar cadastrado.".to_string()),
    }
}

#[tauri::command]
pub async fn gera_token(email: &str) -> Result<String, ()> {
    let token = gera_hash(email);
    Ok(token)
}

pub fn gera_hash(senha: &str) -> String {
    let enc = bcrypt::hash(senha).unwrap();
    return enc;
}

pub fn verifica_hash(senha_digitada: &str, hash: String) -> bool {
    let dec = unix::verify(senha_digitada, &hash);
    return dec;
}
