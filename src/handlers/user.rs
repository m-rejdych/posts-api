use lazy_static::lazy_static;
use pwhash::bcrypt;
use regex::Regex;
use rocket::time::{OffsetDateTime, Duration};
use rocket::http::{Cookie, CookieJar, private::cookie::Expiration};
use rocket::response::status::Created;
use rocket::route::Route;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use rocket_sync_db_pools::diesel;
use rocket_validation::{Validate, Validated};

use crate::config::Config;
use crate::db::Db;
use crate::schema::user::{users, User};
use crate::auth::jwt::{Jwt, Claims};

use diesel::prelude::*;

type Result<T, E = rocket::response::Debug<diesel::result::Error>> = std::result::Result<T, E>;

lazy_static! {
    static ref RE_PASSWORD: Regex = Regex::new(r"^([a-zA-Z0-9@*#]{8,15})$").unwrap();
}

#[derive(Deserialize, Validate)]
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
async fn create(
    db: Db,
    config: &State<Config>,
    jar: &CookieJar<'_>,
    user: Validated<Json<CreateUserData>>,
) -> Result<Created<Json<User>>> {
    let user_data = user.0 .0;
    let hashed_password = bcrypt::hash(user_data.password).unwrap();

    let new_user = db
        .run(move |c| {
            diesel::insert_into(users::table)
                .values((
                    users::username.eq(user_data.username),
                    users::email.eq(user_data.email),
                    users::password.eq(hashed_password),
                ))
                .get_result::<User>(c)
        })
        .await?;

    let claims = Claims {
        user_id: &new_user.id,
        email: &new_user.email,
    };
    let token = Jwt::new(&claims, config.ROCKET_JWT_SECRET.as_ref());

    jar.add(token.cookie());

    Ok(Created::new("/").body(Json(new_user)))
}

#[get("/users")]
async fn list(db: Db) -> Result<Json<Vec<User>>> {
    let users = db.run(move |c| users::table.load(c)).await?;

    Ok(Json(users))
}

#[get("/users/<id>")]
async fn user(db: Db, id: i32) -> Option<Json<User>> {
    db.run(move |c| users::table.find(id).first(c))
        .await
        .map(|user| Json(user))
        .ok()
}

pub fn user_routes() -> Vec<Route> {
    routes![create, list, user]
}
