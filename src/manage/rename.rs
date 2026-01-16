use crate::shared_values::{AppState, FileForm, Info};
use actix_web::{
    post,
    web::{self, Data},
};
use tokio::fs::rename as tokio_rename;
#[post("/rename")]
pub async fn rename(form: web::Json<FileForm>, db: Data<AppState>) -> web::Json<Info> {
    let get_file = match sqlx::query!("SELECT file FROM files WHERE id=$1", form.id)
        .fetch_one(&db.db)
        .await
    {
        Ok(get_file) => match get_file.file {
            Some(file) => file,
            None => {
                return web::Json(Info {
                    status: (404),
                    info: (format!("File not found")),
                });
            }
        },
        Err(_) => {
            return web::Json(Info {
                status: (404),
                info: (format!("File not found")),
            });
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
        Ok(_) => web::Json(Info {
            status: (200),
            info: (format!("OK. Rename success")),
        }),
        Err(_) => web::Json(Info {
            status: (503),
            info: (format!("Rename failed")),
        }),
    };
    return process;
}
