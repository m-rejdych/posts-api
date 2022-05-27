use rocket::serde::{Deserialize, Serialize};

use crate::schema::user::User;

#[derive(Serialize, Deserialize, Queryable, Insertable, Identifiable, Associations)]
#[serde(crate = "rocket::serde")]
#[belongs_to(User, foreign_key = "user_id")]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub published: bool,
    pub user_id: i32,
}

table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        text -> Text,
        published -> Bool,
        user_id -> Integer,
    }
}
