use rocket_validation::validation_catcher;

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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![validation_catcher])
        .attach(db::stage())
}