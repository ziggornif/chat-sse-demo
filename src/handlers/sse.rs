use axum::{
    extract::State,
    response::{Sse, sse::Event},
};
use std::convert::Infallible;
use std::time::Duration;

use axum_extra::TypedHeader;

use futures_util::Stream;
use tokio_stream::{StreamExt as _, wrappers::BroadcastStream};

use crate::state::AppState;

pub async fn sse_handler(
    State(state): State<AppState>,
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("`{}` connected", user_agent.as_str());

    let receiver = state.sse_sender.subscribe();
    let stream = BroadcastStream::new(receiver).filter_map(|result| match result {
        Ok(sse_message) => Some(Ok(Event::default()
            .event(sse_message.event)
            .data(sse_message.data))),
        Err(_) => None,
    });

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(30))
            .text("keep-alive"),
    )
}
