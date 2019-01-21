extern crate url;
type ClientId = u64;

enum ClientType {
    Privilege,
    Public
}

struct Client<'a> {
    id: ClientId,
    name: &'a str,
    type: ClientType,
    secret: &'a str,
    redirect_uris: Vec<url::Url>,
}

struct User<'a> {
    id: u64,
    name: &'a str,
    email: &'a str,
    password: Option<&'a str>,
}
