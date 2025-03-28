use actix_web::{HttpRequest, HttpResponse, Responder};
use serde_json::json;
use tracing::instrument;

#[instrument]
pub async fn login(
    req: HttpRequest,
    //body: web::Json<LoginUserSchema>,
    //data_database_pool: Data<Pool<Postgres>>
) -> impl Responder {

    //// get all products with specific type
    //let search_result = actions::search_products_with_type(
    //    pool, &product_type_name, &query_map
    //    );

    HttpResponse::Ok()
        .json(json!({"status": "success", "token": "login"}))
}

//async fn login_user_handler(
//    body: web::Json<LoginUserSchema>,
//    data: web::Data<AppState>,
//) -> impl Responder {
//
//    let now = Utc::now();
//    let iat = now.timestamp() as usize;
//    let exp = (now + Duration::minutes(60)).timestamp() as usize;
//    let claims: TokenClaims = TokenClaims {
//        sub: user.id.to_string(),
//        exp,
//        iat,
//    };
//
//    let token = encode(
//        &Header::default(),
//        &claims,
//        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
//    )
//    .unwrap();
//
//    let cookie = Cookie::build("token", token.to_owned())
//        .path("/")
//        .max_age(ActixWebDuration::new(60 * 60, 0))
//        .http_only(true)
//        .finish();
//
//    HttpResponse::Ok()
//        .cookie(cookie)
//        .json(json!({"status": "success", "token": token}))
//}

