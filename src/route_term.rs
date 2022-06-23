// let user_term : chrono::NaiveDate = row.take("term").unwrap();
// if chrono::Date::<chrono::Utc>::from_utc(user_term, chrono::Utc) < chrono::Utc::today() {
//     return Err(ApiError::USER_EXPIRED);
// }

// let params = params! {
//     "date_today" => chrono::Utc::today().to_string(),
// };

// #[serde(with = "crate::clock::date_format")]
// pub term_begin: chrono::NaiveDate,
// pub term_end: chrono::NaiveDate,


