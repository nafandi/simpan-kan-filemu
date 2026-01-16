use crate::shared_values::{AppState, FileForm};
use actix_web::{get, web, web::Data};
#[get("/list")]
pub async fn list(db: Data<AppState>) -> web::Json<Vec<FileForm>> {
    let list: Vec<FileForm> = sqlx::query_as("SELECT * from files")
        .fetch_all(&db.db)
        .await
        .unwrap();
    web::Json(list)
}
