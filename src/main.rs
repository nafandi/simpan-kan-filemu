use actix_files::{self, Files};
use actix_multipart::Multipart;
use actix_web::{
    App, HttpResponse, HttpServer, Responder,
    dev::ServiceRequest,
    error::ErrorUnauthorized,
    get, post,
    web::{self, Data},
};
use actix_web_httpauth::{self, extractors::basic::BasicAuth, middleware::HttpAuthentication};
use dotenv::dotenv;
use futures_util::StreamExt as _;
//use sqlx::{PgPool, postgres::PgPoolOptions};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
//use serde::Deserialize;
use tokio::{fs::remove_file, fs::rename as tokio_rename, io::AsyncWriteExt};
struct AppState {
    db: SqlitePool,
    folder: String,
    url: String,
}
#[derive(serde::Deserialize, sqlx::FromRow, Debug)]
struct FileForm {
    id: i32,
    file: String,
}
#[derive(serde::Deserialize, Debug)]
struct FileFormOptional {
    id: Option<i32>,
}
#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Simpan Kan Filemu")
}

#[post("/upload")]
async fn upload(
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
            println!("Uploading...");
            let _ = file.write_all(&chunk).await?;
        }
    }
    Ok(HttpResponse::Ok().finish())
}
#[get("/list")]
async fn list(db: Data<AppState>) -> impl Responder {
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
#[post("/rename")]
async fn rename(form: web::Form<FileForm>, db: Data<AppState>) -> impl Responder {
    let get_file = sqlx::query!("SELECT * FROM files WHERE id=$1", form.id)
        .fetch_one(&db.db)
        .await;
    let _ = sqlx::query!("UPDATE files SET file=$1 WHERE id=$2", form.file, form.id)
        .execute(&db.db)
        .await;
    let process = tokio_rename(
        format!("{}/{}", db.folder, get_file.unwrap().file.unwrap()),
        format!("{}/{}", db.folder, form.file),
    )
    .await;
    HttpResponse::Ok().body(format!("{:#?}", process))
}
// todo
#[post("/delete")]
async fn delete(form: web::Form<FileFormOptional>, db: Data<AppState>) -> impl Responder {
    let get_file = sqlx::query!("SELECT * FROM files WHERE id=$1", form.id)
        .fetch_one(&db.db)
        .await
        .unwrap()
        .file;
    let file_name = get_file.clone().unwrap();
    let _ = sqlx::query!(
        "DELETE FROM files WHERE id=$2 AND file=$1",
        file_name,
        form.id
    )
    .execute(&db.db)
    .await;
    let process = remove_file(format!("{}/{}", db.folder, get_file.unwrap())).await;
    HttpResponse::Ok().body(format!("{:#?}", process))
}

async fn auth(
    req: ServiceRequest,
    cred: BasicAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    if cred.user_id() == "nur" && cred.password() == Some("nur") {
        return Ok(req);
    } else {
        return Err((ErrorUnauthorized("401 : Unauthorized"), req));
    }
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Simpan Kan Filemu - v0.1.0");
    dotenv().ok();
    let url = match std::env::var("HOST_URL") {
        Ok(url) => url,
        Err(_) => {
            eprintln!("Please set HOST_URL on .env file");
            std::process::exit(1);
        }
    };
    let db_url = match std::env::var("DATABASE_URL") {
        Ok(db_url) => db_url,
        Err(_) => {
            eprintln!("Please set DATABASE_URL on .env file");
            std::process::exit(1);
        }
    };
    let saved_file = match std::env::var("FOLDER_STORAGE") {
        Ok(saved_file) => saved_file,
        Err(_) => {
            eprintln!("Please set FOLDER_STORAGE on .env file");
            std::process::exit(1);
        }
    };
    let db = match SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            eprint!("Database can't connect because {}", e);
            std::process::exit(1);
        }
    }; //.expect("Can't connect to DB, so program ends");
    let working_url = url.clone();
    println!("This things run on {}", &url);
    let server = match HttpServer::new(move || {
        App::new()
            .service(home)
            .service(
                web::scope("/manage")
                    .wrap(HttpAuthentication::with_fn(auth))
                    .app_data(web::Data::new(AppState {
                        db: db.clone(),
                        folder: saved_file.clone(),
                        url: working_url.clone(),
                    }))
                    .service(upload)
                    .service(rename)
                    .service(list)
                    .service(delete),
            )
            .service(Files::new("/files", saved_file.clone()))
    })
    .bind(url.clone())
    {
        Ok(server) => server,
        Err(_) => {
            eprintln!("Cant bind url and port, please use other port");
            std::process::exit(1);
        }
    };
    server.run().await
    //.run()
    //.await;
}
