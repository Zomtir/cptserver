use rocket::request::Request;
use rocket::response::{self, Response, Responder};
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

impl<'a> Responder<'a> for ApiError {
    fn respond_to(self, _: &Request) -> response::Result<'a> {
        Response::build()
            .status(match Status::from_code(self.status_code) { Some(status) => status, None => Status::InternalServerError} )
            .raw_header("Error-URI", self.uri)
            .raw_header("Error-Message", self.message)
            .ok()
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
        NO_USER_ENTRY => 400, "There is no such user entry in the database.",
        BAD_USER_PASSWORD => 400, "Password has either invalid formatting or is not belonging to this user.",
        USER_EXPIRED => 400, "User account is expired. The term ended at a point in the past time.",
        USER_DISABLED => 400, "The user account is disabled. Check your membership status."
    }
}
