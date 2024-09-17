use crate::model;
use crate::model::Usuario;
use locadora::formata_cnpj;
use pwhash::bcrypt;
use pwhash::unix;
pub mod endereco;
pub mod locadora;
pub mod locatario;
pub mod maquina;
pub mod socioadm;
pub mod usuario;
pub mod contrato;
pub mod erro;

#[tauri::command]
pub async fn cria_conta(
    nome_completo: &str,
    email: &str,
    senha1: &str,
    senha2: &str,
    cpf: &str,
    cnpj: &str
) -> Result<(), String> {
    let email = email.trim(); // Removendo todos os espaços em branco do email
    if !valida_email(&email) {
        return Err("E-mail inválido. Deve conter '@' e '.'".to_string());
    }
    if senha1.trim() != senha2.trim() {
        return Err("As senhas são diferentes".to_string()); // Conta não criada
    }
    let cnpj = match formata_cnpj(cnpj){
        Ok(cnpj) => {
            cnpj
        },
        Err(e) => {
            return Err(e)
        }
    };
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
    let resultado_cadastro = save_data(nome_completo, &email, usuario.get_hash(), cpf, &cnpj).await;
    match resultado_cadastro {
        Ok(_) => return Ok(()),
        Err(_) => return Err("Erro no cadastro do usuário.".to_string()),
    }
}

#[tauri::command]
pub fn checa_email(email: &str) -> Result<(), String> {
    if !valida_email(email.trim()) {
        return Err("E-mail inválido. Deve conter '@' e '.'".to_string());
    }
    return Ok(());
}

#[tauri::command]
pub async fn verifica_senha(email: &str, senha: &str) -> Result<(), String> {
    // Retorna uma mensagem para o front e um booleano
    let senha = senha.trim();
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

pub async fn save_data(nome: &str, email: &str, senha: &str, cpf: &str, cnpj: &str) -> Result<(), String> {
    let pool = match cria_pool().await {
        Ok(pool) => {
            pool
        }, 
        Err(e) =>{
            return Err(e.to_string())
        }
    };
    let _resultado_criacao = model::save_data(&pool, nome, &email, senha, cpf, cnpj)
        .await
        .map_err(|e| format!("{}", e))?;
    Ok(())
}

pub async fn cria_pool() -> Result<mysql_async::Pool, mysql_async::Error> {
    let pool = model::create_pool().await;
    match pool{
        Ok(pool) =>{
            return Ok(pool)
        },
        Err(e) => {
            return Err(e)
        }
    }
}

pub async fn _verifica_senha(email: &str, senha: &str) -> Result<Usuario, String> {
    let pool = match cria_pool().await {
        Ok(pool) => {
            pool
        }, 
        Err(e) =>{
            return Err(e.to_string())
        }
    };
    let usuario_autenticado = model::verifica_senha(&pool, &email, senha)
        .await
        .map_err(|e| format!("{}", e))?;
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
pub async fn encontra_email_smtp(email: &str) -> Result<String, String> {
    if !valida_email(email) {
        return Err("E-mail inválido. Deve conter '@' e '.'".to_string());
    }
    let pool = match cria_pool().await {
        Ok(pool) => {
            pool
        }, 
        Err(e) =>{
            return Err(e.to_string())
        }
    };
    let _consome_result = model::busca_email(&pool, email).await;
    match _consome_result {
        Ok(_) => {
            let codigo = model::envia_email(_consome_result.unwrap());
            return Ok(codigo);
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

#[tauri::command]
pub async fn verifica_codigo_email(codigo_usuario: String, codigo_banco: String) -> Result<String, String> {
    if codigo_usuario.trim().is_empty(){
        return Err("Erro: Preencha o código.".to_string())
    }
    let eq = codigo_banco == codigo_usuario;
    if eq{
        return Ok("Codigo correto".to_string())
    }
    return Err("Erro: Codigo incorreto".to_string())
}

#[tauri::command]
pub async fn compara_novas_senhas(senha1: String, senha2:String) -> Result<String, String>{
    if senha1.trim().is_empty() || senha2.trim().is_empty(){
        return Err("Erro: Preencha todos os campos.".to_string())
    }
    if senha1 != senha2 {
        return Err("Erro: As senhas são diferentes".to_string())
    }
    match usuario::valida_senha(&senha1){
        Ok(_) => {

        },
        Err(e) => {
            return Err(e);
        }
    }
    match usuario::valida_senha(&senha2){
        Ok(_) => {

        },
        Err(e) => {
            return Err(e);
        }
    }
    return Ok("Senhas válidas!".to_string())
}

pub fn formata_cep(cep: &str) -> Result<String, String>{
    let cep: Vec<char> = cep
        .chars()
        .filter(|c: &char| c.is_digit(10))
        .collect();
    if cep.len() != 8{
        return Err("Erro: CEP de tamanho inválido.".to_string())
    }
    let mut cep: Vec<char> = cep;
    cep.insert(6, '-');
    let mut cepfinal: String = "".to_string();
    for u in cep{
        cepfinal.push(u);
    }
    return Ok(cepfinal);
}