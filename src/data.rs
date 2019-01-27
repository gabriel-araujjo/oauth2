use std::collections::HashMap;

extern crate url;
struct ClientId(u64);

impl From<u64> {
    fn from(n: u64) -> Self {
        ClientId (n)
    }
}

enum ClientType {
    Privileged,
    Public
}

struct Client {
    id: ClientId,
    name: String,
    type: ClientType,
    secret: String,
    redirect_uris: Vec<url::Url>,
}

struct User<'a> {
    id: u64,
    name: &'a str,
    email: &'a str,
    password: Option<&'a str>,
}

const VALID_CLIENTS: HashMap<ClientId, Client> = 
[(1, Client {
    id: ClientId::from(1),
    name: String::from("cleint 1"),
    type: Privileged,
    secret: String::from("catkeyboard"),
    redirect_uris: Vev::new() })].iter().collect();

pub impl for Client {
    fn get_by_id(id: ClientId) -> Option<Client> {
        VALID_CLIENTS.GET(id)
    }
}