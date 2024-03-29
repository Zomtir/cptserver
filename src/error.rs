use rocket::http::Status;
use rocket::request::{Outcome, Request};
use rocket::response::{self, Responder, Response};

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    Default,

    DatabaseURL,
    DatabaseConnection,
    DatabaseError,
    RegexError,
    TimeError,

    SessionTokenMissing,
    SessionTokenInvalid,
    SessionTokenExpired,

    UserMissing,
    UserDisabled,
    UserLoginFail,
    UserKeyInvalid,
    UserPasswordInvalid,
    UserEmailInvalid,

    EventMissing,
    EventSearchLimit,
    EventOwnerMissing,
    EventOwnerPermission,
    EventCourseMissing,
    EventLoginFail,
    EventKeyInvalid,
    EventPasswordInvalid,
    EventWindowInvalid,
    EventWindowConflict,
    EventStatusInvalid,
    EventStatusConflict,

    CourseMissing,
    CourseModeratorMissing,
    CourseModeratorPermission,
    CourseLoginFail,
    CourseKeyInvalid,

    RightConflict,
    RightCompetenceMissing,
    RightCourseMissing,
    RightEventMissing,
    RightInventoryMissing,
    RightTeamMissing,
    RightTermMissing,
    RightUserMissing,
}

impl std::error::Error for Error {}

impl Error {
    fn kind(&self) -> String {
        let kind = match self {
            Error::Default => "DEFAULT",

            Error::DatabaseURL => "DATABASE_URL",
            Error::DatabaseConnection => "DATABASE_CONNECTION",
            Error::DatabaseError => "DATABASE_ERROR",
            Error::RegexError => "REGEX_ERROR",
            Error::TimeError => "TIME_ERROR",

            Error::SessionTokenMissing => "SESSION_TOKEN_MISSING",
            Error::SessionTokenInvalid => "SESSION_TOKEN_INVALID",
            Error::SessionTokenExpired => "SESSION_TOKEN_EXPIRED",

            Error::UserMissing => "USER_MISSING",
            Error::UserDisabled => "USER_DISABLED",
            Error::UserLoginFail => "USER_LOGIN_FAIL",
            Error::UserKeyInvalid => "USER_KEY_INVALID",
            Error::UserPasswordInvalid => "USER_PASSWORD_INVALID",
            Error::UserEmailInvalid => "USER_EMAIL_INVALID",

            Error::EventMissing => "SLOT_MISSING",
            Error::EventSearchLimit => "SLOT_SEARCH_LIMIT",
            Error::EventOwnerMissing => "SLOT_OWNER_MISSING",
            Error::EventOwnerPermission => "SLOT_OWNER_PERMISSION",
            Error::EventCourseMissing => "SLOT_COURSE_MISSING",
            Error::EventLoginFail => "SLOT_LOGIN_FAIL",
            Error::EventKeyInvalid => "SLOT_KEY_INVALID",
            Error::EventPasswordInvalid => "SLOT_PASSWORD_INVALID",
            Error::EventWindowInvalid => "SLOT_WINDOW_INVALID",
            Error::EventWindowConflict => "SLOT_WINDOW_CONFLICT",
            Error::EventStatusInvalid => "SLOT_STATUS_INVALID",
            Error::EventStatusConflict => "SLOT_STATUS_CONFLICT",

            Error::CourseMissing => "COURSE_MISSING",
            Error::CourseModeratorMissing => "COURSE_MODERATOR_MISSING",
            Error::CourseModeratorPermission => "COURSE_MODERATOR_PERMISSION",
            Error::CourseLoginFail => "COURSE_LOGIN_FAIL",
            Error::CourseKeyInvalid => "COURSE_KEY_INVALID",

            Error::RightConflict => "RIGHT_CONFLICT",
            Error::RightCompetenceMissing => "RIGHT_COMPETENCE_MISSING",
            Error::RightCourseMissing => "RIGHT_COURSE_MISSING",
            Error::RightEventMissing => "RIGHT_EVENT_MISSING",
            Error::RightInventoryMissing => "RIGHT_INVENTORY_MISSING",
            Error::RightTeamMissing => "RIGHT_TEAM_MISSING",
            Error::RightTermMissing => "RIGHT_TERM_MISSING",
            Error::RightUserMissing => "RIGHT_USER_MISSING",
        };
        kind.into()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Default => write!(f, "Default error"),

            Error::DatabaseURL => write!(f, "Database URL error"),
            Error::DatabaseConnection => write!(f, "Database connection error"),
            Error::DatabaseError => write!(f, "Database error"),
            Error::RegexError => write!(f, "Regex error"),
            Error::TimeError => write!(f, "Time error"),

            Error::SessionTokenMissing => write!(f, "Session token missing"),
            Error::SessionTokenInvalid => write!(f, "Session token not valid"),
            Error::SessionTokenExpired => write!(f, "Session token expired"),

            Error::UserMissing => write!(f, "User is missing"),
            Error::UserDisabled => write!(f, "User is disabled"),
            Error::UserLoginFail => write!(f, "User login failed"),
            Error::UserKeyInvalid => write!(f, "User key has an invalid format"),
            Error::UserPasswordInvalid => write!(f, "User password has an invalid format"),
            Error::UserEmailInvalid => write!(f, "User email has an invalid format"),

            Error::EventMissing => write!(f, "Event is missing"),
            Error::EventSearchLimit => write!(f, "Event search has invalid criterias"),
            Error::EventOwnerMissing => write!(f, "Slow owner is missing"),
            Error::EventOwnerPermission => write!(f, "The user has insufficient event owner permissions"),
            Error::EventCourseMissing => write!(f, "Slow course is missing"),
            Error::EventLoginFail => write!(f, "Event login failed"),
            Error::EventKeyInvalid => write!(f, "Slow key has an invalid format"),
            Error::EventPasswordInvalid => write!(f, "Event password has an invalid format"),
            Error::EventWindowInvalid => write!(f, "Event time windows have invalid boundaries"),
            Error::EventWindowConflict => write!(f, "Event time window conflicts with others"),
            Error::EventStatusInvalid => write!(f, "Event status has an invalid format"),
            Error::EventStatusConflict => write!(f, "Event status is conflicting"),

            Error::CourseMissing => write!(f, "Course is missing"),
            Error::CourseModeratorMissing => write!(f, "Course moderator is missing"),
            Error::CourseModeratorPermission => write!(f, "The user has insufficient course moderator permissions"),
            Error::CourseLoginFail => write!(f, "Course login failed"),
            Error::CourseKeyInvalid => write!(f, "Course key has an invalid format"),

            Error::RightConflict => write!(f, "Conflicting right permissions"),
            Error::RightCompetenceMissing => write!(f, "Competence permissions are missing"),
            Error::RightCourseMissing => write!(f, "Course permissions are missing"),
            Error::RightEventMissing => write!(f, "Event permissions are missing"),
            Error::RightInventoryMissing => write!(f, "Inventory permissions are missing"),
            Error::RightTeamMissing => write!(f, "Team permissions are missing"),
            Error::RightTermMissing => write!(f, "Term permissions are missing"),
            Error::RightUserMissing => write!(f, "User permissions are missing"),
        }
    }
}

impl From<mysql::UrlError> for Error {
    fn from(_: mysql::UrlError) -> Self {
        Error::DatabaseURL
    }
}

impl From<mysql::Error> for Error {
    fn from(_: mysql::Error) -> Self {
        Error::DatabaseError
    }
}

impl From<chrono::RoundingError> for Error {
    fn from(_: chrono::RoundingError) -> Self {
        Error::TimeError
    }
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .status(Status::BadRequest)
            .raw_header("error-uri", self.kind())
            .raw_header("error-msg", self.to_string())
            .ok()
    }
}

impl Error {
    pub fn outcome<T>(self) -> Outcome<T, Error> {
        rocket::outcome::Outcome::Error((Status::BadRequest, self))
    }
}
