/*
 * GLOBAL CONFIG FLAGS
 */

#![allow(non_snake_case)]

// Rust/chrono does not support constant contructors for Duration atm, that's why there are functions rather than static constants

pub static CONFIG_RESERVATION_AUTO_ACCEPT : bool = false;

pub fn CONFIG_SLOT_WINDOW_MINIMUM() -> chrono::Duration {chrono::Duration::minutes(15)}
pub fn CONFIG_SLOT_WINDOW_SNAP() -> chrono::Duration {chrono::Duration::minutes(15)}
