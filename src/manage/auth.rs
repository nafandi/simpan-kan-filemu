use crate::shared_values::AppState;
use actix_web::{dev::ServiceRequest, error::ErrorUnauthorized, web::Data};
use actix_web_httpauth::{self, extractors::basic::BasicAuth};
pub async fn auth(
    req: ServiceRequest,
    cred: BasicAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let db = req.app_data::<Data<AppState>>();
    let username = cred.user_id();
    let username_fetch =
        match sqlx::query!("SELECT password FROM users WHERE username=$1", username)
            .fetch_one(&db.unwrap().db)
            .await
        {
            Ok(x) => x.password,
            Err(_) => return Err((ErrorUnauthorized("401 : Unauthorized"), req)),
        };
    if cred.password() == Some(&username_fetch) {
        return Ok(req);
    } else {
        return Err((ErrorUnauthorized("401 : Unauthorized"), req));
    }
}
