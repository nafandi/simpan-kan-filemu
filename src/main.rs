use actix_files::{self, Files};
use actix_web::{App, HttpServer, web};
use actix_web_httpauth::{self, middleware::HttpAuthentication};
use dotenv::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
// import own module
mod manage;
mod shared_values;
use crate::manage::{
    auth::auth, delete::delete, home::home, list::list, rename::rename, upload::upload,
};
use crate::shared_values::AppState;

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
            .service(Files::new("/files", saved_file.clone()).index_file("private_index"))
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
