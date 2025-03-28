use super::super::super::back_end::module_auth::Module;
use serde::{Deserialize, Serialize};
//use chrono::{Duration, TimeDelta, Utc};

// token lifetime are hardcoded for now
//const JWT_EXPIRATION_MINS: i64 = 24;

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    NoAccess,
    ReadOnly(Module),
    ReadAndWrite(Module)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPayload {
    pub exp: i64,             //(expiration time): Time after which the JWT expires
    pub iat: Option<i64>,     //(issued at time): Time at which the JWT was issued; can be used to determine age of the JWT
    pub nbf: Option<i64>,     //(not before time): Time before which the JWT must not be accepted for processing
    pub aud: Option<String>,  //(audience): Recipient for which the JWT is intended
    pub iss: Option<String>,  //(issuer): Issuer of the JWT
    pub sub: Option<String>,  //(subject): Subject of the JWT (the user)
    pub id: Option<String>,
    pub permissions: Option<Vec<Permission>>,
}

//impl TokenPayload {
//    pub fn new(id: String, permissions: Vec<Permission>) -> Self {
//        let time_delta = match Duration::try_minutes(JWT_EXPIRATION_MINS) {
//            Some(time_delta) =>  time_delta,
//            None => TimeDelta::MIN
//        };
//
//        Self {
//            exp: (Utc::now() + time_delta).timestamp(),
//            iat: Some((Utc::now()).timestamp()),
//            nbf: None,
//            aud: None,
//            iss: None,
//            sub: None,
//            id: Some(id),
//            permissions: Some(permissions),
//        }
//    }
//}
