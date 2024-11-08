use axum::{
    routing::{delete, get, post, put},
    Router,
    http::Method
};
use tower_http::cors::{Any, CorsLayer};


use crate::controller::{
    checa_email, compara_novas_senhas, contrato::{busca_contrato_cnpj_locatario, busca_contrato_nome_locatario, busca_contrato_nome_maquina, busca_contrato_numserie_maquina, busca_contratos_a_vencer, cadastra_contrato, estrutura_contrato}, encontra_email_smtp, endereco::{
        _salva_endereco, busca_endereco_id
    }, gera_token, locadora::{busca_id_locadora, cadastra_locadora, estrutura_locadora, locadora_existente, verifica_usuario_socio_locadora}, locatario::{busca_id_locatario, busca_locatario_cnpj, busca_locatario_nome, cadastra_locatario, estrutura_locatario}, maquina::{busca_maquina_nome, busca_maquina_numserie, cadastra_maquina, estrutura_maquina, gera_estoque_por_nome, gera_estoque_total, gera_estoque_total_alugadas}, socioadm::{busca_socio_adm_cpf, busca_socio_adm_id, cadastra_socio_adm, estrutura_socio_adm}, usuario::{
        atualiza_email, atualiza_nome, atualiza_senha, busca_cnpj_usuario, busca_email_usuario, busca_id, busca_nome_usuario, cria_conta, deleta_conta, verifica_senha, verifica_token
    }, verifica_codigo_email
};


pub fn cria_rotas() -> Router<>{
    let app: Router<_> = Router::new()
        .route("/verifica_token", post(verifica_token))
        .route("/atualiza_senha", put(atualiza_senha))
        .route("/atualiza_email", put(atualiza_email))
        .route("/cria_conta", post(cria_conta))
        .route("/verifica_senha", post(verifica_senha))
        .route("/busca_id", post(busca_id))
        .route("/atualiza_nome", put(atualiza_nome))
        .route("/busca_email_usuario", get(busca_email_usuario))
        .route("/busca_nome_usuario", get(busca_nome_usuario))
        .route("/busca_cnpj_usuario", get(busca_cnpj_usuario))
        .route("/deleta_conta", delete(deleta_conta))

        .route("/checa_email", post(checa_email))
        .route("/gera_token", post(gera_token))
        .route("/encontra_email_smtp", post(encontra_email_smtp))
        .route("/verifica_codigo_email", post(verifica_codigo_email))
        .route("/compara_novas_senhas", post(compara_novas_senhas))

        .route("/_salva_endereco", post(_salva_endereco))
        .route("/busca_endereco_id", get(busca_endereco_id))

        .route("/estrutura_socio_adm", post(estrutura_socio_adm))
        .route("/cadastra_socio_adm", post(cadastra_socio_adm))
        .route("/busca_socio_adm_id", get(busca_socio_adm_id))
        .route("/busca_socio_adm_cpf", get(busca_socio_adm_cpf))

        .route("/estrutura_maquina", post(estrutura_maquina))
        .route("/cadastra_maquina", post(cadastra_maquina))
        .route("/busca_maquina_nome", get(busca_maquina_nome))
        .route("/busca_maquina_numserie", get(busca_maquina_numserie))
        .route("/gera_estoque_por_nome", post(gera_estoque_por_nome))
        .route("/gera_estoque_total", post(gera_estoque_total))
        .route("/gera_estoque_total_alugadas", post(gera_estoque_total_alugadas))

        .route("/estrutura_locatario", post(estrutura_locatario))
        .route("/cadastra_locatario", post(cadastra_locatario))
        .route("/busca_id_locatario", get(busca_id_locatario))
        .route("/busca_locatario_nome", get(busca_locatario_nome))
        .route("/busca_locatario_cnpj", get(busca_locatario_cnpj))

        .route("/estrutura_locadora", post(estrutura_locadora))
        .route("/cadastra_locadora", post(cadastra_locadora))
        .route("/busca_id_locadora", post(busca_id_locadora))
        .route("/locadora_existente", post(locadora_existente))
        .route("/verifica_usuario_socio_locadora", post(verifica_usuario_socio_locadora))

        .route("/busca_contrato_nome_maquina", post(busca_contrato_nome_maquina))
        .route("/estrutura_contrato", post(estrutura_contrato))
        .route("/cadastra_contrato", post(cadastra_contrato))
        .route("/busca_contratos_a_vencer", post(busca_contratos_a_vencer))
        .route("/busca_contrato_numserie_maquina", post(busca_contrato_numserie_maquina))
        .route("/busca_contrato_nome_locatario", post(busca_contrato_nome_locatario))
        .route("/busca_contrato_cnpj_locatario", post(busca_contrato_cnpj_locatario))

        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(vec![Method::POST, Method::PUT]) 
                .allow_headers(Any),
        );
    app
}