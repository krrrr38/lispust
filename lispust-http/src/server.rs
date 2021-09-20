extern crate lispust_core;

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

#[post("/lispust")]
async fn lispust(req_body: String) -> impl Responder {
    match lispust_core::run(&req_body) {
        Ok(ret) => HttpResponse::Ok().body(ret),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8080";
    println!("start server on {}", addr);
    HttpServer::new(|| App::new().service(health).service(lispust))
        .bind(addr)?
        .run()
        .await
}
