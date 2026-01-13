use sqlx::SqlitePool;
pub struct AppState {
    pub db: SqlitePool,
    pub folder: String,
    pub url: String,
}
#[derive(serde::Deserialize, sqlx::FromRow, Debug)]
pub struct FileForm {
    pub id: i32,
    pub file: String,
}
