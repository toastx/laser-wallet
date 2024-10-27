use actix_cors::Cors;
use actix_web::{ http::header, web::{self, ServiceConfig}};
use dotenv::dotenv;
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::{postgres::PgPoolOptions, Postgres,Pool};
use std::env;

mod routes;

#[derive(Clone)]
struct AppState {
    pool: Pool<Postgres>,
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    if env::var_os("RUST_LOG").is_none(){
        env::set_var("RUST_LOG", "actix_web=info")
    }
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("no database url");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .unwrap();

    println!("server started successfully!!");

    let state = web::Data::new(AppState { pool });

    let config = move |cfg: &mut ServiceConfig| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET","POST","PUT","DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT
            ])
            .supports_credentials();

        cfg.service(
            web::scope("/app")
                .wrap(cors)
                .service(routes::hello_world)
                .service(routes::toastx)
                .app_data(state),
        );
    };

    Ok(config.into())

    
}

/*
    setup the db

    declare the routes
    declare the cors middleware

    start listening for requests


*/