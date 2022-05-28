use lazy_static::lazy_static;
use regex::Regex;
use rocket::route::Route;
use rocket::serde::json::Json;

use crate::db::Db;
use crate::schema::user::{users, User};
use crate::auth::jwt::Jwt;

use diesel::prelude::*;

type Result<T, E = rocket::response::Debug<diesel::result::Error>> = std::result::Result<T, E>;

lazy_static! {
    static ref RE_PASSWORD: Regex = Regex::new(r"^([a-zA-Z0-9@*#]{8,15})$").unwrap();
}

#[get("/users")]
async fn list(db: Db) -> Result<Json<Vec<User>>> {
    let users = db.run(move |c| users::table.load(c)).await?;

    Ok(Json(users))
}

#[get("/users/<id>")]
async fn user(db: Db, _jwt: Jwt, id: i32) -> Option<Json<User>> {
    db.run(move |c| users::table.find(id).first(c))
        .await
        .map(|user| Json(user))
        .ok()
}

pub fn user_routes() -> Vec<Route> {
    routes![list, user]
}
