use actix_web::{get, middleware::Logger, post, App, HttpResponse, HttpServer, Responder};

#[get("/hey")]
async fn hey() -> impl Responder {
    HttpResponse::Ok().body("hey there? this is order!")
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
