use rocket::http::Status;
use rocket::request::{Outcome, Request};
use rocket::response::{self, Responder, Response};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    // General error
    Default,
    // Diverging information between two sources that should be identical
    Mismatch,
    // Logical conflicts like loaning an item out twice or returning an item that is not loaned out
    Conflict,
    // Trying an action that is not allowed in the current state
    Disabled,
    // Not finding an expected item or record
    Missing,
    // Finding multiple items or records where only one was expected
    Duplicate,
    // Invalid values or states, mostly inputs
    Invalid,
    // Overlapping ranges
    Overlap,
    // Range exceeding a boundary or a value exceeding a limit
    Boundary,
    // Permission errors like trying to access or modify a resource without the necessary permissions
    Permission,
    // Protected resources since it would violate integrity, like deleting an event with active sessions or removing oneself as owner of an event
    Protected,
    // Parsing errors like invalid input formats
    Parsing,
    // Using an invalid regex pattern
    Regex,
    // Login and session related errors like invalid credentials
    Authentication,
    // Expired tokens or cooldowns and throttles
    Expired,
    // Technical database errors
    Database,
    // Filesystem errors
    Filesystem,
    // Time and time zone related issues
    Time,
}

#[derive(thiserror::Error, Debug)]
#[error("{message}")]
pub struct Error {
    pub kind: ErrorKind,

    pub message: String,

    #[source]
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl Error {
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source<E>(kind: ErrorKind, message: impl Into<String>, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self {
            kind,
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }
}

impl From<mysql::UrlError> for Error {
    fn from(_: mysql::UrlError) -> Self {
        Error::new(ErrorKind::Database, "Database URL error")
    }
}

impl From<mysql::Error> for Error {
    fn from(_: mysql::Error) -> Self {
        Error::new(ErrorKind::Database, "Generic Database error")
    }
}

impl From<chrono::RoundingError> for Error {
    fn from(_: chrono::RoundingError) -> Self {
        Error::new(ErrorKind::Time, "Time rounding error")
    }
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .status(Status::BadRequest)
            .raw_header("error-uri", format!("{:?}", self.kind))
            .raw_header("error-msg", self.message)
            .ok()
    }
}

impl Error {
    pub fn outcome<T>(self) -> Outcome<T, Error> {
        rocket::outcome::Outcome::Error((Status::BadRequest, self))
    }
}

pub type Result<T = ()> = std::result::Result<T, Error>;
