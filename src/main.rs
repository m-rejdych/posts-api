use rocket_validation::validation_catcher;
use rocket::fairing::AdHoc;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod db;
mod handlers;
mod schema;
mod config;
mod auth;

use handlers::{user::user_routes, post::post_routes, auth::auth_routes};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![validation_catcher])
        .attach(AdHoc::config::<config::Config>())
        .attach(db::stage())
        .mount("/auth", auth_routes())
        .mount("/user", user_routes())
        .mount("/post", post_routes())
}
