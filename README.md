TODO
====

- Dedicated settings file that does not require recompile
- Server action log file
- The response should contain what call stack did result in the repsonse? So that you can track what client action cause the response in shared code
    //Err(crate::Error::user_missing(origin.path()))
    use rocket::http::uri::Origin;
    pub fn user_login(origin: &Origin, credit: Json<Credential>)