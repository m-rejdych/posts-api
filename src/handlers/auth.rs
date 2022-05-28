use lazy_static::lazy_static;
use pwhash::bcrypt;
use regex::Regex;
use rocket::http::CookieJar;
use rocket::response::status::{Created, Unauthorized};
use rocket::serde::{json::Json, Deserialize};
use rocket::Route;
use rocket::State;
use rocket_sync_db_pools::diesel;
use rocket_validation::{Validate, Validated};

use diesel::prelude::*;

use crate::auth::jwt::{Claims, Jwt};
use crate::config::Config;
use crate::db::Db;
use crate::schema::user::{users, User};

lazy_static! {
    static ref RE_PASSWORD: Regex = Regex::new(r"^([a-zA-Z0-9@*#]{8,15})$").unwrap();
}

type Result<T, E = rocket::response::Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[derive(Responder)]
struct Error(String);

#[derive(Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
struct RegisterData {
    #[validate(length(min = 2))]
    username: String,
    #[validate(email)]
    email: String,
    #[validate(regex = "RE_PASSWORD")]
    password: String,
}

#[derive(Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
struct LoginData {
    #[validate(email)]
    email: String,
    password: String,
}

#[post("/register", data = "<user>")]
async fn register(
    db: Db,
    config: &State<Config>,
    jar: &CookieJar<'_>,
    user: Validated<Json<RegisterData>>,
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

    let claims = Claims::new(new_user.id, new_user.email.clone());
    let jwt = Jwt::new(claims, config.ROCKET_JWT_SECRET.as_ref());

    jar.add(jwt.cookie());

    Ok(Created::new("/").body(Json(new_user)))
}

#[put("/login", data = "<user>")]
async fn login(
    db: Db,
    jar: &CookieJar<'_>,
    config: &State<Config>,
    user: Validated<Json<LoginData>>,
) -> Result<Json<User>, Unauthorized<Error>> {
    let login_data = user.0 .0;

    let user = match db
        .run(move |c| {
            users::table
                .filter(users::email.eq(login_data.email))
                .first::<User>(c)
        })
        .await
    {
        Ok(u) => u,
        Err(_) => {
            return Err(Unauthorized(Some(Error(
                "Incorrect email or password.".to_string(),
            ))))
        }
    };

    if !bcrypt::verify(login_data.password, &user.password) {
        return Err(Unauthorized(Some(Error(
            "Incorrect email or password.".to_string(),
        ))));
    }

    let claims = Claims::new(user.id, user.email.clone());
    let jwt = Jwt::new(claims, config.ROCKET_JWT_SECRET.as_ref());

    jar.add(jwt.cookie());
    Ok(Json(user))
}

pub fn auth_routes() -> Vec<Route> {
    routes![register, login]
}
