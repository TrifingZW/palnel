use std::sync::{Arc, RwLock};

use axum::extract::FromRef;
use leptos::prelude::*;
use leptos_axum::AxumRouteListing;
use palguard::Palguard;
use serde::Serialize;
use sqlx::SqlitePool;
use tokio::sync::broadcast;

use crate::{
    config::AppConfig,
    pal::{PalInfo, PalMetrics, PalPlayerList, PalguardProcessStatus},
    sysinfo::SystemMetrics,
};

#[derive(Debug, Clone, Serialize)]
pub enum SsePayload {
    PalInfoPayload(PalInfo),
    PalMetricsPayload(PalMetrics),
    PalPlayerListPayload(PalPlayerList),
    SystemMetricsPayload(SystemMetrics),
    PalguardProcessPayload(PalguardProcessStatus),
}

#[derive(FromRef, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub routes: Vec<AxumRouteListing>,
    pub config: AppConfig,
    pub pool: SqlitePool,
    pub palguard: Palguard,

    pub sys_metrics: Arc<RwLock<SystemMetrics>>,

    pub pal_info: Arc<RwLock<PalInfo>>,
    pub pal_metrics: Arc<RwLock<PalMetrics>>,
    pub pal_player_list: Arc<RwLock<PalPlayerList>>,
    pub palguard_process: Arc<RwLock<PalguardProcessStatus>>,

    pub sse_tx: broadcast::Sender<SsePayload>,
}
