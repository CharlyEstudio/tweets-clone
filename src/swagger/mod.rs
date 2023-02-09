use actix_web::{HttpResponse, Responder};

pub async fn doc() -> impl Responder {
  HttpResponse::Ok().body("Documentacion")
}