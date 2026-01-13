use actix_web::{HttpResponse, Responder, get};

#[get("/")]
pub async fn home() -> impl Responder {
    HttpResponse::Ok().body("Simpan Kan Filemu")
}
