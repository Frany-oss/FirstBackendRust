
use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "Hello World"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            // prefixes all resources and routes attached to it...
            web::scope("/app")
            // ...so this handles requests fot 'GET /app/index.hmtl'
            .route("/index.html", web::get().to(index)),
        )
    })
    .bind(("127.0.0.1", 8082))?
    .run()
    .await
}
