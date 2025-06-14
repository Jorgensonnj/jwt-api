use super::super::{
    models::auth_models::LoginUser,
    super::super::super::shared::models::auth_models::TokenPayload
};
use chrono::{Duration, Utc};

use jsonwebtoken::{encode, Header, EncodingKey};


//// create a json web token
//pub fn jwt_create(claims: TokenPayload) -> Result<String, actix_web::Error> {
//    let encoding_key = EncodingKey::from_secret(SECRET.as_bytes());
//
//    let result = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &encoding_key)
//        .map_err(|err| ErrorUnauthorized(err.to_string()));
//
//    result
//}

pub async fn login(login_user: LoginUser, secret: something) -> Result<> {

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;
    let claims = TokenPayload::new(login_user.username, permissions);

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
    .unwrap();

    token
}

