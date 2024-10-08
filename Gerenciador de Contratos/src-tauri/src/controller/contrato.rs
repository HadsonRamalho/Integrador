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
pub async fn filtra_contrato_nome_maquina(nome_maquina: String, idusuario: String) -> Result<Vec<model::contrato::Contrato>, String>{
    let pool = match controller::cria_pool().await {
        Ok(pool) => {
            pool
        }, 
        Err(e) =>{
            return Err(e.to_string())
        }
    };
    let cnpj = controller::usuario::_busca_cnpj_usuario(&pool, &idusuario).await;
    let cnpj = match cnpj{
        Ok(cnpj) => {
            cnpj
        }, Err(_) => {
            return Err("Erro: O usuário não tem um CNPJ cadastrado.".to_string())
        }
    };
    let resultado_busca: Result<Vec<model::contrato::Contrato>, mysql_async::Error> = _filtra_contrato_nome_maquina(nome_maquina, cnpj).await;

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

fn formata_data(value: Value) -> String {
    match value {
        Value::Date(ano, mes, dia, hora, minuto, segundo, microsegundo) => {
            if hora == 0 && minuto == 0 && segundo == 0 {
                // se o horário for 00:00:00, trata como Date
                let data = NaiveDate::from_ymd_opt(ano as i32, mes as u32, dia as u32).unwrap()
                    .format("%Y-%m-%d")
                    .to_string();
                return data;
            }
            // trata como DateTime
            let data_precisa = NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(ano as i32, mes as u32, dia as u32).unwrap(),
                    NaiveTime::from_hms_opt(hora as u32, minuto as u32, segundo as u32).unwrap()
                )
                .format("%Y-%m-%d %H:%M:%S")
                .to_string();
            return data_precisa;
            
        },
        _ => "".to_string(),
    }
}

pub async fn _filtra_contrato_nome_maquina(nome_maquina: String, cnpj: String) -> Result<Vec<Contrato>, mysql_async::Error> {
    let pool = match controller::cria_pool().await {
        Ok(pool) => {
            pool
        }, 
        Err(e) =>{
            return Err(e)
        }
    };
    let mut conn = pool.get_conn().await?;
    let cnpj = cnpj.trim();


    let rows: Vec<Row> = conn.exec(
        "SELECT ca.idcontrato, ca.prazolocacao, ca.dataretirada, ca.valormensal, ca.vencimento,
       ca.multaatraso, ca.jurosatraso, ca.avisotransferencia, ca.prazodevolucao, 
       ca.cidadeforo, ca.datacontrato, ca.idlocatario, ca.idlocador, ca.idmaquina, 
       ca.enderecoretirada
        FROM contrato_aluguel ca
        JOIN locadora ld ON ca.idlocador = ld.idlocadora
        JOIN maquina ma ON ca.idmaquina = ma.idmaquina
        WHERE ma.nomemaquina = :nome_maquina AND ld.cnpj = :cnpj
        ORDER BY ca.valormensal DESC;",
        params! { "nome_maquina" => nome_maquina, "cnpj" => cnpj }
    ).await?;

    let contratos: Vec<Contrato> = rows.into_iter().map(|row| {
        let idcontrato = row.get::<String, _>("idcontrato").unwrap_or_default();
        let prazolocacao = row.get::<f32, _>("prazolocacao").unwrap_or_default();
        let dataretirada = formata_data(row.get::<Value, _>("dataretirada").unwrap());
        let valormensal = row.get::<f32, _>("valormensal").unwrap_or_default();
        let vencimento = formata_data(row.get::<Value, _>("vencimento").unwrap());
        let multaatraso = row.get::<f32, _>("multaatraso").unwrap_or_default();
        let jurosatraso = row.get::<f32, _>("jurosatraso").unwrap_or_default();
        let avisotransferencia = row.get::<String, _>("avisotransferencia").unwrap();
        let prazodevolucao = formata_data(row.get::<Value, _>("prazodevolucao").unwrap());
        let cidadeforo = row.get::<String, _>("cidadeforo").unwrap_or_default();
        let datacontrato = formata_data(row.get::<Value, _>("datacontrato").unwrap());
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