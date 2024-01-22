// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        chat_id -> Int8,
        name -> Varchar,
        group -> Varchar,
    }
}
