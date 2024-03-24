use actix_web::{get, post, HttpResponse, Responder, HttpRequest, web};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Pet found successfully", body = Pet),
        (status = NOT_FOUND, description = "Pet was not found")
    )
)]
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[utoipa::path(
    post,
    path = "/echo",
    responses(
        (status = 200, description = "Pet found successfully", body = Pet),
        (status = NOT_FOUND, description = "Pet was not found")
    ),
    params(
        ("req_body" = String, Path, description = "Pet database id to get Pet for"),
    )
)]
#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

use utoipa::{ToResponse, ToSchema};
use utoipa::openapi::{RefOr, Response};
use serde::Serialize;
use proc_macro::ApiResponseResponder;
use shared::ApiResponse;

#[derive(ToSchema, Serialize, ApiResponseResponder)]
pub struct Pet {
    id: u64,
    name: String,
    age: Option<i32>,
}

#[utoipa::path(
    get,
    path = "/pets/by_id/{id}",
    responses(
        (status = 200, description = "Pet found successfully", body = Pet),
        (status = NOT_FOUND, description = "Pet was not found")
    ),
    params(
        ("id" = u64, Path, description = "Pet database id to get Pet for"),
    )
)]
#[get("/by_id/{id}")]
async fn get_pet_by_id(pet_id: web::Path<u64>) -> Pet {
    Pet {
        id: pet_id.into_inner(),
        age: None,
        name: "lightning".to_string(),
    }
}
