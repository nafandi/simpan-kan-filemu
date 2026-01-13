use crate::shared_values::AppState;
use actix_multipart::Multipart;
use actix_web::{
    HttpResponse, post,
    web::{self},
};
use futures_util::StreamExt as _;
use tokio::io::AsyncWriteExt;
#[post("/upload")]
pub async fn upload(
    db: web::Data<AppState>,
    mut payload: Multipart,
) -> actix_web::Result<HttpResponse> {
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let mut file =
            tokio::fs::File::create(format!("{}/{}", db.folder, &field.name().unwrap())).await?;
        let file_name = &field.name().unwrap();
        let _ = sqlx::query!("INSERT into files (file) VALUES ($1)", file_name)
            .fetch_one(&db.db)
            .await;
        while let Some(chunk) = field.next().await {
            let chunk = chunk?;
            let _ = file.write_all(&chunk).await?;
        }
    }
    Ok(HttpResponse::Ok().body("Upload success"))
}
