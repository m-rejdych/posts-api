use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::{Cookie, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{Deserialize, Serialize};
use rocket::time::{Duration, OffsetDateTime};

use crate::config::Config;

#[derive(Debug)]
pub struct Error(pub &'static str);

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub user_id: i32,
    pub email: String,
    pub exp: u64,
}

#[derive(Debug)]
pub struct Jwt {
    pub token: String,
    pub claims: Claims,
}

impl<'r> Jwt {
    pub fn new(claims: Claims, secret: &[u8]) -> Jwt {
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret),
        )
        .unwrap();

        Jwt { token, claims }
    }

    pub fn cookie(&self) -> Cookie<'r> {
        let mut cookie = Cookie::new("jwt", self.token.clone());
        cookie.set_http_only(true);
        cookie.set_expires(OffsetDateTime::now_utc() + Duration::seconds(10));

        cookie
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Jwt {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let Config { ROCKET_JWT_SECRET } = match request.rocket().figment().extract::<Config>() {
            Ok(config) => config,
            Err(_) => {
                return Outcome::Failure((Status::Unauthorized, Error("Invalid configuration.")))
            }
        };

        let token = match request.cookies().get("jwt") {
            Some(cookie) => cookie.value().to_string(),
            None => return Outcome::Failure((Status::Unauthorized, Error("Invalid token."))),
        };

        let claims = match decode::<Claims>(
            &token,
            &DecodingKey::from_secret(ROCKET_JWT_SECRET.as_ref()),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(decoded) => decoded.claims,
            Err(_) => return Outcome::Failure((Status::Unauthorized, Error("Invalid token."))),
        };

        Outcome::Success(Jwt { token, claims })
    }
}
