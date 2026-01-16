use sqlx::SqlitePool;
pub struct AppState {
    pub db: SqlitePool,
    pub folder: String,
}
#[derive(serde::Deserialize, serde::Serialize, sqlx::FromRow, Debug)]
pub struct FileForm {
    pub id: i32,
    pub file: String,
}
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Info {
    pub status: i32,
    pub info: String,
}
