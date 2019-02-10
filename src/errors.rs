use std::convert::From;
use std::error;
use std::fmt;
use bcrypt::BcryptError;
use diesel::result::Error as DieselError;

/// Library generic result type.
pub type APIResult<T> = Result<T, APIError>;

#[derive(Debug)]
pub enum APIError {
    PersonNotFound(i32),
    Bcrypt(BcryptError),
    Diesel(DieselError),
}

macro_rules! impl_from_error {
    ($f: ty, $e: expr) => {
        impl From<$f> for APIError {
            fn from(f: $f) -> Self {
                $e(f)
            }
        }
    };
}

impl_from_error!(BcryptError, APIError::Bcrypt);
impl_from_error!(DieselError, APIError::Diesel);

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            APIError::Bcrypt(ref err) => write!(f, "BCrypt: {}", err),
            APIError::Diesel(ref err) => write!(f, "Diesel: {}", err),
        }
    }
}

impl error::Error for APIError {
    fn description(&self) -> &str {
        match self {
            APIError::Bcrypt(ref err) => err.description(),
            APIError::Diesel(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            APIError::Bcrypt(ref err) => Some(err),
            APIError::Diesel(ref err) => Some(err),
        }
    }
}
