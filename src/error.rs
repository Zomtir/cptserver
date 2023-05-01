use rocket::http::Status;
use rocket::outcome::Outcome::Failure;
use rocket::request::{Outcome, Request};
use rocket::response::{self, Responder, Response};

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

    SlotMissing,
    SlotOwnerMissing,
    SlotOwnerPermission,
    SlotCourseMissing,
    SlotLoginFail,
    SlotKeyInvalid,
    SlotPasswordInvalid,
    SlotWindowInvalid,
    SlotWindowConflict,
    SlotStatusInvalid,
    SlotStatusConflict,

    CourseMissing,
    CourseModeratorMissing,
    CourseModeratorPermission,
    CourseLoginFail,
    CourseKeyInvalid,

    RightConflict,
    RightCourseMissing,
    RightEventMissing,
    RightInventoryMissing,
    RightRankingMissing,
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

            Error::SlotMissing => "SLOT_MISSING",
            Error::SlotOwnerMissing => "SLOT_OWNER_MISSING",
            Error::SlotOwnerPermission => "SLOT_OWNER_PERMISSION",
            Error::SlotCourseMissing => "SLOT_COURSE_MISSING",
            Error::SlotLoginFail => "SLOT_LOGIN_FAIL",
            Error::SlotKeyInvalid => "SLOT_KEY_INVALID",
            Error::SlotPasswordInvalid => "SLOT_PASSWORD_INVALID",
            Error::SlotWindowInvalid => "SLOT_WINDOW_INVALID",
            Error::SlotWindowConflict => "SLOT_WINDOW_CONFLICT",
            Error::SlotStatusInvalid => "SLOT_STATUS_INVALID",
            Error::SlotStatusConflict => "SLOT_STATUS_CONFLICT",

            Error::CourseMissing => "COURSE_MISSING",
            Error::CourseModeratorMissing => "COURSE_MODERATOR_MISSING",
            Error::CourseModeratorPermission => "COURSE_MODERATOR_PERMISSION",
            Error::CourseLoginFail => "COURSE_LOGIN_FAIL",
            Error::CourseKeyInvalid => "COURSE_KEY_INVALID",

            Error::RightConflict => "RIGHT_CONFLICT",
            Error::RightCourseMissing => "RIGHT_COURSE_MISSING",
            Error::RightEventMissing => "RIGHT_EVENT_MISSING",
            Error::RightInventoryMissing => "RIGHT_INVENTORY_MISSING",
            Error::RightRankingMissing => "RIGHT_RANKING_MISSING",
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

            Error::SlotMissing => write!(f, "Slot is missing"),
            Error::SlotOwnerMissing => write!(f, "Slow owner is missing"),
            Error::SlotOwnerPermission => write!(f, "The user has insufficient slot owner permissions"),
            Error::SlotCourseMissing => write!(f, "Slow course is missing"),
            Error::SlotLoginFail => write!(f, "Slot login failed"),
            Error::SlotKeyInvalid => write!(f, "Slow key has an invalid format"),
            Error::SlotPasswordInvalid => write!(f, "Slot password has an invalid format"),
            Error::SlotWindowInvalid => write!(f, "Slot time windows have invalid boundaries"),
            Error::SlotWindowConflict => write!(f, "Slot time window conflicts with others"),
            Error::SlotStatusInvalid => write!(f, "Slot status has an invalid format"),
            Error::SlotStatusConflict => write!(f, "Slot status is conflicting"),

            Error::CourseMissing => write!(f, "Course is missing"),
            Error::CourseModeratorMissing => write!(f, "Course moderator is missing"),
            Error::CourseModeratorPermission => write!(f, "The user has insufficient course moderator permissions"),
            Error::CourseLoginFail => write!(f, "Course login failed"),
            Error::CourseKeyInvalid => write!(f, "Course key has an invalid format"),

            Error::RightConflict => write!(f, "Conflicting right permissions"),
            Error::RightCourseMissing => write!(f, "Course permissions are missing"),
            Error::RightEventMissing => write!(f, "Event permissions are missing"),
            Error::RightInventoryMissing => write!(f, "Inventory permissions are missing"),
            Error::RightRankingMissing => write!(f, "Ranking permissions are missing"),
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
        Failure((Status::BadRequest, self))
    }
}
