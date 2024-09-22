// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

// Útil para salvar arquivos localmente
/*
use std::fs::File;
use std::io::{self, Read, Write};
use bincode::serialize;
use serde::{Deserialize, Serialize};
use bincode::deserialize;
*/

// Relacionado ao banco de dados
use std::env;

mod controller;
mod model;

/// Função principal que inicia a aplicação Tauri.
///
/// Esta função configura o manipulador de invocação do Tauri, registrando as funções disponíveis para chamadas do front-end.
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            controller::usuario::busca_nome_usuario,
            controller::usuario::busca_id,
            controller::usuario::verifica_token,
            controller::usuario::atualiza_senha,
            controller::usuario::atualiza_email,
            controller::usuario::busca_email_usuario,
            controller::usuario::busca_cnpj_usuario,

            controller::locatario::busca_id_locatario,
            controller::locatario::cadastra_locatario,
            controller::locatario::estrutura_locatario,
            controller::locatario::busca_locatario_nome,
            controller::locatario::busca_locatario_cnpj,

            controller::locadora::cadastra_locadora,
            controller::locadora::busca_id_locadora,
            controller::locadora::estrutura_locadora,
            controller::locadora::locadora_existente,

            controller::maquina::estrutura_maquina,
            controller::maquina::busca_maquina_nome,
            controller::maquina::cadastra_maquina,
            controller::maquina::busca_maquina_numserie,
            controller::maquina::gera_estoque_por_nome,
            controller::maquina::gera_estoque_total,

            controller::socioadm::estrutura_socio_adm,
            controller::socioadm::cadastra_socio_adm,            

            controller::endereco::estrutura_endereco,
            controller::endereco::_salva_endereco,
            controller::endereco::busca_endereco_id,

            controller::contrato::busca_contrato_nome_maquina,
            controller::contrato::estrutura_contrato,
            controller::contrato::cadastra_contrato,

            controller::cria_conta,
            controller::verifica_senha,
            controller::checa_email,
            controller::encontra_email_smtp,
            controller::gera_token,
            controller::verifica_codigo_email,
            controller::compara_novas_senhas,
        ]) // Registra funções do Tauri
        .run(tauri::generate_context!())
        .expect("erro ao tentar executar a aplicação Tauri");
}
