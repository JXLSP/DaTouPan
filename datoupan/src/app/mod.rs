use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

pub mod api;
pub mod core;
pub mod models;
pub mod routers;
pub mod middlewares;
mod services;

#[get("/test")]
async fn test() -> impl Responder {
    HttpResponse::Ok().body("test OK!")
}

#[tokio::main]
async fn run_http_server() -> std::io::Result<()> {
    let mut server = HttpServer::new(move || {
        App::new().route("/test", web::get().to(test))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await;

    Ok(())
}

pub async fn main() {
    let result = run_http_server();
    if let Some(result) = result.await.err() {
        println!("{:?}", result);
    }
}