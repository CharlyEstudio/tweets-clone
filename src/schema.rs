// @generated automatically by Diesel CLI.

diesel::table! {
    likes (id) {
        id -> Integer,
        created_at -> Nullable<Timestamp>,
        tweet_id -> Integer,
    }
}

diesel::table! {
    tweets (id) {
        id -> Integer,
        created_at -> Nullable<Timestamp>,
        message -> Varchar,
    }
}

diesel::joinable!(likes -> tweets (tweet_id));

diesel::allow_tables_to_appear_in_same_query!(
    likes,
    tweets,
);
