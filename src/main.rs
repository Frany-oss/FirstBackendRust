mod api;
mod models;
mod repository;

use actix_web::{web::Data, App, HttpServer};
use api::user_api::{create_user};
use repository::mongodb_repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || { 
        App::new()
            .app_data(db_data.clone())
            .service(create_user)
    })
        .bind(("localhost", 8082))?
        .run()
        .await
}
