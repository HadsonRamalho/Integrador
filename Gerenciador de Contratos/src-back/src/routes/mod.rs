use axum::{
    http::Method, routing::{get, get_service, patch, post}, Router
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use utoipa::openapi::Contact;
use crate::controllers::{self, codigos_recuperacao::{envia_codigo_recuperacao, verifica_codigo_recuperacao}, imagens_maquinas::{cadastra_imagem_maquina, recupera_imagem_maquina}, maquinas::{cadastra_maquina, lista_todas_maquinas}, multipart::cadastra_imagem, usuarios::{self, atualiza_email_usuario, atualiza_senha_usuario, busca_email_usuario, busca_senha_usuario, busca_usuario_id, cadastra_usuario, realiza_login}};
use crate::controllers::usuarios::busca_usuario_email;
use crate::routes::usuarios::{__path_realiza_login, __path_cadastra_usuario, __path_busca_usuario_email};
use crate::controllers::codigos_recuperacao::__path_envia_codigo_recuperacao;
use utoipa_axum::{router, routes};
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;
pub fn cria_rotas() -> Router<>{
    
    let (router, mut api) = OpenApiRouter::<()>::new()
    
        .routes(routes!(realiza_login))
        .routes(routes!(cadastra_usuario))
        .routes(routes!(envia_codigo_recuperacao))
        .routes(routes!(busca_usuario_email))
        .split_for_parts();

    api.info.description = Some("\nDocumentação para as rotas da API do sistema MaqExpress.\n
        Em caso de dúvidas ou erros, contate os desenvolvedores do back-end. ;) dadso".to_string());
    
    let mut contato = Contact::new();
    contato.email = Some("hadsonramalho@gmail.com".to_string());
    contato.name = Some("Hadson Ramalho".to_string());
    contato.url = Some("https://github.com/HadsonRamalho".to_string());
    api.info.contact = Some(contato);
    api.info.license = None;
    api.info.title = "MaqExpress".to_string();
    api.info.version = "1.1.22".to_string();
    api.info.extensions = None;
    api.info.terms_of_service = None;
    api.external_docs = None;
    
    let app: Router<_> = Router::new()
        .route("/cadastra_usuario", post(cadastra_usuario))
        .route("/busca_email_usuario", get(busca_email_usuario))
        .route("/realiza_login", post(realiza_login))
        .route("/busca_senha_usuario", post(busca_senha_usuario))
        .route("/atualiza_senha_usuario", patch(atualiza_senha_usuario))
        .route("/atualiza_email_usuario", patch(atualiza_email_usuario))
        .route("/busca_usuario_email/{email}", get(busca_usuario_email))
        .route("/busca_usuario_id", get(busca_usuario_id))

        .route("/verifica_codigo_recuperacao", post(verifica_codigo_recuperacao))
        .route("/envia_codigo_recuperacao", post(envia_codigo_recuperacao))

        .route("/cadastra_maquina", post(cadastra_maquina))
        .route("/lista_todas_maquinas", get(lista_todas_maquinas))

        .route("/cadastra_imagem", post(cadastra_imagem))
        .route("/cadastra_imagem_maquina", post(cadastra_imagem_maquina))
        .route("/recupera_imagem_maquina", post(recupera_imagem_maquina))

        .nest_service("/images", get_service(ServeDir::new("./images")))

        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(vec![Method::POST, Method::PUT, Method::PATCH]) 
                .allow_headers(Any),
        )
        
        .merge(SwaggerUi::new("/swagger-ui").url("/api/openapi.json", api));
    app
}
