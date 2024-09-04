use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use mysql_async::params;
use mysql_async::prelude::*;
use mysql_async::Row;
use mysql_async::Value;

use crate::model::{self, contrato::Contrato};
use crate::controller;

#[tauri::command]
pub async fn filtra_contrato_nome_maquina(nome_maquina: String) -> Result<Vec<model::contrato::Contrato>, String>{
    let resultado_busca: Result<Vec<model::contrato::Contrato>, mysql_async::Error> = _filtra_contrato_nome_maquina(nome_maquina).await;

    match resultado_busca{
        Ok(resultado) => {
            if !resultado.is_empty(){
                return Ok(resultado);
            }
            return Err("Erro: Máquina não encontrada".to_string());
        },
        Err(erro) => {
            return Err(erro.to_string());
        }
    }
}

fn format_date(value: Value) -> String {
    match value {
        Value::Date(year, month, day, hour, minute, second, _microsecond) => {
            if hour == 0 && minute == 0 && second == 0 {
                // se o horário for 00:00:00, trata como Date
                NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32).unwrap()
                    .format("%Y-%m-%d")
                    .to_string()
            } else {
                // trata como DateTime
                NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32).unwrap(),
                    NaiveTime::from_hms_opt(hour as u32, minute as u32, second as u32).unwrap()
                )
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
            }
        },
        _ => "".to_string(),
    }
}

pub async fn _filtra_contrato_nome_maquina(nome_maquina: String) -> Result<Vec<Contrato>, mysql_async::Error> {
    let pool = controller::cria_pool().await.unwrap();
    let mut conn = pool.get_conn().await?;

    let rows: Vec<Row> = conn.exec(
        "SELECT idcontrato, prazolocacao, dataretirada, valormensal, vencimento,
                multaatraso, jurosatraso, avisotransferencia, prazodevolucao, cidadeforo,
                datacontrato, idlocatario, idlocador, idmaquina, enderecoretirada
         FROM contrato_aluguel WHERE avisotransferencia = :nome_maquina ORDER BY valormensal DESC",
        params! { "nome_maquina" => nome_maquina }
    ).await?;

    let contratos: Vec<Contrato> = rows.into_iter().map(|row| {
        let idcontrato = row.get::<String, _>("idcontrato").unwrap_or_default();
        let prazolocacao = row.get::<f32, _>("prazolocacao").unwrap_or_default();
        let dataretirada = format_date(row.get::<Value, _>("dataretirada").unwrap());
        let valormensal = row.get::<f32, _>("valormensal").unwrap_or_default();
        let vencimento = format_date(row.get::<Value, _>("vencimento").unwrap());
        let multaatraso = row.get::<f32, _>("multaatraso").unwrap_or_default();
        let jurosatraso = row.get::<f32, _>("jurosatraso").unwrap_or_default();
        let avisotransferencia = row.get::<String, _>("avisotransferencia").unwrap();
        let prazodevolucao = format_date(row.get::<Value, _>("prazodevolucao").unwrap());
        let cidadeforo = row.get::<String, _>("cidadeforo").unwrap_or_default();
        let datacontrato = format_date(row.get::<Value, _>("datacontrato").unwrap());
        let idlocatario = row.get::<String, _>("idlocatario").unwrap_or_default();
        let idlocador = row.get::<String, _>("idlocador").unwrap_or_default();
        let idmaquina = row.get::<String, _>("idmaquina").unwrap_or_default();
        let enderecoretirada = row.get::<String, _>("enderecoretirada").unwrap_or_default();

        Contrato {
            idcontrato,
            prazolocacao,
            dataretirada,
            valormensal,
            vencimento,
            multaatraso,
            jurosatraso,
            avisotransferencia,
            prazodevolucao,
            cidadeforo,
            datacontrato,
            idlocatario,
            idlocador,
            idmaquina,
            enderecoretirada,
        }
    }).collect();

    Ok(contratos)
}