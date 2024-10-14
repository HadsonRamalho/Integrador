use crate::model;
use crate::model::erro::MeuErro;
use axum::http::StatusCode;
use axum::Json;
use pwhash::bcrypt;
use pwhash::unix;
pub mod endereco;
pub mod locadora;
pub mod locatario;
pub mod maquina;
pub mod socioadm;
pub mod usuario;
pub mod contrato;

/// ## Recebe um e-mail e verifica se ele está formatado corretamente
/// Chama a função responsável pela validação, passando o e-mail sem espaços como argumento.
/// ```
/// if !valida_email(email.trim())
/// ```
//#[tauri::command]
pub async fn checa_email(input: Json<String>) -> Result<(), (StatusCode, String)> {
    let email = input.0;
    if !valida_email(email.trim()) {
        return Err((StatusCode::BAD_REQUEST, "E-mail inválido. Deve conter '@' e '.'".to_string()));
    }
    return Ok(());
}

/// ## Cria uma pool de conexões com o banco de dados, retorna um `mysql_async::Pool` ou um `mysql_async::Error`, dependendo do sucesso da operação.
/// Chama a função do model que é responsável por carregar os dados da conexão para o sistema, 
/// retornando a `pool` caso a criação seja bem-sucedida:
/// ```
/// let pool = model::create_pool().await;
/// ```
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

/// ## Recebe um e-mail e verifica se ele é válido ao checar sua formatação, retornando false caso esteja incorreta.
/// Remove os espaços em branco do e-mail e verifica se ele possui os caracteres '@' e '.', retornando true caso possua:
/// ```
/// let email: String = email.chars().filter(|c| !c.is_whitespace()).collect(); 
/// if email.contains("@") && email.contains(".") && !email.is_empty() {
///     return true
/// }
/// ```
pub fn valida_email(email: &str) -> bool {
    let email: String = email.chars().filter(|c| !c.is_whitespace()).collect(); 
    if email.contains("@") && email.contains(".") && !email.is_empty() {
        return true
    }
    return false
}

/// ## Recebe um e-mail e verifica se ele é válido e se está cadastrado no banco. 
/// ## Caso ambas as verificações sejam verdadeiras, chama a função que faz o envio do e-mail de recuperação de senha.
/// Primeiro, faz a validação do e-mail ao chamar a função responsável por isso:
/// ```
/// if !valida_email(email) {
///        return Err("E-mail inválido. Deve conter '@' e '.'".to_string());
/// }
/// ```
/// Cria uma pool de conexões e passa essa pool como argumento para a função que busca o e-mail no banco de dados:
/// ```
/// let pool = match cria_pool().await
/// [...]
/// let _consome_result: Result<String, mysql_async::Error> = model::busca_email(&pool, email).await;
/// ```
/// Se o registro existir no banco de dados, a função de envio é chamada e um código de verificação é mandado para o e-mail do usuário:
/// ```
/// Ok(_) => {
///     let codigo = model::envia_email(_consome_result.unwrap());
///     return Ok(codigo);
/// }
/// ```
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
    let _consome_result: Result<String, mysql_async::Error> = model::busca_email(&pool, email).await;
    match _consome_result {
        Ok(_) => {
            let codigo = model::envia_email(_consome_result.unwrap());
            return Ok(codigo);
        }
        _ => return Err("Erro: O e-mail não é válido ou pode não estar cadastrado.".to_string()),
    }
}

/// ## Recebe um e-mail e retorna um token/hash que é gerado a partir da string slice
#[tauri::command]
pub async fn gera_token(email: &str) -> Result<String, ()> {
    let token = gera_hash(email);
    Ok(token)
}

/// ## Recebe uma string slice e gera um hash a partir de seu conteúdo
pub fn gera_hash(senha: &str) -> String {
    let enc = bcrypt::hash(senha).unwrap();
    return enc;
}

/// ## Recebe uma string slice e uma String, verifica se o conteúdo da string slice é o correto para verificar o hash da String
pub fn verifica_hash(senha_digitada: &str, hash: String) -> bool {
    let dec = unix::verify(senha_digitada, &hash);
    return dec;
}

/// ## Recebe o código digitado pelo usuário e o código armazenado no banco de dados, verifica se são iguais e retorna uma String caso sejam
/// Primeiro, verifica se `codigo_usuario` é uma String vazia, retornando erro caso seja:
/// ```
/// if codigo_usuario.trim().is_empty(){
///     return Err("Erro: Preencha o código.".to_string())
// }
/// ```
/// Se `codigo_usuario` e `codigo_banco` forem iguais, retorna `Ok`:
/// ```
/// let eq = codigo_banco == codigo_usuario;
/// if eq{
///    return Ok("Codigo correto".to_string())
/// }
/// ```
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

/// ## Recebe duas senhas, compara se são iguais e verifica se ambas são fortes o suficiente
#[tauri::command]
pub async fn compara_novas_senhas(senha1: String, senha2:String) -> Result<String, String>{
    if senha1.trim().is_empty() || senha2.trim().is_empty(){
        return Err(MeuErro::CamposVazios.to_string())
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

/// ## Recebe um CEP com ou sem formatação, faz a formatação e retorna o resultado como String
pub fn formata_cep(cep: &str) -> Result<String, String>{
    let cep: Vec<char> = cep
        .chars()
        .filter(|c: &char| c.is_digit(10))
        .collect();
    if cep.len() != 8{
        return Err("Erro: CEP de tamanho inválido.".to_string())
    }
    let mut cep: Vec<char> = cep;
    cep.insert(5, '-');
    let mut cepfinal: String = "".to_string();
    for u in cep{
        cepfinal.push(u);
    }
    return Ok(cepfinal);
}

/// ## Recebe um CPF com ou sem formatação, faz a formatação e retorna o resultado como String
pub fn formata_cpf(cpf: &str) -> Result<String, String>{
    let cpf: Vec<char> = cpf
        .chars()
        .filter(|c: &char| c.is_digit(10))
        .collect();
    if cpf.len() != 11{
        return Err("Erro: CPF de tamanho inválido.".to_string())
    }
    let mut cpf: Vec<char> = cpf;
    cpf.insert(3, '.');
    cpf.insert(7, '.');
    cpf.insert(11, '-');
    let mut cpffinal: String = "".to_string();
    for u in cpf{
        cpffinal.push(u);
    }
    return Ok(cpffinal);
}