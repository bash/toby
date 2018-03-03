use crate::config::{self, Config};
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request, State};
use std::ops;

pub struct ValidToken<'a, 'r>(&'r config::Token, &'a str);

fn parse_authorization_header<'a>(header_val: &'a str) -> Option<(&'a str, &'a str)> {
    const SCHEME: &str = "Token";

    if header_val.starts_with(SCHEME) && header_val.len() > SCHEME.len() + 1 {
        let val = &header_val[SCHEME.len() + 1..];
        let mut parts = val.split(':');
        let token = parts.next()?;
        let secret = parts.next()?;

        Some((token, secret))
    } else {
        None
    }
}

impl<'a, 'r> ValidToken<'a, 'r> {
    pub fn token_name(&self) -> &'a str {
        self.1
    }
}

impl<'a, 'r> ops::Deref for ValidToken<'a, 'r> {
    type Target = config::Token;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ValidToken<'a, 'r> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        const AUTHORIZATION: &str = "authorization";

        macro forbidden() {
            Outcome::Failure((Status::Forbidden, ()));
        };

        let config = {
            let config: State<Config> = match request.guard() {
                Outcome::Success(val) => val,
                _ => return forbidden!(),
            };

            config.inner()
        };

        let (token_str, secret) = match request
            .headers()
            .get_one(AUTHORIZATION)
            .and_then(parse_authorization_header)
        {
            Some(val) => val,
            None => return forbidden!(),
        };

        match config.tokens.get(token_str) {
            Some(token) if token.secret == secret => Outcome::Success(ValidToken(token, token_str)),
            _ => forbidden!(),
        }
    }
}
