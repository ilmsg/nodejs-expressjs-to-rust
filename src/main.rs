use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: i32,
}

async fn index(user: web::Json<User>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(user.0))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::post().to(index)))
        .bind(("127.0.0.1", 7010))?
        .run()
        .await
}
