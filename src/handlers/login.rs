use axum::{
    Form,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use axum_extra::extract::{CookieJar, cookie::Cookie};

use crate::types::LoginForm;

#[axum::debug_handler]
pub async fn login(cookies: CookieJar, Form(input): Form<LoginForm>) -> impl IntoResponse {
    let cookie = Cookie::new("session_username", input.username.clone());
    let jar = cookies.add(cookie);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("HX-Redirect", "/")
        .body("".to_string())
        .unwrap();

    (jar, response)
}
