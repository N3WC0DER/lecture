use diesel::{Insertable, Queryable};

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub chat_id: i64,
    pub name: String,
    pub group: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub chat_id: i64,
    pub name: &'a str,
    pub group: &'a str,
}
