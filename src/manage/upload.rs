use crate::shared_values::{AppState, Info};
use actix_multipart::Multipart;
use actix_web::{
    post,
    web::{self},
};
use futures_util::StreamExt as _;
use tokio::io::AsyncWriteExt;
#[post("/upload")]
pub async fn upload(db: web::Data<AppState>, mut payload: Multipart) -> web::Json<Info> {
    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();
        let mut file = tokio::fs::File::create(format!("{}/{}", db.folder, &field.name().unwrap()))
            .await
            .unwrap();
        let file_name = &field.name().unwrap();
        let _ = sqlx::query!("INSERT into files (file) VALUES ($1)", file_name)
            .fetch_one(&db.db)
            .await;
        while let Some(chunk) = field.next().await {
            let chunk = chunk.unwrap();
            let _ = file.write_all(&chunk).await.unwrap();
        }
    }
    web::Json(Info {
        status: (200),
        info: ("Upload Success".to_string()),
    })
}
