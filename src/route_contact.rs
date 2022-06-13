// #[post("/user_email", format = "text/plain", data = "<email>")]
// pub fn user_email(session: UserSession, email: String) {
//     if !verify_email(&email) {return};

//     let mut conn : PooledConn = get_pool_conn();
//     let stmt = conn.prep("UPDATE user_contact SET email = :email WHERE user_id = :user_id").unwrap();

//     conn.exec::<String,_,_>(
//         &stmt,
//         mysql::params! { "user_id" => &session.user.id, "email" => &email},
//     ).unwrap();
// }

// pub email: Option<String>,