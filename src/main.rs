extern crate diesel;

use actix_web::{web, App, HttpServer};
use std::env;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::MysqlConnection;
use dotenvy::dotenv;

mod routes;
mod swagger;
mod constants;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("Database url not found");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = Pool::builder().build(manager).expect("Could not create pool");

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .route("/api", web::get().to(swagger::doc))
        .service(routes::get_tweets)
        .service(routes::create_tweet)
        .service(routes::get_tweet)
        .service(routes::get_tweet_by_like)
        .service(routes::like_tweet)
        .service(routes::remove_like)
    })
    .bind("localhost:9090")?
    .run()
    .await
}
