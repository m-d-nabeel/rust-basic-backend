use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use axum::extract::State;
use axum::http::{HeaderName, StatusCode};
use axum::Json;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;

use crate::model::model::{
    FilteredUser, LoginUserSchema, ModelController, RealUser, RegisterUserSchema, TokenClaims,
};

pub type CustomResult<T> = Result<T, (StatusCode, Json<serde_json::Value>)>;

pub async fn register_user_handler(
    State(mc): State<ModelController>,
    Json(register_us): Json<RegisterUserSchema>,
) -> CustomResult<impl IntoResponse> {
    let existing_user = sqlx::query_scalar(
        r#"SELECT EXISTS(
                SELECT 1 FROM real_user WHERE email = $1
            );"#,
    )
    .bind(register_us.email.to_owned().to_ascii_lowercase())
    .fetch_one(&mc.db_pool)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status":"fail",
            "message": format!("Database Error : {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    if let Some(exists) = existing_user {
        if exists {
            let error_response = serde_json::json!({
                "status":"fail",
                "message":"User with that email already exists"
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(register_us.password.as_bytes(), &salt)
        .map_err(|e| {
            let error_response = json!({
                "status":"fail",
                "message": format!("Error while hashing password, {}",e)
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })
        .map(|hash| hash.to_string())?;

    let user = sqlx::query_as!(
        RealUser,
        r#"INSERT INTO real_user (name, email, password)
                VALUES ($1, $2, $3) RETURNING *;"#,
        register_us.name.to_string(),
        register_us.email.to_string().to_ascii_lowercase(),
        hashed_password
    )
    .fetch_one(&mc.db_pool)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;
    let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
        "user": filter_user_record(&user)
    })});

    Ok(Json(user_response))
}

/******************************END BLOCK****************************/

pub async fn login_user_handler(
    State(mc): State<ModelController>,
    Json(login_us): Json<LoginUserSchema>,
) -> CustomResult<impl IntoResponse> {
    let user_email = login_us.email.to_ascii_lowercase();
    let user_password = login_us.password;
    let user = sqlx::query_as!(
        RealUser,
        "SELECT * FROM real_user WHERE email = $1",
        user_email
    )
    .fetch_optional(&mc.db_pool)
    .await
    .map_err(|e| {
        let error_response = json! ({
            "status":"error",
            "message":format!("Database error, {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?
    .ok_or_else(|| {
        let error_response = json! ({
            "status":"fail",
            "message":"Invalid email"
        });
        (StatusCode::BAD_REQUEST, Json(error_response))
    })?;

    let is_valid = match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(user_password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        let error_response = json!({
            "status" : "fail",
            "message":"Invalid password"
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(15)).timestamp() as usize;

    let claims = TokenClaims {
        sub: user.id.to_string(),
        iat,
        exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(b"my_ultra_secure_secret"),
    )
    .unwrap();

    let cookie = Cookie::build(("token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .build();

    let mut response = Response::new(
        json!({
            "status":"success",
            "token" : token
        })
        .to_string(),
    );

    response.headers_mut().insert(
        "set_cookie".parse::<HeaderName>().unwrap(),
        cookie.to_string().parse().unwrap(),
    );
    Ok(response)
}

/*******************************************************/
fn filter_user_record(user: &RealUser) -> FilteredUser {
    FilteredUser {
        id: user.id.to_string(),
        email: user.email.to_owned(),
        name: user.name.to_owned(),
        created_at: Option::from(user.created_at.unwrap()),
    }
}
