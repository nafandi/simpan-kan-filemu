use crate::shared_values::{AppState, FileForm};
use actix_web::{
    HttpResponse, Responder, post,
    web::{self, Data},
};
use tokio::fs::rename as tokio_rename;
#[post("/rename")]
pub async fn rename(form: web::Form<FileForm>, db: Data<AppState>) -> impl Responder {
    let get_file = match sqlx::query!("SELECT file FROM files WHERE id=$1", form.id)
        .fetch_one(&db.db)
        .await
    {
        Ok(get_file) => match get_file.file {
            Some(file) => file,
            None => {
                return HttpResponse::Ok()
                    .body("Error getting file from id, you might typed wrong id");
            }
        },
        Err(_) => {
            return HttpResponse::Ok().body("Error getting file from id, you might typed wrong id");
        }
    };
    let _ = sqlx::query!("UPDATE files SET file=$1 WHERE id=$2", form.file, form.id)
        .execute(&db.db)
        .await;
    let process = match tokio_rename(
        format!("{}/{}", db.folder, get_file),
        format!("{}/{}", db.folder, form.file),
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().body("File rename success"),
        Err(_) => HttpResponse::Ok().body("File rename error"),
    };
    return process;
}
