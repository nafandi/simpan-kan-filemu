#[derive(serde::Deserialize, Debug)]
struct FileFormOptional {
    id: Option<i32>,
}
use crate::shared_values::AppState;
use actix_web::{
    HttpResponse, Responder, post,
    web::{self, Data},
};
use tokio::fs::remove_file;
#[post("/delete")]
pub async fn delete(form: web::Form<FileFormOptional>, db: Data<AppState>) -> impl Responder {
    let get_file = match sqlx::query!("SELECT * FROM files WHERE id=$1", form.id)
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
    let file_name = get_file.clone();
    let _ = sqlx::query!(
        "DELETE FROM files WHERE id=$2 AND file=$1",
        file_name,
        form.id
    )
    .execute(&db.db)
    .await;
    let process = match remove_file(format!("{}/{}", db.folder, get_file)).await {
        Ok(_) => HttpResponse::Ok().body("File delete success"),
        Err(_) => HttpResponse::Ok().body("File delete error"),
    };
    return process;
}
