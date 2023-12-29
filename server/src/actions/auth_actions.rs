use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use axum::extract::State;
use axum::http::{HeaderName, StatusCode};
use axum::Json;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::model::model::{
    FilteredMember, LoginMemberSchema, Member, ModelController, RegisterMemberSchema, TokenClaims,
};

pub type CustomResult<T> = Result<T, (StatusCode, Json<serde_json::Value>)>;

pub async fn register_member_handler(
    State(mc): State<ModelController>,
    Json(register_ms): Json<RegisterMemberSchema>,
) -> CustomResult<impl IntoResponse> {
    let existing_member = sqlx::query_scalar(
        r#"SELECT EXISTS(
                SELECT 1 FROM member WHERE email = $1
            );"#,
    )
    .bind(register_ms.email.to_owned().to_ascii_lowercase())
    .fetch_one(&mc.db_pool)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status":"fail",
            "message": format!("Database Error : {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    if let Some(exists) = existing_member {
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
        .hash_password(register_ms.password.as_bytes(), &salt)
        .map_err(|e| {
            let error_response = serde_json::json!({
                "status":"fail",
                "message": format!("Error while hashing password, {}",e)
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })
        .map(|hash| hash.to_string())?;

    let member = sqlx::query_as!(
        Member,
        r#"INSERT INTO member (name, email, password)
           VALUES ($1, $2, $3) RETURNING *;"#,
        register_ms.name.to_string(),
        register_ms.email.to_string().to_ascii_lowercase(),
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
    let member_response = serde_json::json!({"status": "success","data": serde_json::json!({
        "member": filter_member_record(&member)
    })});

    Ok(Json(member_response))
}

/******************************END BLOCK****************************/

pub async fn login_member_handler(
    State(mc): State<ModelController>,
    Json(login_ms): Json<LoginMemberSchema>,
) -> CustomResult<impl IntoResponse> {
    let member_email = login_ms.email.to_ascii_lowercase();
    let member_password = login_ms.password;
    let member = sqlx::query_as!(
        Member,
        "SELECT * FROM member WHERE email = $1",
        member_email
    )
    .fetch_optional(&mc.db_pool)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status":"error",
            "message":format!("Database error, {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?
    .ok_or_else(|| {
        let error_response = serde_json::json!({
            "status":"fail",
            "message":"Invalid email"
        });
        (StatusCode::BAD_REQUEST, Json(error_response))
    })?;

    let is_valid = match PasswordHash::new(&member.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(member_password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        let error_response = serde_json::json!({
            "status" : "fail",
            "message":"Invalid password"
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(15)).timestamp() as usize;

    let claims = TokenClaims {
        sub: member.id.to_string(),
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
        serde_json::json!({
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
fn filter_member_record(member: &Member) -> FilteredMember {
    FilteredMember {
        id: member.id.to_string(),
        email: member.email.to_owned(),
        name: member.name.to_owned(),
        created_at: Option::from(member.created_at.map(|date| date.clone())),
        profile_pic: Option::from(member.profile_pic.as_ref().map(|pfp| pfp.clone())),
        status: Option::from(member.status.as_ref().map(|status| status.clone())),
    }
}