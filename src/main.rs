mod api;

use std::future::Future;
use std::io::Bytes;
use std::pin::Pin;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, dev, guard};
use actix_web::guard::GuardContext;
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa_swagger_ui::SwaggerUi;

use utoipa::{Modify, ToSchema};

use utoipa::OpenApi;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use crate::api::example_api;


async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(OpenApi)]
#[openapi(
    paths(example_api::get_pet_by_id, example_api::echo, example_api::hello),
    components(schemas(SomeResponse, example_api::Pet)),
    modifiers(&SecurityAddon),
    security(("bearer" = []))
)]
struct ApiDoc;

#[derive(ToSchema)]
struct SomeResponse {
    pub id: i32
}

struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build()
            )
        )
    }
}

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    data: serde_json::Value, // To hold any response data
    is_success: bool,
}

pub fn verify_token(ctx: &GuardContext) -> bool {
    let auth_header = ctx.head().headers().get("authorization");
    if auth_header.is_none() {
        HttpResponse::Unauthorized().json(json!({"error" : "Acesso negado"}));
        return false;
    } else {
        return true;
    }
}

pub fn user_scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/pets")
                    .guard(guard::fn_guard(verify_token))
                               .service(example_api::get_pet_by_id),
                    );
}
pub async fn handle_unauthorized() -> HttpResponse {
    HttpResponse::Unauthorized().json(json!({"error": "Unauthorized"}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()

            .service(example_api::hello)
            .service(example_api::echo)
            .configure(user_scoped_config)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .route("/hey", web::get().to(manual_hello))
            .default_service(web::route().to(handle_unauthorized))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
