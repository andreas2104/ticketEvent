use axum::{
    extract::{State},
    http::StatusCode,
    Json,
};
use crate::repositories::user_repository::UserRepository;
use crate::domain::user::CreateUser;
use crate::domain::login::{LoginRequest, AuthResponse};
use std::sync::Arc;
use bcrypt::verify;
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use std::env;
use axum::extract::Query;

#[derive(Debug, Deserialize)]
pub struct AuthCallbackParams {
    pub code: String,
    pub state: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    role: String,
    exp: usize,
}

pub async fn register(
    State(repo): State<Arc<UserRepository>>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<i64>), (StatusCode, String)> {
    match repo.create(payload).await {
        Ok(id) => Ok((StatusCode::CREATED, Json(id))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn login(
    State(repo): State<Arc<UserRepository>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let user = repo.get_by_email(&payload.email).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))?;

    let valid = verify(&payload.password, user.password.as_ref().unwrap_or(&"".to_string()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.email.clone(),
        role: user.role.clone(),
        exp: expiration,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_ref()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AuthResponse {
        token,
        user,
    }))
}

pub async fn google_login(
    State(repo): State<Arc<UserRepository>>,
    Json(payload): Json<crate::domain::login::GoogleLoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    // 1. Verify token with Google
    let client = reqwest::Client::new();
    let response = client.get("https://www.googleapis.com/oauth2/v3/tokeninfo")
        .query(&[("id_token", &payload.token)])
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    if !response.status().is_success() {
        return Err((StatusCode::UNAUTHORIZED, "Invalid Google token".to_string()));
    }

    let google_user: crate::domain::oauth::GoogleUser = response.json().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 2. Find or create user
    let user = repo.find_or_create_google_user(google_user).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 3. Generate token
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.email.clone(),
        role: user.role.clone(),
        exp: expiration,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_ref()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AuthResponse {
        token,
        user,
    }))
}

pub async fn google_callback(
    State(repo): State<Arc<UserRepository>>,
    Query(params): Query<AuthCallbackParams>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let client_id = env::var("GOOGLE_CLIENT_ID").map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "GOOGLE_CLIENT_ID not set".to_string()))?;
    let client_secret = env::var("CLIENT_SECRET").map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "CLIENT_SECRET not set".to_string()))?;
    let redirect_uri = env::var("GOOGLE_URI").unwrap_or_else(|_| "http://127.0.0.1:4000/auth/callback".to_string());

    let client = reqwest::Client::new();
    
    // 1. Exchange code for access_token and id_token
    let token_response = client.post("https://oauth2.googleapis.com/token")
        .form(&[
            ("code", params.code.as_str()),
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("redirect_uri", redirect_uri.as_str()),
            ("grant_type", "authorization_code"),
        ])
        .send()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !token_response.status().is_success() {
        let error_body = token_response.text().await.unwrap_or_default();
        return Err((StatusCode::UNAUTHORIZED, format!("Token exchange failed: {}", error_body)));
    }

    let token_data: serde_json::Value = token_response.json().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let id_token = token_data.get("id_token")
        .and_then(|t| t.as_str())
        .ok_or((StatusCode::UNAUTHORIZED, "No id_token in response".to_string()))?;

    // 2. Verify id_token and get user info
    let verify_response = client.get("https://www.googleapis.com/oauth2/v3/tokeninfo")
        .query(&[("id_token", id_token)])
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    if !verify_response.status().is_success() {
        return Err((StatusCode::UNAUTHORIZED, "Invalid Google token".to_string()));
    }

    let google_user: crate::domain::oauth::GoogleUser = verify_response.json().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 3. Find or create user
    let user = repo.find_or_create_google_user(google_user).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 4. Generate local JWT
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.email.clone(),
        role: user.role.clone(),
        exp: expiration,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_ref()))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AuthResponse {
        token,
        user,
    }))
}
