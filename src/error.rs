use rocket::http::Status;
use rocket::request::{Outcome, Request};
use rocket::response::{self, Responder, Response};

#[allow(dead_code)]
#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    #[error("Default error")]
    Default,
    #[error("Parsing error")]
    Parsing,

    #[error("Object already exists")]
    AlreadyExists,
    #[error("Object is missing")]
    Missing,

    #[error("Database URL error")]
    DatabaseURL,
    #[error("Database connection error")]
    DatabaseConnection,
    #[error("Database pool error")]
    DatabasePool,
    #[error("Database error")]
    DatabaseError,
    #[error("Regex error")]
    RegexError,
    #[error("Time error")]
    TimeError,

    #[error("Session token missing")]
    SessionTokenMissing,
    #[error("Session token not valid")]
    SessionTokenInvalid,
    #[error("Session token expired")]
    SessionTokenExpired,

    #[error("User is missing")]
    UserMissing,
    #[error("User is disabled")]
    UserDisabled,
    #[error("User login failed")]
    UserLoginFail,
    #[error("User key is missing")]
    UserKeyMissing,
    #[error("User key has an invalid format")]
    UserKeyInvalid,
    #[error("User password is missing")]
    UserPasswordMissing,
    #[error("User password has an invalid format")]
    UserPasswordInvalid,
    #[error("User email is missing")]
    UserEmailMissing,
    #[error("User email has an invalid format")]
    UserEmailInvalid,

    #[error("Event is missing")]
    EventMissing,
    #[error("Event search has invalid criterias")]
    EventSearchLimit,
    #[error("Event owner is missing")]
    EventOwnerMissing,
    #[error("The user is not event owner")]
    EventOwnerPermission,
    #[error("Event owner is missing")]
    EventOwnerProtection,
    #[error("Event presence is missing")]
    EventPresenceForbidden,
    #[error("Event course is missing")]
    EventCourseMissing,
    #[error("Event login failed")]
    EventLoginFail,
    #[error("Event key is missing")]
    EventKeyMissing,
    #[error("Event key has an invalid format")]
    EventKeyInvalid,
    #[error("Event password is missing")]
    EventPasswordMissing,
    #[error("Event password has an invalid format")]
    EventPasswordInvalid,
    #[error("Event time window has invalid boundaries")]
    EventWindowInvalid,
    #[error("Event time window conflicts with others")]
    EventWindowConflict,
    #[error("Event status has an invalid format")]
    EventStatusInvalid,
    #[error("Event status is conflicting")]
    EventStatusConflict,

    #[error("Course is missing")]
    CourseMissing,

    #[error("Course moderator is missing")]
    CourseModeratorMissing,
    #[error("The user has insufficient course moderator permissions")]
    CourseModeratorPermission,
    #[error("Course login failed")]
    CourseLoginFail,
    #[error("Course key has an invalid format")]
    CourseKeyInvalid,

    #[error("Club is missing")]
    ClubMissing,

    #[error("Team is missing")]
    TeamMissing,

    #[error("Organisation is missing")]
    OrganisationMissing,

    #[error("Inventory stock is invalid")]
    InventoryStockInvalid,
    #[error("Inventory stock limit was reached")]
    InventoryStockLimit,
    #[error("Action does conflict with current stock values")]
    InventoryStockConflict,
    #[error("Inventory stock is missing")]
    InventoryStockMissing,
    #[error("Inventory possession is missing")]
    InventoryPossessionMissing,
    #[error("Inventory loaning has internal conflicts")]
    InventoryLoanConflict,
    #[error("Inventory transfer has internal conflicts")]
    InventoryTransferConflict,

    #[error("Conflicting permissions")]
    RightConflict,
    #[error("Club permissions are missing")]
    RightClubMissing,
    #[error("Competence permissions are missing")]
    RightCompetenceMissing,
    #[error("Course permissions are missing")]
    RightCourseMissing,
    #[error("Event permissions are missing")]
    RightEventMissing,
    #[error("Inventory permissions are missing")]
    RightInventoryMissing,
    #[error("Location permissions are missing")]
    RightLocationMissing,
    #[error("Organisation permissions are missing")]
    RightOrganisationMissing,
    #[error("Team permissions are missing")]
    RightTeamMissing,
    #[error("User permissions are missing")]
    RightUserMissing,
}

impl From<mysql::UrlError> for ErrorKind {
    fn from(_: mysql::UrlError) -> Self {
        ErrorKind::DatabaseURL
    }
}

impl From<mysql::Error> for ErrorKind {
    fn from(_: mysql::Error) -> Self {
        ErrorKind::DatabaseError
    }
}

impl From<chrono::RoundingError> for ErrorKind {
    fn from(_: chrono::RoundingError) -> Self {
        ErrorKind::TimeError
    }
}

impl<'r> Responder<'r, 'static> for ErrorKind {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .status(Status::BadRequest)
            .raw_header("error-uri", format!("{:?}", self))
            .raw_header("error-msg", self.to_string())
            .ok()
    }
}

impl ErrorKind {
    pub fn outcome<T>(self) -> Outcome<T, ErrorKind> {
        rocket::outcome::Outcome::Error((Status::BadRequest, self))
    }
}

pub type Result<T = ()> = std::result::Result<T, ErrorKind>;

#[derive(Debug)]
pub struct Error(pub anyhow::Error);

impl<E> From<E> for Error
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        Error(error.into())
    }
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, request: &Request<'_>) -> response::Result<'static> {
        response::Debug(self.0).respond_to(request)
    }
}
