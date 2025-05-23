use super::{
    super::configuration::Settings,
    shared::models::auth_models::{TokenPayload, ModulePermission}
};
use std::collections::HashSet;
use actix_web::{
    web::Data,
    dev::ServiceRequest,
    http::header::Header,
    error::ErrorUnauthorized,
};
use actix_web_httpauth::headers::authorization::{Bearer, Authorization};
use jsonwebtoken::{DecodingKey, Validation, Algorithm};
use secrecy::ExposeSecret;
//use jsonwebtoken::EncodingKey;

//// create a json web token
//pub fn jwt_create(claims: TokenPayload) -> Result<String, actix_web::Error> {
//    let encoding_key = EncodingKey::from_secret(SECRET.as_bytes());
//
//    let result = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &encoding_key)
//        .map_err(|err| ErrorUnauthorized(err.to_string()));
//
//    result
//}

pub async fn extract_jwt_permissions(
    req: &ServiceRequest
) -> Result<HashSet<ModulePermission>, actix_web::Error> {

    // parse request for bearer authorization
    let bearer_auth = Authorization::<Bearer>::parse(req)?;

    // extract secret
    let config = req.app_data::<Data<Settings>>().expect("Settings not found.");
    let secret = config.authentication.password.expose_secret();

    // decode bearer authorization
    let token = jsonwebtoken::decode::<TokenPayload>(
            bearer_auth.as_ref().token(),
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::new(Algorithm::HS256)
        )
        .map(|data| data.claims)
        .map_err(|err| ErrorUnauthorized(err.to_string()))?;

    //NOTE: for debug purposes
    //dbg!(&token);

    // convert hashmap to hashset
    let permissions = match token.permissions {
        Some(permissions) => {
            permissions
                .into_iter()
                .map(|(key, value)|
                    ModulePermission {
                        module: key,
                        access_level: value
                    }
                )
                .collect::<HashSet<ModulePermission>>()
        },
        None => HashSet::new()
    };

    Ok(permissions)
}
