use actix_web::error::{Error, ErrorUnauthorized};
use jsonwebtoken::{self, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

// Token lifetime and Secret key are hardcoded for clarity
const JWT_EXPIRATION_HOURS: i64 = 24;
const SECRET: &str = "Indisputably-Ursine-Statement-05";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: i64,                         //(expiration time): Time after which the JWT expires
    pub aud: Option<String>,              //(audience): Recipient for which the JWT is intended
    pub iss: Option<String>,              //(issuer): Issuer of the JWT
    pub sub: Option<String>,              //(subject): Subject of the JWT (the user)
    pub nbf: Option<i64>,                 //(not before time): Time before which the JWT must not be accepted for processing
    pub iat: Option<i64>,                 //(issued at time): Time at which the JWT was issued; can be used to determine age of the JWT
    pub username: Option<String>,
    pub permissions: Option<Vec<String>>,
}

impl Claims {
    pub fn new(username: String, permissions: Vec<String>) -> Self {
        Self {
            exp: (Utc::now() + Duration::try_minutes(JWT_EXPIRATION_HOURS).unwrap()).timestamp(),
            aud: None,
            iss: None,
            sub: None,
            nbf: None,
            iat: Some((Utc::now()).timestamp()),
            username: Some(username),
            permissions: Some(permissions),
        }
    }
}

// Create a json web token (JWT)
pub fn create_jwt(claims: Claims) -> Result<String, Error> {
    let encoding_key = EncodingKey::from_secret(SECRET.as_bytes());

    jsonwebtoken::encode(&Header::default(), &claims, &encoding_key)
        .map_err(|err| ErrorUnauthorized(err.to_string()))
}

// Decode a json web token (JWT)
pub fn decode_jwt(token: &str) -> Result<Claims, Error> {
    let decoding_key = DecodingKey::from_secret(SECRET.as_bytes());

    print!("\nToken: {:?}\n", token);

    let temp = jsonwebtoken::decode::<Claims>(token, &decoding_key, &Validation::new(Algorithm::HS256))
        .map(|data| data.claims)
        .map_err(|err| ErrorUnauthorized(err.to_string()));

    print!("\nResult: {:?}\n", temp);

    temp
}
