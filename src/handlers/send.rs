use askama::Template;
use axum::{Form, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;
use ulid::Ulid;

use crate::{
    state::{AppState, SseMessage},
    templates::HtmlTemplate,
    types::{Message, NewMessage},
};

#[derive(Template)]
#[template(path = "message.html")]
struct MessageTemplate {
    message: Message,
}

#[axum::debug_handler]
pub async fn send(
    cookies: CookieJar,
    state: State<AppState>,
    Form(input): Form<NewMessage>,
) -> impl IntoResponse {
    let username = match cookies.get("session_username") {
        Some(cookie) => cookie.value().to_string(),
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    let message = Message {
        id: Ulid::new(),
        content: input.content,
        author: username.clone(),
        avatar: format!("https://api.dicebear.com/9.x/thumbs/svg?seed={}", username),
    };

    match state.db.write() {
        Ok(mut db) => {
            db.insert(message.id, message.clone());
            let message_html = MessageTemplate {
                message: message.clone(),
            };
            let html_string = message_html.render().unwrap_or_default();
            let _ = state.sse_sender.send(SseMessage {
                event: "message".to_string(),
                data: html_string,
            });
            Ok(HtmlTemplate(MessageTemplate { message }))
        }
        Err(e) => {
            println!("Lock error {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
