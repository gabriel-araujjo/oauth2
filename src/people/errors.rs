use bcrypt::BcryptError;
use diesel::result::{Error as DieselError, DatabaseErrorKind};
use std::convert::{From, Into};
use std::fmt;
use rocket::http::Status;

pub type PeopleResult<T> = Result<T, PeopleError>;

#[derive(Debug)]
pub enum PeopleError {
    // When there is no person registered with this id
    NotFound,
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
            DieselError::NotFound => PeopleError::NotFound,
            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, ref info) => {
                match info.constraint_name() {
                    Some("people_email_unique") => PeopleError::DuplicateEmail,
                    _ => PeopleError::DieselError(e),
                }
            },
            _ => PeopleError::DieselError(e),
        }
    }
}

impl fmt::Display for PeopleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PeopleError::NotFound => write!(f, "Person not found"),
            PeopleError::InvalidPassword => write!(f, "Password contain a forbidden null character (\\0)"),
            PeopleError::DuplicateEmail => write!(f, "Duplicate email"),
            PeopleError::AuthenticationFail => write!(f, "Authentication fail"),
            PeopleError::DieselError(err) => write!(f, "Diesel: {}", err),
            PeopleError::BcryptError(err) => write!(f, "Bcrypt: {}", err),
        }
    }
}

impl Into<Status> for PeopleError {
    fn into(self) -> Status {
        match self {
            PeopleError::NotFound => Status::NotFound,
            PeopleError::InvalidPassword => Status::new(400, "Invalid password (password has a \\0 character)"),
            PeopleError::DuplicateEmail => Status::new(400, "Duplicated email"),
            PeopleError::AuthenticationFail => Status::Forbidden,
            _ => Status::InternalServerError,
        }
    }
}
