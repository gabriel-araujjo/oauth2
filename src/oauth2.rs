use url::Url;
use rocket::http::{Status, RawStr};
use rocket::Outcome;
use rocket::request::{self, Request, FromRequest, FromFormValue, FromQuery};

pub type ClientId = String;

#[derive(Debug)]
pub enum ResponseType {
    Code,
    Token,
    Other(String)
}

impl<'v> FromFormValue<'v> for ResponseType {
    type Error = ();
    fn from_form_value(from_value: &'v RawStr) -> Result<ResponseType, ()> {
        match from_value.as_str() {
            "code" => Ok(ResponseType::Code),
            "token" => Ok(ResponseType::Token),
            s => Ok(ResponseType::Other(s.into()))
        }
    }
}

pub enum GrantType {
    Code(String),
    Implicit,
    Password {username: String, password: String},
    ClientCredentials,
    RefreshToken(String),
    Other(String)
}

#[derive(Debug)]
pub enum Scope {
    Name,
    Email,
    OpenId,
    Other(String)
}

impl<T: Into<String>> From<T> for Scope {
    fn from(s: T) -> Scope {
        match s.into().as_ref() {
            "name" => Scope::Name,
            "email" => Scope::Email,
            "openid" => Scope::OpenId,
            s => Scope::Other(String::from(s))
        }
    }
}

#[derive(FormData)]
pub struct AuthRequest {
    response_type: ResponseType,
    client_id: ClientId,
    redirect_uri: Url,
    scope: Vec<Scope>,
    state: Option<String>,
}

#[derive(Debug)]
pub enum AuthRequestErr {
    InvalidRequest,
    UnauthorizedClient(ClientId),
    AccessDenied,
    UnsupportedResponseType(ResponseType),
    InvalidScope(Vec<Scope>),
    ServerError,
    TemporarilyUnavailable,
}

impl<'q> FromQuery<'q> for AuthRequest {
    
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthRequest {

    type Error = AuthRequestErr;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthRequest, AuthRequestErr> {
        let response_type: ResponseType = match request.get_query_value("response_type") {
            Some(Ok(ResponseType::Code)) => ResponseType::Code,
            _ => return Outcome::Failure((Status::BadRequest, AuthRequestErr::TemporarilyUnavailable))
        };
        let client_id: String = match request.get_query_value("client_id") {
            Some(Ok(s)) => s,
            _ => return Outcome::Failure((Status::BadRequest, AuthRequestErr::TemporarilyUnavailable))
        };
        let redirect_uri: String = match request.get_query_value("redirect_uri") {
            Some(Ok(u)) => u,
            _ => return Outcome::Failure((Status::BadRequest, AuthRequestErr::TemporarilyUnavailable))
        };
        let scope: Vec<Scope> = match request.get_query_value::<RawStr>("scope") {
            Some(Ok(s)) => s.split_whitespace().into_iter().map(Scope::from).collect(),
            _ => return Outcome::Failure((Status::BadRequest, AuthRequestErr::TemporarilyUnavailable))
        };

        match request.get_query_value("response_type") { 
            Some(Ok(ResponseType::Code)) => {
                Outcome::Success(AuthRequest {
                    response_type: ResponseType::Code,
                    client_id: String::from("client_id"),
                    redirect_uri: Url::parse("https://localhost/cb").unwrap(),
                    scope: vec![Scope::Name, Scope::Email],
                    state: None,
                })
            },
            _ => Outcome::Failure((Status::BadRequest, AuthRequestErr::TemporarilyUnavailable))
        }
    }   
}

struct TokenRequest {
    grant_type: GrantType,
    client_id: ClientId,
    scope: Vec<Scope>,
}
