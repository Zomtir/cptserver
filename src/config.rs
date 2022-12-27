/*
 * GLOBAL CONFIG FLAGS
 */

#![allow(non_snake_case)]

// Rust/chrono does not support constant contructors for Duration atm, that's why there are functions rather than static constants

pub static CONFIG_RESERVATION_AUTO_CHECK : bool = false;
pub static CONFIG_COURSE_MODERATOR_PROMOTION : bool = true;

pub static CONFIG_SLOT_WINDOW_DAY_MIN : i64 = 1;
pub static CONFIG_SLOT_WINDOW_DAY_MAX : i64 = 366;

pub fn CONFIG_SLOT_WINDOW_MINIMUM() -> chrono::Duration {chrono::Duration::minutes(15)}
pub fn CONFIG_SLOT_WINDOW_SNAP() -> chrono::Duration {chrono::Duration::minutes(15)}
