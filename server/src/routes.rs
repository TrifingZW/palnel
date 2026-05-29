use std::{convert::Infallible, time::Duration};

use axum::{
    Router,
    extract::{DefaultBodyLimit, FromRef, State},
    response::{
        Sse,
        sse::{Event, KeepAlive},
    },
};
use common::state::{AppState, SsePayload};
use futures::Stream;
use leptos::prelude::LeptosOptions;
use tokio_stream::{StreamExt, wrappers::BroadcastStream};
use tower_http::limit::RequestBodyLimitLayer;

pub trait WitiumRoutes<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn witium_routes(self) -> Self;
}

impl<S> WitiumRoutes<S> for Router<S>
where
    S: Clone + Send + Sync + 'static,
    AppState: FromRef<S>,
    LeptosOptions: FromRef<S>,
{
    fn witium_routes(self) -> Self {
        self.layer(DefaultBodyLimit::disable())
            .layer(RequestBodyLimitLayer::new(250 * 1024 * 1024))
            .route("/api/sse", axum::routing::get(sse_handler))
    }
}

/// SSE 端点：将后台轮询结果推送到所有已连接的 Web 前端。
async fn sse_handler(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.sse_tx.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(|res| {
        let payload: SsePayload = res.ok()?;
        let (event_name, json) = match &payload {
            SsePayload::PalInfoPayload(v) => ("palinfo", serde_json::to_string(v).ok()?),
            SsePayload::PalMetricsPayload(v) => ("palmetrics", serde_json::to_string(v).ok()?),
            SsePayload::PalPlayerListPayload(v) => {
                ("palplayerlist", serde_json::to_string(v).ok()?)
            }
            SsePayload::SystemMetricsPayload(v) => {
                ("systemmetrics", serde_json::to_string(v).ok()?)
            }
            SsePayload::PalguardProcessPayload(v) => {
                ("palguardprocess", serde_json::to_string(v).ok()?)
            }
        };
        Some(Ok(Event::default().event(event_name).data(json)))
    });
    Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(15)).text("ping"))
}
