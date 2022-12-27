use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use chrono::NaiveDate;

use crate::db::get_pool_conn;
use crate::common::{Slot, Location};

/*
 * METHODS
 */

// TODO make a check that status is not an invalid string by implementing a proper trait
// status shouldn't even be a searchable criteria? if so, please make it enum with FromFormValue::default()
pub fn get_event_list(
    begin: NaiveDate,
    end: NaiveDate,
    status: Option<String>,
    owner_id: Option<u32>,
) -> Option<Vec<Slot>> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT s.slot_id, s.slot_key, s.title, l.location_id, l.location_key, l.title, s.begin, s.end, s.status
                          FROM slots s
                          JOIN locations l ON l.location_id = s.location_id
                          LEFT JOIN slot_owners o ON s.slot_id = o.slot_id
                          WHERE s.begin > :frame_start
                          AND s.begin < :frame_stop
                          AND (:status IS NULL OR :status = s.status)
                          AND (:owner_id IS NULL OR :owner_id = o.user_id)
                          GROUP BY s.slot_id").unwrap();

    let params = params! {
        "frame_start" => &begin,
        "frame_stop" => &end,
        "status" => &status,
        "owner_id" => &owner_id,
    };

    let map = |(
        slot_id,
        slot_key,
        slot_title,
        location_id,
        location_key,
        location_title,
        begin,
        end,
        status,
    ): (i64, String, String, u32, _, _, _, _, String)| Slot {
        id: slot_id,
        key: slot_key,
        pwd: None,
        title: slot_title,
        begin,
        end,
        status: Some(status),
        location: Location {
            id: location_id,
            key: location_key,
            title: location_title,
        },
        course_id: None,
        owners: None,
    };

    match conn.exec_map(&stmt, &params, &map) {
        Err(..) => None,
        Ok(slots) => Some(slots),
    }
}

pub fn create_event(slot : &Slot) -> Option<i64> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("INSERT INTO slots (slot_key, pwd, title, location_id, begin, end, status)
                          VALUES (:slot_key, :pwd, :title, :location_id, :begin, :end, :status)").unwrap();

    let params = params! {
        "slot_key" => crate::common::random_string(8),
        "pwd" => crate::common::random_string(8),
        "title" => &slot.title,
        "location_id" => &slot.location.id,
        "begin" => &slot.begin,
        "end" => &slot.end,
        "status" => "DRAFT",
    };

    match conn.exec_drop(&stmt,&params) {
        Err(..) => return None,
        Ok(..) => (),
    };
    
    let stmt_id = conn.prep("SELECT LAST_INSERT_ID()").unwrap();

    match conn.exec_first::<i64,_,_>(&stmt_id,params::Params::Empty) {
        Err(..) | Ok(None) => None,
        Ok(Some(slot_id)) => Some(slot_id),
    }
}


pub fn edit_event(slot: &Slot) -> Option<()> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("UPDATE slots SET
        title = :title,
        location_id = :location_id,
        begin = :begin,
        end = :end,
        status = 'DRAFT'
        WHERE slot_id = :slot_id
        AND (status = 'DRAFT' OR status = 'REJECTED' OR status = 'CANCELED')").unwrap();

    let params = params! {
        "slot_id" => &slot.id,
        "title" => &slot.title,
        "location_id" => &slot.location.id,
        "begin" => &slot.begin,
        "end" => &slot.end,
    };

    match conn.exec_drop(&stmt,&params) {
        Err(..) => return None,
        Ok(..) => (),
    };

    // TODO, set the password with a seperate call and not plain-text in the JSON
    if slot.pwd.is_none() || slot.pwd.as_ref().unwrap().len() < 8 {
        return Some(());
    };

    let stmt_pwd = conn.prep("UPDATE slots SET pwd = :pwd WHERE slot_id = :slot_id").unwrap();
    let params_pwd = params! {
        "slot_id" => &slot.id,
        "pwd" => &slot.pwd.as_ref().unwrap(),
    };

    match conn.exec_drop(&stmt_pwd,&params_pwd) {
        Err(..) => None,
        Ok(..) => Some(()),
    }
}

pub fn delete_event(slot_id : i64) -> Option<()> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE s FROM slots s
                          WHERE slot_id = :slot_id").unwrap();

    let params = params! {
        "slot_id" => &slot_id,
    };

    match conn.exec_drop(&stmt,&params) {
        Err(..) => None,
        Ok(..) => Some(()),
    }
}