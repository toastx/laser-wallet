use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/toastx")]
async fn toastx() -> &'static str{
    "this is a toastx website"
}


#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
        cfg.service(toastx);
        
    };

    Ok(config.into())
}

/*
    setup the db

    declare the routes
    declare the cors middleware

    start listening for requests


*/