
#[derive(Debug)]
pub enum CptError {
    RegexError,
    DbError,
    UserMissing,
    UserKeyMissing,
    UserKeyBad,
    UserEmailBad,
}

impl std::error::Error for CptError {}

impl std::fmt::Display for CptError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CptError::RegexError => write!(f, "Regex error"),
            CptError::DbError => write!(f, "Database error"),
            CptError::UserMissing => write!(f, "User is missing"),
            CptError::UserKeyMissing => write!(f, "User key is missing"),
            CptError::UserKeyBad => write!(f, "User key has bad format"),
            CptError::UserEmailBad => write!(f, "User email has bad format"),
        }
    }
}

impl From<mysql::Error> for CptError {
    fn from(_: mysql::Error) -> Self {
        CptError::DbError
    }
}
