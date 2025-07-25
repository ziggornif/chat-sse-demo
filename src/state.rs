use tokio::sync::broadcast;

use crate::types::Db;

#[derive(Clone, Debug)]
pub struct SseMessage {
    pub event: String,
    pub data: String,
}

// État partagé de l'application
#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub sse_sender: broadcast::Sender<SseMessage>,
}
