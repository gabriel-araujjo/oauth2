use rocket::http::RawStr;
use rocket::request::FromFormValue;

#[derive(Debug)]
pub enum OAuthErr {
    InvalidRequest,
    UnauthorizedClient(u64),
    AccessDenied,
    UnsupportedResponseType(ResponseType),
    InvalidScope(Scope),
    ServerError,
    TemporarilyUnavailable,
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

struct TokenRequest {
    grant_type: GrantType,
    client_id: u64,
    scope: Vec<Scope>,
}

#[derive(FromForm)]
pub struct AuthForm<'a> {
    response_type: ResponseType,
    client_id: u64,
    redirect_uri: Option<&'a RawStr>,
    scope: Option<&'a RawStr>,
}
