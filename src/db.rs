use rocket::fairing::AdHoc;
use rocket::{Build, Rocket};
use rocket_sync_db_pools::diesel;

use crate::handlers::user::user_routes;

#[database("diesel")]
pub struct Db(diesel::PgConnection);

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!("db/migrations");

    let conn = Db::get_one(&rocket).await.expect("Database connection");
    conn.run(|c| embedded_migrations::run_with_output(c, &mut std::io::stdout()))
        .await
        .expect("Running migrations");

    rocket
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Stage", |rocket| async {
        rocket
            .attach(Db::fairing())
            .attach(AdHoc::on_ignite("Diesel migrations", run_migrations))
            .mount("/user", user_routes())
    })
}
