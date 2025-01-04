use rocket::http::Status;
use rocket::request::{Outcome, Request};
use rocket::response::{self, Responder, Response};

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    Default,

    DatabaseURL,
    DatabaseConnection,
    DatabasePool,
    DatabaseError,
    RegexError,
    TimeError,

    SessionTokenMissing,
    SessionTokenInvalid,
    SessionTokenExpired,

    UserMissing,
    UserDisabled,
    UserLoginFail,
    UserKeyMissing,
    UserKeyInvalid,
    UserPasswordInvalid,
    UserEmailMissing,
    UserEmailInvalid,

    EventMissing,
    EventSearchLimit,
    EventOwnerMissing,
    EventOwnerPermission,
    EventOwnerProtection,
    EventPresenceForbidden,
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

    ClubMissing,

    InventoryStockInvalid,
    InventoryStockLimit,
    InventoryStockConflict,
    InventoryStockMissing,
    InventoryPossessionMissing,
    InventoryLoanConflict,
    InventoryTransferConflict,

    RightConflict,
    RightClubMissing,
    RightCompetenceMissing,
    RightCourseMissing,
    RightEventMissing,
    RightInventoryMissing,
    RightLocationMissing,
    RightOrganisationMissing,
    RightTeamMissing,
    RightUserMissing,
}

impl std::error::Error for Error {}

impl Error {
    fn kind(&self) -> String {
        let kind = match self {
            Error::Default => "DEFAULT",

            Error::DatabaseURL => "DATABASE_URL",
            Error::DatabaseConnection => "DATABASE_CONNECTION",
            Error::DatabasePool => "DATABASE_POOL",
            Error::DatabaseError => "DATABASE_ERROR",
            Error::RegexError => "REGEX_ERROR",
            Error::TimeError => "TIME_ERROR",

            Error::SessionTokenMissing => "SESSION_TOKEN_MISSING",
            Error::SessionTokenInvalid => "SESSION_TOKEN_INVALID",
            Error::SessionTokenExpired => "SESSION_TOKEN_EXPIRED",

            Error::UserMissing => "USER_MISSING",
            Error::UserDisabled => "USER_DISABLED",
            Error::UserLoginFail => "USER_LOGIN_FAIL",
            Error::UserKeyMissing => "USER_KEY_MISSING",
            Error::UserKeyInvalid => "USER_KEY_INVALID",
            Error::UserPasswordInvalid => "USER_PASSWORD_INVALID",
            Error::UserEmailMissing => "USER_EMAIL_MISSING",
            Error::UserEmailInvalid => "USER_EMAIL_INVALID",

            Error::EventMissing => "SLOT_MISSING",
            Error::EventSearchLimit => "SLOT_SEARCH_LIMIT",
            Error::EventOwnerMissing => "SLOT_OWNER_MISSING",
            Error::EventOwnerPermission => "SLOT_OWNER_PERMISSION",
            Error::EventOwnerProtection => "SLOT_OWNER_PROTECTION",
            Error::EventPresenceForbidden => "SLOT_PRESENCE_FORBIDDEN",
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

            Error::ClubMissing => "CLUB_MISSING",

            Error::InventoryStockInvalid => "INVENTORY_STOCK_INVALID",
            Error::InventoryStockLimit => "INVENTORY_STOCK_LIMIT",
            Error::InventoryStockConflict => "INVENTORY_STOCK_CONFLICT",
            Error::InventoryStockMissing => "INVENTORY_STOCK_MISSING",
            Error::InventoryPossessionMissing => "INVENTORY_POSSESSION_MISSING",
            Error::InventoryLoanConflict => "INVENTORY_LOAN_CONFLICT",
            Error::InventoryTransferConflict => "INVENTORY_TRANSFER_CONFLICT",

            Error::RightConflict => "RIGHT_CONFLICT",
            Error::RightClubMissing => "RIGHT_TERM_MISSING",
            Error::RightCompetenceMissing => "RIGHT_COMPETENCE_MISSING",
            Error::RightCourseMissing => "RIGHT_COURSE_MISSING",
            Error::RightEventMissing => "RIGHT_EVENT_MISSING",
            Error::RightInventoryMissing => "RIGHT_INVENTORY_MISSING",
            Error::RightLocationMissing => "RIGHT_LOCATION_MISSING",
            Error::RightOrganisationMissing => "RIGHT_ORGANISATION_MISSING",
            Error::RightTeamMissing => "RIGHT_TEAM_MISSING",
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
            Error::DatabasePool => write!(f, "Database pool error"),
            Error::DatabaseError => write!(f, "Database error"),
            Error::RegexError => write!(f, "Regex error"),
            Error::TimeError => write!(f, "Time error"),

            Error::SessionTokenMissing => write!(f, "Session token missing"),
            Error::SessionTokenInvalid => write!(f, "Session token not valid"),
            Error::SessionTokenExpired => write!(f, "Session token expired"),

            Error::UserMissing => write!(f, "User is missing"),
            Error::UserDisabled => write!(f, "User is disabled"),
            Error::UserLoginFail => write!(f, "User login failed"),
            Error::UserKeyMissing => write!(f, "User key is missing"),
            Error::UserKeyInvalid => write!(f, "User key has an invalid format"),
            Error::UserPasswordInvalid => write!(f, "User password has an invalid format"),
            Error::UserEmailMissing => write!(f, "User email is missing"),
            Error::UserEmailInvalid => write!(f, "User email has an invalid format"),

            Error::EventMissing => write!(f, "Event is missing"),
            Error::EventSearchLimit => write!(f, "Event search has invalid criterias"),
            Error::EventOwnerMissing => write!(f, "Event owner is missing"),
            Error::EventOwnerPermission => write!(f, "The user is not event owner"),
            Error::EventOwnerProtection => write!(f, "A user cannot remove oneself as event owner"),
            Error::EventPresenceForbidden => write!(f, "The user is not part of the presence pool"),
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

            Error::ClubMissing => write!(f, "Club is missing"),

            Error::InventoryStockInvalid => write!(f, "Current inventory stock is invalid"),
            Error::InventoryStockLimit => write!(f, "Inventory stock limit was reached"),
            Error::InventoryStockConflict => write!(f, "Action does conflict with current stock values"),
            Error::InventoryStockMissing => write!(f, "Inventory stock is missing"),
            Error::InventoryPossessionMissing => write!(f, "Inventory possession is missing"),
            Error::InventoryLoanConflict => write!(f, "Inventory loaning has internal conflicts"),
            Error::InventoryTransferConflict => write!(f, "Inventory transfer has internal conflicts"),

            Error::RightConflict => write!(f, "Conflicting right permissions"),
            Error::RightClubMissing => write!(f, "Club permissions are missing"),
            Error::RightCompetenceMissing => write!(f, "Competence permissions are missing"),
            Error::RightCourseMissing => write!(f, "Course permissions are missing"),
            Error::RightEventMissing => write!(f, "Event permissions are missing"),
            Error::RightInventoryMissing => write!(f, "Inventory permissions are missing"),
            Error::RightLocationMissing => write!(f, "Location permissions are missing"),
            Error::RightOrganisationMissing => write!(f, "Organisation permissions are missing"),
            Error::RightTeamMissing => write!(f, "Team permissions are missing"),
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
