// use chrono::{NaiveDateTime, Utc};
// use serde::{Deserialize, Serialize};
// use diesel::{Insertable, Queryable, RunQueryDsl};

// #[derive(Insertable, Queryable, Deserialize, Serialize)]
// pub struct Tweet {
//   created_at: NaiveDateTime,
//   message: String,
// }

// impl Tweet {
//   pub fn new(message: String) -> Self {
//     Self { created_at: Utc::now().naive_utc(), message }
//   }
// }