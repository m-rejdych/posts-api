use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::http::Cookie;
use rocket::serde::Serialize;
use rocket::time::{Duration, OffsetDateTime};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims<'r> {
    pub user_id: &'r i32,
    pub email: &'r String,
}

pub struct Jwt(pub String);

impl<'r> Jwt {
    pub fn new(claims: &Claims<'r>, secret: &[u8]) -> Jwt {
        let token = encode(
            &Header::default(),
            claims,
            &EncodingKey::from_secret(secret),
        )
        .unwrap();

        Jwt(token)
    }

    pub fn cookie(&self) -> Cookie<'r> {
        let mut cookie = Cookie::new("jwt", self.0.clone());
        cookie.set_http_only(true);
        cookie.set_expires(OffsetDateTime::now_utc() + Duration::milliseconds(10000));

        cookie
    }
}
