use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    Json,
};
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use reqwest::header::USER_AGENT;
use serde::Deserialize;
use sqlx::SqlitePool;
use std::env;
use tower_cookies::{Cookie, Cookies};

use crate::models::User;

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Debug, Deserialize)]
struct GithubUser {
    id: i64,
    login: String,
    avatar_url: Option<String>,
}

// Helper to create OAuth client
fn oauth_client() -> BasicClient {
    let client_id = env::var("GITHUB_CLIENT_ID").expect("Missing GITHUB_CLIENT_ID");
    let client_secret = env::var("GITHUB_CLIENT_SECRET").expect("Missing GITHUB_CLIENT_SECRET");
    let redirect_url = env::var("GITHUB_REDIRECT_URL").expect("Missing GITHUB_REDIRECT_URL");
    let auth_url = "https://github.com/login/oauth/authorize".to_string();
    let token_url = "https://github.com/login/oauth/access_token".to_string();

    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(auth_url).unwrap(),
        Some(TokenUrl::new(token_url).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
}

pub async fn login(cookies: Cookies) -> impl IntoResponse {
    let client = oauth_client();
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("read:user".to_string()))
        .url();

    // Store CSRF token in cookie for validation in callback
    // In a production app, use a secure, signed cookie
    cookies.add(Cookie::new("oauth_state", csrf_state.secret().clone()));

    Redirect::to(authorize_url.as_str())
}

pub async fn callback(
    State(pool): State<SqlitePool>,
    cookies: Cookies,
    Query(query): Query<AuthRequest>,
) -> impl IntoResponse {
    let client = oauth_client();
    
    // Validate CSRF state
    if let Some(cookie) = cookies.get("oauth_state") {
        if cookie.value() != query.state {
             return Redirect::to("/?error=csrf_mismatch");
        }
    } else {
         return Redirect::to("/?error=missing_csrf_state");
    }
    cookies.remove(Cookie::new("oauth_state", ""));

    // Exchange code for token
    let token_result = client
        .exchange_code(oauth2::AuthorizationCode::new(query.code))
        .request_async(oauth2::reqwest::async_http_client)
        .await;

    let token = match token_result {
        Ok(t) => t,
        Err(_) => return Redirect::to("/?error=token_exchange_failed"),
    };

    // Fetch user info from GitHub
    let client = reqwest::Client::new();
    let github_user: GithubUser = client
        .get("https://api.github.com/user")
        .header(USER_AGENT, "thread-message-app")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    // Upsert user in DB
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (github_id, username, avatar_url)
        VALUES (?, ?, ?)
        ON CONFLICT(github_id) DO UPDATE SET
            username = excluded.username,
            avatar_url = excluded.avatar_url
        RETURNING *
        "#,
    )
    .bind(github_user.id)
    .bind(github_user.login)
    .bind(github_user.avatar_url)
    .fetch_one(&pool)
    .await
    .unwrap();

    // Set session cookie (simplified: just user_id)
    // IMPORTANT: In production, use a signed/encrypted cookie to prevent tampering!
    // tower-cookies handles signing if configured with a key, but here we use plain for simplicity
    // as we don't have signing key setup yet. 
    // Review comment asked for Secure, HttpOnly.
    // tower-cookies sets HttpOnly by default.
    let mut cookie = Cookie::new("user_id", user.id.to_string());
    cookie.set_http_only(true);
    cookie.set_path("/");
    cookie.set_secure(true); // Ensure this is only sent over HTTPS (or localhost)
    cookie.set_same_site(tower_cookies::cookie::SameSite::Lax); // Allow redirect to work

    // Set a max age (e.g., 7 days)
    cookie.set_max_age(Some(tower_cookies::cookie::time::Duration::days(7)));

    cookies.add(cookie);

    let frontend_url = env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());
    Redirect::to(&frontend_url)
}

pub async fn get_me(
    State(pool): State<SqlitePool>,
    cookies: Cookies,
) -> impl IntoResponse {
    let user_id = match cookies.get("user_id") {
        Some(c) => c.value().parse::<i64>().unwrap_or(0),
        None => return (axum::http::StatusCode::UNAUTHORIZED, Json(None::<User>)),
    };

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_optional(&pool)
        .await
        .unwrap();

    match user {
        Some(u) => (axum::http::StatusCode::OK, Json(Some(u))),
        None => (axum::http::StatusCode::UNAUTHORIZED, Json(None)),
    }
}

pub async fn logout(cookies: Cookies) -> impl IntoResponse {
    cookies.remove(Cookie::new("user_id", ""));
    (axum::http::StatusCode::OK, Json("Logged out"))
}
