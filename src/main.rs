use actix_web::{web ,App, HttpServer};
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::env;
use dotenv::dotenv;

mod controllers;


#[derive(Clone)]
struct AppState {
    pool: MySqlPool
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    println!("heheh{:#?}", env::var("DATABASE_URL"));
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();
    let app_state = AppState { pool };
    HttpServer::new( || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .data(pool.clone())
            .route("/users/{name}", web::get().to(controllers::handlers::get_user))  

            //.route("/users/{name}", web::Path(controllers::handlers::get_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}