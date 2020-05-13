use serde_derive::{Serialize, Deserialize};
use url::Url;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;

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