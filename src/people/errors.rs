use bcrypt::BcryptError;
use diesel::result::{Error as DieselError, DatabaseErrorKind, DatabaseErrorInformation};
use std::convert::From;
use std::fmt;
use uuid::Uuid;

pub type PeopleResult<T> = Result<T, PeopleError>;

#[derive(Debug)]
pub enum PeopleError {
    // When there is no person registered with this id
    NotFound(Uuid),
    // When the password has a null character 
    InvalidPassword,
    // When trying insert or update a person email and
    // another person has this email
    DuplicateEmail,
    // When person credentials are invalid or do not match
    AuthenticationFail,
    // When an Unexpected Diesel error occur
    DieselError(DieselError),
    // When some unexpected Bcryt error occur
    BcryptError(BcryptError),
}

impl From<BcryptError> for PeopleError {
    fn from(e: BcryptError) -> Self {
        match e {
            BcryptError::InvalidPassword => PeopleError::InvalidPassword,
            _ => PeopleError::BcryptError(e),
        }
    }
}

impl From<DieselError> for PeopleError {
    fn from(e: DieselError) -> Self {
        // FIXME: find a way to remove this arrow
        match e {
            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, info) => {
                match info.constraint_name() {
                    Some("people_email_unique") => PeopleError::DuplicateEmail,
                    _ => PeopleError::DieselError(e),
                }
            },
            NotFound => PeopleError::NotFound,
            _ => PeopleError::DieselError(e),
        }
    }
}

impl fmt::Display for PeopleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PeopleError::NotFound(id) => write!(f, "Person with id {} not found", id),
            PeopleError::InvalidPassword => write!(f, "Password contain a forbidden null character (\\0)"),
            PeopleError::DuplicateEmail => write!(f, "Duplicate email"),
            PeopleError::AuthenticationFail => write!(f, "Authentication fail"),
            PeopleError::DieselError(err) => write!(f, "Diesel: {}", err),
            PeopleError::BcryptError(err) => write!(f, "Bcrypt: {}", err),
        }
    }
}
