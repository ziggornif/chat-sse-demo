use axum::{
    Router,
    routing::{get, post},
};

use chat_sse_demo::{
    feeder::feed_db,
    handlers::{index::index, login::login, send::send, sse::sse_handler},
    state::{AppState, SseMessage},
    types::Db,
};
use tokio::{net::TcpListener, sync::broadcast};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let db = Db::default();
    feed_db(&db);

    let (sse_sender, _) = broadcast::channel::<SseMessage>(1000);

    let app_state = AppState { db, sse_sender };

    let app = Router::new()
        .route("/", get(index))
        .route("/send", post(send))
        .route("/sse", get(sse_handler))
        .route("/login", post(login))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;
    Ok(())
}
