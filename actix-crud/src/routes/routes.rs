use actix_web::get;
use actix_web::HttpResponse;
use actix_web::Responder;



#[get("/")]
pub async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[get("/toastx")]
pub async fn toastx() -> impl Responder {
    HttpResponse::Ok().body("this is a toastx website")
}