use actix_web::{get, post, put, delete, web, App, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::body::BoxBody;

use serde::{Serialize, Deserialize};

use std::fmt::Display;
use std::sync::Mutex;


#[derive(Serialize, Deserialize)]
struct Ticket {
   id: u32,
   author: String,
}

#[derive(Debug, Serialize)]
struct ErrNoId {
   id: u32,
   err: String,
}

// Implement Responder Trait for Ticket
impl Responder for Ticket {
   type Body = BoxBody;

   fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
      let res_body = serde_json::to_string(&self).unwrap();
      
      // Create HttpResponse and set Content Type
      HttpResponse::Ok()
         .content_type(ContentType::json())
         .body(res_body)
   }
}

// Implement ResponseError for ErrNoId
impl ResponseError for ErrNoId {
   fn status_code(&self) -> StatusCode {
       StatusCode::NOT_FOUND
   }

   fn error_response(&self) -> HttpResponse<BoxBody> {
      let body = serde_json::to_string(&self).unwrap();
      let res = HttpResponse::new(self.status_code());
      res.set_body(BoxBody::new(body))
   }
}

// Implement Display for ErrNoId
impl Display for ErrNoId {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{:?}", self)
   }
}

