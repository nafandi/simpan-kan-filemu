use crate::shared_values::{AppState, FileForm};
use actix_web::{HttpResponse, Responder, get, web::Data};
#[get("/list")]
pub async fn list(db: Data<AppState>) -> impl Responder {
    let list2: Vec<FileForm> = sqlx::query_as("SELECT * from files")
        .fetch_all(&db.db)
        .await
        .unwrap();
    let mut list = String::from("id,file\n");
    for i in &list2 {
        list += &format!("{},{}\n", i.id, i.file);
    }
    match list2.is_empty() {
        true => {
            list += &format!("you might want to upload to {}/upload/", db.url);
        }
        false => {
            list += &format!("you can access file with {}/files/(file)\n", db.url);
            list += &format!("example: {}/files/{}\n", db.url, list2[0].file);
            list += &format!(
                "or you want to delete file with {}/delete/\nwith fill id on form request\n",
                db.url
            );
            list += &format!(
                "or you want to rename file with {}/rename/\nwith fill file and id in form request",
                db.url
            );
        }
    }
    HttpResponse::Ok().body(format!("{}", list))
}
