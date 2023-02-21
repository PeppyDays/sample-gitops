use actix_web::{get, middleware::Logger, post, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;

#[get("/hey")]
async fn hey() -> impl Responder {
    HttpResponse::Ok().body("hey there?")
}

#[get("/env")]
async fn env() -> impl Responder {
    HttpResponse::Ok().body(std::env::var("ENV").unwrap_or("default".to_string()))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .service(hey)
            .service(env)
            .service(echo)
            .service(healthz)
            .wrap(Logger::default())
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
