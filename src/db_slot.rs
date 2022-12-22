use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use crate::db::get_pool_conn;
use crate::api::ApiError;
use crate::common::{Location, Slot, User};

/*
 * METHODS
 */

pub fn get_slot_info(slot_id : & u32) -> Option<Slot> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT slot_id, slot_key, s.title, l.location_id, l.location_key, l.title, s.begin, s.end, s.status, s.course_id
                          FROM slots s
                          JOIN locations l ON l.location_id = s.location_id
                          WHERE slot_id = :slot_id").unwrap();

    let params = params! { "slot_id" => slot_id };
    let map =
        | (slot_id, slot_key, slot_title, location_id, location_key, location_title, begin, end, status, course_id)
        : (u32, _, _, u32, _, _, _, _, String, Option<u32>)
        | Slot {
            id: slot_id, key: slot_key, pwd: None, title: slot_title, begin, end, status: Some(status),
            location: Location {id: location_id, key: location_key, title: location_title},
            course_id: course_id, owners: None};

    let mut slot : Slot = match conn.exec_map(&stmt, &params, &map) {
        Err(..) => return None,
        Ok(mut slots) => slots.remove(0),
    };

    slot.owners = get_slot_owners(slot_id);

    return Some(slot);
}

pub fn get_slot_owners(slot_id : & u32) -> Option<Vec<User>> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT u.user_id, u.user_key, u.firstname, u.lastname
                          FROM slot_owners
                          JOIN users u ON u.user_id = slot_owners.user_id
                          WHERE slot_owners.slot_id = :slot_id").unwrap();

    let params = params! { "slot_id" => slot_id };
    let map =
        | (user_id, user_key, firstname, lastname)
        : (u32, String, String, String)
        | User::from_info(user_id, user_key, firstname, lastname);

    match conn.exec_map(&stmt, &params, &map) {
        Err(..) => return None,
        Ok(members) => return Some(members),
    }
}

pub fn is_slot_owner(slot_id : & u32, user_id : & u32) -> Option<bool> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT COUNT(1)
                          FROM slot_owners
                          WHERE slot_id = :slot_id AND user_id = :user_id").unwrap();

    let params = params! {
        "slot_id" => slot_id,
        "user_id" => user_id,
    };

    match conn.exec_first::<u32,_,_>(&stmt, &params){
        Err(..) => return None,
        Ok(None) => return Some(false),
        Ok(Some(count)) => return Some(count == 1),
    };
}

pub fn is_slot_valid(slot: & Slot) -> bool {
    return slot.begin + crate::config::CONFIG_SLOT_WINDOW_MINIMUM() < slot.end;
}

// Perhaps the database should be locked between checking for a free slot and modifying the slot later
pub fn is_slot_free(slot: & Slot) -> Option<bool> {
    if !is_slot_valid(slot) {return Some(false)};

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT COUNT(1) FROM slots
                          WHERE location_id = :location_id
                          AND NOT (end <= :begin OR begin >= :end)
                          AND status = 'OCCURRING'").unwrap();
    
    let params = params! {
        "location_id" => &slot.location.id,
        "begin" => &slot.begin,
        "end" => &slot.end,
    };

    match conn.exec_first::<u32,_,_>(&stmt, &params){
        Err(..) => return None,
        Ok(None) => return None,
        Ok(Some(count)) => return Some(count == 0),
    };
}

pub fn set_slot_status(slot_id : u32, status_required : &str, status_update : &str) -> Option<()> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("UPDATE slots SET
        status = :status_update
        WHERE slot_id = :slot_id AND status = :status_required").unwrap();
    let params = params! {
        "slot_id" => slot_id,
        "status_required" => status_required,
        "status_update" => status_update,
    };

    match conn.exec_drop(&stmt,&params){
        Err(..) => return None,
        Ok(..) => return Some(()),
    };
}

pub fn add_slot_owner(slot_id : u32, user_id : u32) -> Result<(), ApiError> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("INSERT INTO slot_owners (slot_id, user_id)
                          VALUES (:slot_id, :user_id)").unwrap();
    
    let params = params! {
        "slot_id" => &slot_id,
        "user_id" => &user_id,
    };
    
    match conn.exec_drop(&stmt,&params) {
        Err(..) => return Err(ApiError::DB_CONFLICT),
        Ok(..) => return Ok(()),
    };
}

pub fn remove_slot_owner(slot_id : u32, user_id : u32) -> Result<(), ApiError> {    
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("
        DELETE FROM slot_owners
        WHERE slot_id = :slot_id AND user_id = :user_id").unwrap();

    let params = params! {
        "slot_id" => &slot_id,
        "user_id" => &user_id,
    };

    match conn.exec_drop(&stmt,&params) {
        Err(..) => return Err(ApiError::DB_CONFLICT),
        Ok(..) => return Ok(()),
    };
}