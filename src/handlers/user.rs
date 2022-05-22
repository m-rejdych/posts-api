use rocket::response::status::Created;
use rocket::route::Route;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_sync_db_pools::diesel;
use rocket_validation::{Validate, Validated};
use regex::Regex;
use lazy_static::lazy_static;

use crate::db::Db;
use crate::schema::user::{users, User};

use diesel::prelude::*;

type Result<T, E = rocket::response::Debug<diesel::result::Error>> = std::result::Result<T, E>;

lazy_static! {
    static ref RE_PASSWORD: Regex = Regex::new(r"^(?=.*\d).{4,8}$").unwrap();
}

#[derive(Serialize, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
struct CreateUserData {
    #[validate(length(min = 4))]
    username: String,
    #[validate(email)]
    email: String,
    #[validate(regex = "RE_PASSWORD")]
    password: String,
}

#[post("/create", data = "<user>")]
async fn create(db: Db, user: Validated<Json<CreateUserData>>) -> Result<Created<Json<User>>> {
    let new_user = db
        .run(move |c| {
            diesel::insert_into(users::table)
                .values((
                    users::username.eq(user.0 .0.username),
                    users::email.eq(user.0 .0.email),
                    users::password.eq(user.0 .0.password),
                ))
                .get_result::<User>(c)
        })
        .await?;

    Ok(Created::new("/").body(Json(new_user)))
}

pub fn user_routes() -> Vec<Route> {
    routes![create]
}
