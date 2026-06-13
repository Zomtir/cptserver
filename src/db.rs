pub mod club;
pub mod competence;
pub mod course;
pub mod event;
pub mod inventory;
pub mod location;
pub mod login;
pub mod migrate;
pub mod organisation;
pub mod skill;
pub mod team;
pub mod user;

pub use migrate::*;

static SCHEME_VERSION: u8 = 3;
