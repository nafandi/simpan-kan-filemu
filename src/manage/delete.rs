#[derive(serde::Deserialize, Debug)]
struct FileFormOptional {
    id: Option<i32>,
}
use crate::shared_values::{AppState, Info};
use actix_web::{
    post,
    web::{self, Data},
};
use tokio::fs::remove_file;
#[post("/delete")]
pub async fn delete(form: web::Json<FileFormOptional>, db: Data<AppState>) -> web::Json<Info> {
    let get_file = match sqlx::query!("SELECT * FROM files WHERE id=$1", form.id)
        .fetch_one(&db.db)
        .await
    {
        Ok(get_file) => match get_file.file {
            Some(file) => file,
            None => {
                return web::Json(Info {
                    status: (404),
                    info: (format!("Not Found")),
                });
            }
        },
        Err(_) => {
            return web::Json(Info {
                status: (404),
                info: (format!("Not Found")),
            });
        }
    };
    let file_name = get_file.clone();
    let _ = sqlx::query!(
        "DELETE FROM files WHERE id=$2 AND file=$1",
        file_name,
        form.id
    )
    .execute(&db.db)
    .await;
    let process = match remove_file(format!("{}/{}", db.folder, get_file)).await {
        Ok(_) => web::Json(Info {
            status: (200),
            info: (format!("File deletion success")),
        }),
        Err(_) => web::Json(Info {
            status: (503),
            info: (format!("Deletion failed")),
        }),
    };
    return process;
}
