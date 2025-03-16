use bigdecimal::BigDecimal;

pub mod usuarios;
pub mod codigos_recuperacao;
pub mod maquinas;
pub mod imagens;
pub mod imagens_maquinas;
pub mod maquinas_usuarios;
pub mod enderecos;
pub mod enderecos_usuarios;
pub mod locatarios;
pub mod locadoras;
pub mod contas_bancarias;
pub mod solicitacoes_contratos;
pub mod contratos;
pub mod notificacoes;

pub fn str_to_f64_bigdecimal(price_str: &str) -> f64 {
    let decimal = BigDecimal::parse_bytes(price_str.as_bytes(), 10).unwrap();
    let d = decimal.to_string();
    let decimal = d.parse().unwrap_or(0.0);
    decimal
}