use rocket::http::Status;
use rocket::response::status::{Created, Unauthorized};
use rocket::route::Route;
use rocket::serde::{json::Json, Deserialize};
use rocket_sync_db_pools::diesel;
use rocket_validation::{Validate, Validated};

use diesel::prelude::*;

use crate::auth::jwt::Jwt;
use crate::db::Db;
use crate::schema::{
    post::{posts, Post},
    user::{users, User},
};

type Result<T, E = rocket::response::Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[derive(Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
struct CreatePostData {
    #[validate(length(min = 5))]
    title: String,
    #[validate(length(min = 10))]
    text: String,
}

#[post("/create", data = "<post>")]
async fn create(
    db: Db,
    jwt: Jwt,
    post: Validated<Json<CreatePostData>>,
) -> Result<Created<Json<Post>>> {
    let post_data = post.0 .0;

    let new_post = db
        .run(move |c| {
            diesel::insert_into(posts::table)
                .values((
                    posts::title.eq(post_data.title),
                    posts::text.eq(post_data.text),
                    posts::user_id.eq(jwt.claims.user_id),
                ))
                .get_result::<Post>(c)
        })
        .await?;

    Ok(Created::new("/").body(Json(new_post)))
}

#[put("/<id>/publish")]
async fn publish(db: Db, jwt: Jwt, id: i32) -> Result<Json<Post>, Status> {
    let post = match db
        .run(move |c| {
            diesel::update(posts::table.find(id))
                .set(posts::published.eq(true))
                .get_result::<Post>(c)
        })
        .await
    {
        Ok(post) => Json(post),
        Err(_) => return Err(Status::Unauthorized),
    };

    if post.user_id != jwt.claims.user_id {
        return Err(Status::Unauthorized);
    }

    Ok(post)
}

#[get("/posts-by-user-id/<id>")]
async fn posts_by_user_id(db: Db, id: i32) -> Result<Json<Vec<Post>>> {
    let posts = db
        .run(move |c| {
            let user = users::table.find(id).first::<User>(c)?;

            Post::belonging_to(&user).load::<Post>(c)
        })
        .await?;

    Ok(Json(posts))
}

pub fn post_routes() -> Vec<Route> {
    routes![create, posts_by_user_id, publish]
}
