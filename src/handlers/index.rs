use askama::Template;
use axum::{extract::State, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{state::AppState, templates::HtmlTemplate, types::Message};

#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate {
    messages: Vec<Message>,
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {}

#[axum::debug_handler]
pub async fn index(cookies: CookieJar, State(state): State<AppState>) -> impl IntoResponse {
    let session_cookie = cookies
        .get("session_username")
        .map(|c| c.value().to_string());

    match session_cookie {
        Some(user) if !user.is_empty() => {
            // Utilisateur connecté
            let db = state.db.read().unwrap();
            let mut messages = db.values().cloned().collect::<Vec<_>>();
            messages.sort_by(|a, b| a.id.cmp(&b.id));

            HtmlTemplate(HomeTemplate { messages }).into_response()
        }
        _ => {
            // Pas connecté
            HtmlTemplate(LoginTemplate {}).into_response()
        }
    }
}
