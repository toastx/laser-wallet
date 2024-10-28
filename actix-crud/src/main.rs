use actix_cors::Cors;
use actix_web::{ http::header, web::{self, ServiceConfig}};
use dotenv::dotenv;
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::{pool, postgres::PgPoolOptions, Pool, Postgres};
use std::env;

mod routes;
mod models;

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
    println!("connected to pool ✅");

    println!("server started successfully!! 🚀");

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
            web::scope("/api")
                .wrap(cors)
                .service(routes::create_todo)
                .service(routes::update_todo)
                .service(routes::delete_todo)
                .service(routes::get_todo_by_id)
                .service(routes::get_todo_items)
                .service(routes::health_checker_endpoint)
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