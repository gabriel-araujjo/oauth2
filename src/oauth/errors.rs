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
