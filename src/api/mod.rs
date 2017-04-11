use rocket::{State, Outcome};
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

use ::config::Config;

pub struct ApiKey;

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, ()> {
        use self::Outcome::*;

        let mut keys = request.headers().get("x-api-key");

        if let Some(key) = keys.next() {
            let config: State<Config> = State::from_request(request).expect("Config state not managed!");

            for k in &config.keys {
                if k == key { return Outcome::Success(ApiKey) }
            }

            return Outcome::Failure((Status::Unauthorized, ()));
        } else {
            return Failure((Status::BadRequest, ()));
        }        
    }
}