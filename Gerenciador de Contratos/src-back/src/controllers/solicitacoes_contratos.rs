use axum::{extract::Query, Json};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{controllers::{contratos::{cadastra_contrato, ContratoInput}, enderecos::busca_endereco_idusuario, notificacoes::{cadastra_notificacao, NotificacaoInput}, usuarios::UserId}, models::{self, solicitacoes_contratos::SolicitacaoContrato}};

use super::{cria_conn, gera_hash, usuarios::IdInput};

#[derive(Serialize, Deserialize)]
pub struct SolicitacaoContratoInput{
    pub idlocador: String,
    pub idlocatario: String,
    pub idmaquina: String,
    pub medidatempolocacao: String,
    pub origemsolicitacao: String,
    pub valorsolicitacao: f64,
    pub prazolocacao: f64,
}

pub async fn cadastra_solicitacao_contrato(input: Json<SolicitacaoContratoInput>)
    -> Result<(StatusCode, Json<String>), (StatusCode, Json<String>)>{
    if input.idlocador.trim().is_empty() || input.idlocatario.trim().is_empty() || 
      input.idmaquina.trim().is_empty() || input.origemsolicitacao.trim().is_empty()
      || input.prazolocacao.to_string().trim().is_empty() || input.medidatempolocacao.trim().is_empty(){
      return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }

    let idlocador = input.idlocador.trim().to_string();
    let idlocatario = input.idlocatario.trim().to_string();
    let idmaquina = input.idmaquina.trim().to_string();
    let origemsolicitacao = input.origemsolicitacao.trim().to_string();
    let statussolicitacao = "Aguardando aprovação".to_string();
    let medidatempolocacao = input.medidatempolocacao.to_string();
    let prazolocacao = input.prazolocacao;
    let valorsolicitacao = input.valorsolicitacao;

    let idsolicitacao = gera_hash(&idmaquina);
    let datasolicitacao = chrono::Utc::now().naive_utc();
    let solicitacao = SolicitacaoContrato{
      idsolicitacao,
      idlocador,
      idlocatario,
      idmaquina,
      medidatempolocacao,
      prazolocacao,
      origemsolicitacao,
      valorsolicitacao,
      statussolicitacao,
      datasolicitacao
    };

    if valorsolicitacao < 1.{
      return Err((StatusCode::BAD_REQUEST, Json("O valor do aluguel não é válido.".to_string())))
    }

    let conn = &mut cria_conn()?;
    let id = match models::solicitacoes_contratos::cadastra_solicitacao_contrato(conn, solicitacao.clone()).await{
      Ok(id) => {
        id
      },
      Err(e) => {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
      }
    };

    cadastra_notificacao(Json(NotificacaoInput{
      idusuario: solicitacao.idlocador.clone(),
      titulo: "Alguém solicitou uma máquina!".to_string(),
      mensagem: "Um cliente acabou de solicitar uma de suas máquinas! Clique aqui para ver suas solicitações.".to_string(),
      onclick: "/contract-request".to_string(),
    })).await?.1.0;

    cadastra_notificacao(Json(NotificacaoInput{
      idusuario: solicitacao.idlocatario.clone(),
      titulo: "Você solicitou uma máquina!".to_string(),
      mensagem: "Enviamos uma notificação para o dono da máquina que você solicitou. Ele receberá a notificação em breve, e poderá decidir se aprova ou recusa sua solicitação. Clique nessa mensagem para acompanhar suas solicitações.".to_string(),
      onclick: "/contract-request".to_string(),
    })).await?.1.0;

    return Ok((StatusCode::OK, Json(id)))
}

pub async fn busca_solicitacao_idsolicitacao(Query(id): Query<IdInput>)
    -> Result<(StatusCode, Json<SolicitacaoContrato>), (StatusCode, Json<String>)>{
    if id.id.trim().is_empty(){
      return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }
    let id = id.id.trim().to_string();

    let conn = &mut cria_conn()?;

    match models::solicitacoes_contratos::busca_solicitacao_idsolicitacao(conn, id).await{
      Ok(solicitacao) => {
        return Ok((StatusCode::OK, Json(solicitacao)))
      },
      Err(e) => {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
      }
    }
}

pub async fn busca_solicitacoes_idlocador(Query(id): Query<IdInput>)
    -> Result<(StatusCode, Json<Vec<SolicitacaoContrato>>), (StatusCode, Json<String>)>{
    if id.id.trim().is_empty(){
      return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }
    let id = id.id.trim().to_string();

    let conn = &mut cria_conn()?;

    let solicitacoes = match models::solicitacoes_contratos::busca_solicitacoes_idlocador(conn, id).await{
      Ok(solicitacoes) => {
        solicitacoes
      },
      Err(e) => {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
      }
    };

    if solicitacoes.is_empty(){
      return Err((StatusCode::INTERNAL_SERVER_ERROR, Json("Você não possui solicitações de contratos.".to_string())))
    }

    return Ok((StatusCode::OK, Json(solicitacoes)))
}

#[derive(Serialize, Deserialize)]
pub struct StatusSolicitacaoInput{
    pub id: String,
    pub status: String
}

pub async fn atualiza_status_solicitacao(input: Json<StatusSolicitacaoInput>)
    -> Result<(StatusCode, Json<SolicitacaoContrato>), (StatusCode, Json<String>)>{
    if input.id.trim().is_empty() || input.status.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }

    let conn = &mut cria_conn()?;

    let id = input.id.trim().to_string();
    let novostatus = input.status.to_string();
    let solicitacao = match models::solicitacoes_contratos::atualiza_status_solicitacao(conn, id, novostatus).await{
        Ok(solicitacao) => {
            solicitacao
        },
        Err(e) => {
          return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    };

    let idlocatario = busca_solicitacao_idsolicitacao(Query(IdInput { id: {input.id.clone()} })).await?.1.0.idlocatario;

    let idnotificacao = cadastra_notificacao(
      Json(NotificacaoInput { 
        idusuario: idlocatario, 
        titulo: "Sua solicitação de aluguel foi atualizada!".to_string(), 
        mensagem: format!("Uma locadora atualizou o status de sua solicitação para '{}'. Clique aqui para ver suas solicitações.", solicitacao.statussolicitacao.clone()), 
        onclick: ("/contract-request".to_string()) })
    ).await?.1.0;

    if solicitacao.statussolicitacao != "Solicitação aprovada"{
      return Ok((StatusCode::OK, Json(solicitacao)))
    }

    let idenderecolocatario = busca_endereco_idusuario(Query(UserId{idusuario: solicitacao.idlocatario.clone()})).await?.1.0.idendereco;
    let idenderecolocador = busca_endereco_idusuario(Query(UserId{idusuario: solicitacao.idlocador.clone()})).await?.1.0.idendereco;

    let idcontrato = cadastra_contrato(Json(ContratoInput{
        idlocatario: solicitacao.idlocatario.clone(),
        idlocador: solicitacao.idlocador.clone(),
        idenderecolocatario,
        idenderecolocador: idenderecolocador.clone(),
        idenderecoretirada: idenderecolocador,
        idmaquina: solicitacao.idmaquina.clone(),
        idsolicitacaocontrato: solicitacao.idsolicitacao.clone(),
    })).await?.1.0;

    println!("Contrato registrado!");

    return Ok((StatusCode::OK, Json(solicitacao)));
}

pub async fn busca_solicitacoes_idlocatario(Query(id): Query<IdInput>)
    -> Result<(StatusCode, Json<Vec<SolicitacaoContrato>>), (StatusCode, Json<String>)>{
    if id.id.trim().is_empty(){
      return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }
    let id = id.id.trim().to_string();

    let conn = &mut cria_conn()?;

    let solicitacoes = match models::solicitacoes_contratos::busca_solicitacoes_idlocatario(conn, id).await{
      Ok(solicitacoes) => {
        solicitacoes
      },
      Err(e) => {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
      }
    };

    if solicitacoes.is_empty(){
      return Err((StatusCode::INTERNAL_SERVER_ERROR, Json("Você não emitiu solicitações de contratos.".to_string())))
    }

    return Ok((StatusCode::OK, Json(solicitacoes)))
}