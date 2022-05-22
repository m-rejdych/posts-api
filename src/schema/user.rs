use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: Option<i32>,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
}

table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        email -> Text,
        password -> Text,
    }
}
