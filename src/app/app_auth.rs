use super::shared::models::auth_models::{TokenPayload, Permission};
use std::collections::HashSet;
use actix_web::{
    dev::ServiceRequest,
    http::header::Header,
    error::{Error, ErrorUnauthorized},
};
use actix_web_httpauth::headers::authorization::{Bearer, Authorization};
use jsonwebtoken::{DecodingKey, EncodingKey, Validation, Algorithm};

const SECRET: &str = "Indisputably-Ursine-Statement-05";

// create a json web token
pub fn jwt_create(claims: TokenPayload) -> Result<String, Error> {
    let encoding_key = EncodingKey::from_secret(SECRET.as_bytes());

    let result = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &encoding_key)
        .map_err(|err| ErrorUnauthorized(err.to_string()));

    result
}

// decode a json web token
pub fn jwt_decode(token: &str) -> Result<TokenPayload, Error> {
    let decoding_key = DecodingKey::from_secret(SECRET.as_bytes());

    let result = jsonwebtoken::decode::<TokenPayload>(
            token, &decoding_key, &Validation::new(Algorithm::HS256)
        )
        .map(|data| data.claims)
        .map_err(|err| ErrorUnauthorized(err.to_string()));

    result
}

pub async fn jwt_extract(
    req: &ServiceRequest
) -> Result<HashSet<Permission>, actix_web::Error> {

    let permissions = match Authorization::<Bearer>::parse(req).ok() {
        Some(auth) => {
            let claims = jwt_decode(auth.as_ref().token())?;
            match claims.permissions {
                Some(perm) => perm.into_iter().collect(),
                None => {HashSet::new()}
            }
        },
        None => {HashSet::new()}
    };

    Ok(permissions)
}
