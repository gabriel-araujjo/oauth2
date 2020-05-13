use rocket::http::RawStr;
use rocket::request::FromFormValue;
use std::vec::Vec;
use serde_derive::{Serialize, Deserialize};
use url::Url;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;

pub mod errors;
pub mod routes;

#[derive(FromForm)]
pub struct AuthForm<'a> {
    response_type: ResponseType,
    client_id: u64,
    redirect_uri: Option<&'a RawStr>,
    scope: Option<&'a RawStr>,
}

#[derive(Debug)]
pub enum ResponseType {
    Code,
    Token,
    Other(String)
}

impl<'v> FromFormValue<'v> for ResponseType {
    type Error = OAuthErr;
    fn from_form_value(from_value: &'v RawStr) -> Result<Self, Self::Error> {
        match from_value.as_str() {
            "code" => Ok(ResponseType::Code),
            "token" => Ok(ResponseType::Token),
            s => Err(OAuthErr::UnsupportedResponseType(ResponseType::Other(String::from(s))))
        }
    }
}

#[derive(Debug)]
pub enum UserDatum {
    Name,
    Email,
    OpenId,
    Other(String)
}

impl<T: Into<String>> From<T> for UserDatum {
    fn from(s: T) -> UserDatum {
        match s.into().as_ref() {
            "name" => UserDatum::Name,
            "email" => UserDatum::Email,
            "openid" => UserDatum::OpenId,
            s => UserDatum::Other(String::from(s))
        }
    }
}

#[derive(Debug)]
pub Scope(Vec<UserDatum>);

impl<'v> FromFormValue<'v> for Scope {
    type Error = ();
    fn from_form_value(value: &'v RawStr) -> Result<Self, Self::Error> {
        Ok(Scope(value.as_str().split_whitespace().map(UserDatum::from).collect()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    #[serde(with = "url_serde")]
    iss: Url, // Issuer
    sub: Uuid, // Person id
    aud: Uuid, // Client id
    #[serde(with = "ts_seconds")]
    iat: DateTime<Utc>, // Issued at
    #[serde(with = "ts_seconds")]
    exp: DateTime<Utc>, // Expiration time
}
