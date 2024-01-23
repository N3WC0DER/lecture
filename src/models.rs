use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub chat_id: i64,
    pub username: String,
    pub moderator: bool,
    pub institute_id: i32,
    pub course: i32,
    pub direction_id: i32,
    pub notification: bool,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::reports)]
pub struct Report {
    pub subject_id: i32,
    pub lecture_id: i32,
}
