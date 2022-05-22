#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod db;
mod schema;
mod handlers;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(db::stage())
}
