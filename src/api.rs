use rocket::request::{Request, Outcome};
use rocket::response::{self, Response, Responder};
use rocket::outcome::Outcome::{Failure};
use rocket::http::Status;

#[derive(Debug)]
pub struct ApiError {
    // the unique identifier of this error type (URI)
    uri: &'static str,
    // the HTTP response status code
    status_code: u16,
    // the origin of the error (URL)
    //origin: String,
    // the verbatim name of the error (URN)
    //name: String,
    // human-readable error information
    message: &'static str,
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .status(match Status::from_code(self.status_code) { Some(status) => status, None => Status::InternalServerError} )
            .raw_header("Error-URI", self.uri)
            .raw_header("Error-Message", self.message)
            .ok()
    }
}

impl ApiError {
    pub fn outcome<T>(self) -> Outcome<T,ApiError> {
        Failure((Status::from_code(self.status_code).unwrap(),self))
    }
}

macro_rules! ctrs {
    ($($uri:ident => $code:expr, $message:expr),+) => {
        $(
            pub const $uri: ApiError = ApiError { uri: stringify!($uri), status_code: $code, message: $message };
        )+
    };
}

impl ApiError {

    ctrs! {
        USER_NO_ENTRY => 400, "This user has no entry in the database.",
        USER_BAD_PASSWORD => 400, "Password has either invalid formatting or is not belonging to this user.",
        USER_DISABLED => 400, "The user account is disabled.",
        SLOT_NO_ENTRY => 400, "This slot has no entry in the database.",
        SLOT_BAD_PASSWORD => 400, "This password does not belong to given slot.",
        SLOT_BAD_TIME => 400, "Time window too narrow or negative.",
        SLOT_OVERLAP_TIME => 409, "Time window overlaps with an existing slot.",
        SLOT_STATUS_INCOMPAT => 400, "Slot status is incompatible.",

        SESSION_TOKEN_MISSING => 400, "Header token missing.",
        SESSION_TOKEN_INVALID => 400, "Header token not valid.",
        SESSION_TOKEN_EXPIRED => 403, "Header token expired.",

        RIGHT_CONFLICT => 403, "You tried to access or edit some resource that you were not supposed to.",
        RIGHT_NO_RESERVATIONS => 403, "You do not have rights to edit reservations.",

        DB_CONFLICT => 409, "The database query failed. Might still be your fault because you didn't refresh."
    }
}
