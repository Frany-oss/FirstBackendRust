use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use actix_web::{
    post, get,
    web::{Data, Json, Path},
    HttpResponse,
};

#[post("/user")]
pub async fn create_user(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse {
    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };
     let user_detail = db.create_user(data).await;
     match user_detail {
         Ok(user) => HttpResponse::Ok().json(user),
         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/user/{id}")]
pub async fn get_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("Invalid ID");
    }
    let user_detail = db.get_user(&id).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
