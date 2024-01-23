// @generated automatically by Diesel CLI.

diesel::table! {
    reports (subject_id) {
        subject_id -> Int4,
        lecture_id -> Int4,
    }
}

diesel::table! {
    users (chat_id) {
        chat_id -> Int8,
        username -> Varchar,
        moderator -> Bool,
        institute_id -> Int4,
        course -> Int4,
        direction_id -> Int4,
        notification -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(reports, users,);
