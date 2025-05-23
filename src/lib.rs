use std::net::TcpListener;

use actix_web::{web, App, HttpServer, Responder, Result};
use actix_web::dev::Server;
use serde::{ Deserialize, Serialize };


pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/check_flag", web::get().to(check_flag))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

async fn check_flag(form: web::Query<GetFlagQuery>) -> impl Responder {
    let returned_user = FeatureFlagData {
        user: form.user.clone(), enabled: form.user == "khang"
    };


    web::Json(returned_user)
    // format!("name = {}; enabled = {}", returned_user.user, returned_user.enabled)
}


#[derive(Deserialize)]
pub struct GetFlagQuery {
    user: String,
}

#[derive(Serialize, Deserialize)]
pub struct FeatureFlagData {
    pub user: String,
    pub enabled: bool
}
