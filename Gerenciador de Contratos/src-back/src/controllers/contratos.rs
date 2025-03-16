use axum::{extract::Query, Json};
use chrono::NaiveDateTime;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::models::{self, contratos::Contrato, locatarios::busca_locatario_idlocatario};

use super::{contas_bancarias::busca_conta_bancaria_idusuario, cria_conn, enderecos::{busca_endereco_id, busca_endereco_idusuario}, formata_cnpj, formata_cpf, gera_hash, locadoras::busca_locadora_idusuario, locatarios::busca_locatario_idusuario, maquinas::busca_maquina_id, solicitacoes_contratos::busca_solicitacao_idsolicitacao, usuarios::{busca_usuario_id, formata_documento, IdInput, UserId}};

pub struct ContratoInput{
    pub idlocatario: String,
    pub idlocador: String,
    pub idenderecolocatario: String,
    pub idenderecolocador: String,
    pub idenderecoretirada: String,
    pub idmaquina: String,
    pub idsolicitacaocontrato: String,
}

pub async fn cadastra_contrato(input: Json<ContratoInput>)
    -> Result<(StatusCode, Json<String>), (StatusCode, Json<String>)>{
    if input.idenderecolocador.trim().is_empty() || input.idenderecolocatario.trim().is_empty()
      || input.idenderecoretirada.trim().is_empty() || input.idlocador.trim().is_empty()
      || input.idlocatario.trim().is_empty() || input.idmaquina.trim().is_empty()
      || input.idsolicitacaocontrato.trim().is_empty() {
      return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))    
    }

    let idcontrato = gera_hash(&input.idsolicitacaocontrato);
    let idsolicitacao = input.idsolicitacaocontrato.to_string();

    let solicitacao = busca_solicitacao_idsolicitacao(Query(IdInput{id: idsolicitacao})).await?.1.0;
    let medidatempo = solicitacao.medidatempolocacao;
    let prazolocacao = solicitacao.prazolocacao;
    let valorsolicitacao = solicitacao.valorsolicitacao;

    let enderecoretirada = busca_endereco_id(Query(input.idenderecoretirada.to_string())).await?.1.0;
    let cidadeforo = enderecoretirada.cidade;

    let contabancaria = busca_conta_bancaria_idusuario(Query(IdInput{id: input.idlocador.to_string()})).await?.1.0;
    let datacontrato = chrono::Utc::now().naive_utc();

    let contrato = Contrato{
        idcontrato,
        idlocatario: input.idlocatario.to_string(),
        idlocador: input.idlocador.to_string(),
        idenderecolocatario: input.idenderecolocatario.to_string(),
        idenderecolocador: input.idenderecolocador.to_string(),
        idenderecoretirada: input.idenderecoretirada.to_string(),
        idmaquina: input.idmaquina.to_string(),
        idsolicitacaocontrato: input.idsolicitacaocontrato.to_string(),
        prazolocacao: prazolocacao,
        medidatempolocacao: medidatempo,
        valorlocacao: valorsolicitacao,
        idcontabancarialocador: contabancaria.idconta,
        cidadeforo,
        datacontrato,
        statuscontrato: "Ativo".to_string(),
    };

    let conn = &mut cria_conn()?;
 
    match models::contratos::cadastra_contrato(conn, contrato).await{
      Ok(idcontrato) => {
        return Ok((StatusCode::OK, Json(idcontrato)))
      },
      Err(e) => {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
      }
    }  
}

#[derive(Serialize, Deserialize)]
pub struct ContratoPDF{
    pub idcontrato: String,

    pub nomelocatario: String,
    pub documentolocatario: String,
    pub tipodocumentolocatario: String,

    pub nomelocador: String,
    pub documentolocador: String,
    pub tipodocumentolocador: String,

    pub estadoenderecolocatario: String,
    pub cidadeenderecolocatario: String,
    pub cependerecolocatario: String,
    pub bairroenderecolocatario: String,
    pub logradouroenderecolocatario: String,
    pub numeroenderecolocatario: String,
    pub complementoenderecolocatario: String,

    pub estadoenderecolocador: String,
    pub cidadeenderecolocador: String,
    pub cependerecolocador: String,
    pub bairroenderecolocador: String,
    pub logradouroenderecolocador: String,
    pub numeroenderecolocador: String,
    pub complementoenderecolocador: String,

    pub estadoenderecoretirada: String,
    pub cidadeenderecoretirada: String,
    pub cependerecoretirada: String,
    pub bairroenderecoretirada: String,
    pub logradouroenderecoretirada: String,
    pub numeroenderecoretirada: String,
    pub complementoenderecoretirada: String,

    pub nomemaquina: String,
    pub numeroseriemaquina: String,
    pub valoraluguelmaquina: f64,

    pub numerocontabanco: String,
    pub numeroagenciabanco: String,
    pub nomebanco: String,

    pub medidatempolocacao: String,

    pub prazolocacao: f64,
    pub valorlocacao: f64,
    
    pub datacontrato: String,
}

pub async fn busca_contrato_id(Query(id): Query<IdInput>)
    -> Result<(StatusCode, Json<Contrato>), (StatusCode, Json<String>)>{
    if id.id.trim().is_empty(){
      return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))    
    }

    let id = id.id.trim().to_string();

    let conn = &mut cria_conn()?;

    match models::contratos::busca_contrato_id(conn, id).await{
      Ok(contrato) => {
        return Ok((StatusCode::OK, Json(contrato)))
      },
      Err(e) => {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
      }
    }
}

pub fn tipo_documento(documento_: &str) -> Result<String, (StatusCode, Json<String>)>{
  match formata_cpf(documento_){
      Ok(cpf) => {
          return Ok("CPF".to_string())
      },
      Err(_) => {
      }
  }
  match formata_cnpj(documento_){
      Ok(cnpj) => {
          return Ok("CNPJ".to_string())
      },
      Err(_) => {
          return Err((StatusCode::BAD_REQUEST, Json("O documento não é válido.".to_string())))
      }
  }
}

pub async fn gera_contrato_idsolicitacao(Query(id): Query<IdInput>)
    -> Result<(StatusCode, Json<ContratoPDF>), (StatusCode, Json<String>)>{
    if id.id.trim().is_empty(){
      return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))    
    }

    let id = id.id.trim().to_string();

    let conn = &mut cria_conn()?;

    let contrato = match models::contratos::busca_contrato_idsolicitacao(conn, id).await{
      Ok(contrato) => {
        contrato
      },
      Err(e) => {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
      }
    };

    let locatario = busca_usuario_id(Query(IdInput{id: contrato.idlocatario})).await?.1.0;
    let locador = busca_usuario_id(Query(IdInput{id: contrato.idlocador})).await?.1.0;

    let tipodocumentolocatario = tipo_documento(&locatario.documento)?;
    let tipodocumentolocador = tipo_documento(&locatario.documento)?;

    let enderecolocatario = busca_endereco_idusuario(Query(UserId{idusuario: locatario.idusuario})).await?.1.0;
    let enderecolocador = busca_endereco_idusuario(Query(UserId{idusuario: locador.idusuario.clone()})).await?.1.0;

    let maquina = busca_maquina_id(Query(IdInput{id: contrato.idmaquina})).await?.1.0;

    let contabancaria = busca_conta_bancaria_idusuario(Query(IdInput{id: locador.idusuario})).await?.1.0;

    let solicitacao = busca_solicitacao_idsolicitacao(Query(IdInput{id: contrato.idsolicitacaocontrato})).await?.1.0;

    let contratopdf = ContratoPDF{
        idcontrato: contrato.idcontrato,

        nomelocatario: locatario.nome,
        documentolocatario: locatario.documento,
        tipodocumentolocatario,

        nomelocador: locador.nome,
        documentolocador: locador.documento,
        tipodocumentolocador,

        estadoenderecolocatario: enderecolocatario.estado,
        cidadeenderecolocatario: enderecolocatario.cidade,
        cependerecolocatario: enderecolocatario.cep,
        bairroenderecolocatario: enderecolocatario.bairro,
        logradouroenderecolocatario: enderecolocatario.logradouro,
        numeroenderecolocatario: enderecolocatario.numero,
        complementoenderecolocatario: enderecolocatario.complemento,

        estadoenderecolocador: enderecolocador.estado.clone(),
        cidadeenderecolocador: enderecolocador.cidade.clone(),
        cependerecolocador: enderecolocador.cep.clone(),
        bairroenderecolocador: enderecolocador.bairro.clone(),
        logradouroenderecolocador: enderecolocador.logradouro.clone(),
        numeroenderecolocador: enderecolocador.numero.clone(),
        complementoenderecolocador: enderecolocador.complemento.clone(),

        estadoenderecoretirada: enderecolocador.estado,
        cidadeenderecoretirada: enderecolocador.cidade,
        cependerecoretirada: enderecolocador.cep,
        bairroenderecoretirada: enderecolocador.bairro,
        logradouroenderecoretirada: enderecolocador.logradouro,
        numeroenderecoretirada: enderecolocador.numero,
        complementoenderecoretirada: enderecolocador.complemento,

        nomemaquina: maquina.nome,
        numeroseriemaquina: maquina.numeroserie,
        valoraluguelmaquina: maquina.valoraluguel,

        numerocontabanco: contabancaria.numeroconta,
        numeroagenciabanco: contabancaria.numeroagencia,
        nomebanco: contabancaria.nomebanco,

        medidatempolocacao: solicitacao.medidatempolocacao,
        prazolocacao: solicitacao.prazolocacao,
        valorlocacao: solicitacao.valorsolicitacao,
        datacontrato: contrato.datacontrato.to_string(),
    };

    return Ok((StatusCode::OK, Json(contratopdf)))

}

pub async fn busca_contrato_idsolicitacao(Query(id): Query<IdInput>)
    -> Result<(StatusCode, Json<Contrato>), (StatusCode, Json<String>)>{
    if id.id.trim().is_empty(){
      return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }

    let id = id.id.trim().to_string();

    let conn = &mut cria_conn()?;

    match models::contratos::busca_contrato_idsolicitacao(conn, id).await{
      Ok(contrato) => {
        return Ok((StatusCode::OK, Json(contrato)))
      },
      Err(e) => {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
      }
    }
}