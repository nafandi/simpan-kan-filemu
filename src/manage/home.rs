use actix_web::{get, web};

use crate::shared_values::Info;

#[get("/")]
pub async fn home() -> web::Json<Info> {
    web::Json(Info {
        status: (200),
        info: "Simpan Kan Filemu".to_string(),
    })
}
