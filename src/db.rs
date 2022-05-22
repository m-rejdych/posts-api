use rocket::fairing::AdHoc;
use rocket_sync_db_pools::diesel;

use crate::handlers::user::user_routes;

#[database("diesel")]
pub struct Db(diesel::PgConnection);

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Stage", |rocket| async {
        rocket.attach(Db::fairing()).mount("/user", user_routes())
    })
}
