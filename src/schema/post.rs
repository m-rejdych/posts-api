use rocket::serde::{Deserialize, Serialize};

use crate::schema::user::User;

#[derive(Serialize, Deserialize, Queryable, Insertable, Identifiable, Associations)]
#[serde(crate = "rocket::serde")]
#[belongs_to(User)]
#[table_name = "posts"]
struct Post {
    id: Option<i32>,
    title: String,
    text: String,
    published: bool,
    user_id: i32,
}

table! {
    posts (id) {
        id -> Nullable<Integer>,
        title -> Text,
        text -> Text,
        published -> Bool,
        user_id -> Integer,
    }
}
