extern crate url;

use std::collections::HashSet;
use url::Url;
use rocket::http::RawStr;
use rocket::request::FromFormValue;

struct Client {
    id: u64,
    name: String,
    type: ClientType,
    secret: String,
    redirect_uris: Vec<Url>,
}

enum ClientType {
    Privileged,
    Public
}

enum UserInformation {
    Name,
    Email,
    OpenId,
    Other(String)
}
