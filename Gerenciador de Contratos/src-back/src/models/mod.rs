use bigdecimal::BigDecimal;

pub mod usuarios;
pub mod codigos_recuperacao;
pub mod maquinas;

pub fn str_to_f64_bigdecimal(price_str: &str) -> f64 {
    let decimal = BigDecimal::parse_bytes(price_str.as_bytes(), 10).unwrap();
    let d = decimal.to_string();
    let decimal = d.parse().unwrap_or(0.0);
    decimal
}