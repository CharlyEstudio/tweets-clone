use actix_web::{get, post, delete, HttpResponse};
use actix_web::web::{Path, Data};
use serde::{Deserialize, Serialize};
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::MysqlConnection;
use diesel::{Insertable, Queryable, RunQueryDsl};
use chrono::{NaiveDateTime, Utc};
use super::schema::tweets;

use crate::constants::APPLICATION_JSON;

#[derive(Insertable, Queryable, Deserialize, Serialize)]
pub struct Tweet {
  created_at: NaiveDateTime,
  message: String,
}

impl Tweet {
  pub fn new(message: String) -> Self {
    Self { created_at: Utc::now().naive_utc(), message }
  }
}

#[get("/tweets")]
pub async fn get_tweets() -> HttpResponse {
    // TODO: Obtener tweets
  let tweets = [
    "tweet 1: Hola",
    "tweet 2: Chao"
  ];

  HttpResponse::Ok()
    .content_type(APPLICATION_JSON)
    .json(tweets)
}

#[post("/tweets")]
pub async fn create_tweet(req_body: String, pool: Data<Pool<ConnectionManager<MysqlConnection>>>) -> HttpResponse {
  let new_tweet = Tweet::new(req_body);
  let mut conn = pool.get().expect("could not get database connection");

  diesel::insert_into(tweets::table)
    .values(&new_tweet)
    .execute(&mut conn)
    .expect("error inseting tweet");
  HttpResponse::Created()
    .content_type(APPLICATION_JSON)
    .json(&new_tweet)
}

#[get("/tweets/{id}")]
pub async fn get_tweet(path: Path<(String,)>) -> HttpResponse {
    // TODO: Obtener tweet con el id
  let tweet = format!("Este es el tweet {:?}", path.0);

  HttpResponse::Ok()
    .content_type(APPLICATION_JSON)
    .json(tweet)
}

#[get("/tweets/{id}/likes")]
pub async fn get_tweet_by_like() -> HttpResponse {
    // TODO: Obtener el numero de likes del tweet
  let likes = 100;

  HttpResponse::Ok()
    .content_type(APPLICATION_JSON)
    .json(likes)
}

#[post("/tweets/{id}/likes")]
pub async fn like_tweet(path: Path<(String,)>) -> HttpResponse {
    // TODO: Hacer like en tweet
  let like = format!("Ok {:?}", path.0);

  HttpResponse::Created()
    .content_type(APPLICATION_JSON)
    .json(like)
}

#[delete("/tweets/{id}/likes")]
pub async fn remove_like(path: Path<(String,)>) -> HttpResponse {
    // TODO: Disminuir el numero de likes de ese tweet
  let like = format!("Ok {:?}", path.0);

  HttpResponse::Ok()
    .content_type(APPLICATION_JSON)
    .json(like)
}
