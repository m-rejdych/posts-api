use rocket::route::Route;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket_sync_db_pools::diesel;

use crate::db::Db;
use crate::schema::user::{users, User};

use diesel::prelude::*;

type Result<T, E = rocket::response::Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[post("/create", data = "<user>")]
async fn create(db: Db, user: Json<User>) -> Result<Created<Json<User>>> {
    let new_user = db
        .run(move |c| {
            diesel::insert_into(users::table)
                .values(&*user)
                .get_result::<User>(c)
        })
        .await?;

    Ok(Created::new("/").body(Json(new_user)))
}

pub fn user_routes() -> Vec<Route> {
    routes![create]
}
