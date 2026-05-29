use std::time::Duration;

use common::{
    pal::{PalInfo, PalMetrics, PalPlayerList, PalguardProcessStatus},
    state::{AppState, SsePayload},
};
use tokio::time::sleep;

use crate::pal_rest::{fetch_info, fetch_metrics, fetch_players};
use crate::sys_poller::collect_system_metrics;

/// 启动 Palworld 服务器状态监控后台任务
pub fn start_palworld_fetch(app_state: &AppState) {
    let ip = app_state.config.palworld.ip.clone();
    let rest_port = app_state.config.palworld.rest_port;
    let username = app_state.config.palworld.username.clone();
    let password = app_state.config.palworld.password.clone();
    let sse_tx = app_state.sse_tx.clone();

    // 1) Metrics
    let metrics_state = app_state.clone();
    let ip_m = ip.clone();
    let user_m = username.clone();
    let pass_m = password.clone();
    let tx_m = sse_tx.clone();
    tokio::spawn(async move {
        loop {
            if let Some(raw_metrics) = fetch_metrics(&ip_m, rest_port, &user_m, &pass_m).await {
                let clean_metrics: PalMetrics = raw_metrics.into();
                let mut lock = metrics_state.pal_metrics.write().unwrap();
                *lock = clean_metrics.clone();
                let _ = tx_m.send(SsePayload::PalMetricsPayload(clean_metrics));
            } else {
                // 推送空数据清除前端旧值
                let _ = tx_m.send(SsePayload::PalMetricsPayload(PalMetrics::default()));
            }
            sleep(Duration::from_secs(1)).await;
        }
    });

    // 2) Player List
    let players_state = app_state.clone();
    let ip_p = ip.clone();
    let user_p = username.clone();
    let pass_p = password.clone();
    let tx_p = sse_tx.clone();
    tokio::spawn(async move {
        loop {
            if let Some(raw_players) = fetch_players(&ip_p, rest_port, &user_p, &pass_p).await {
                let clean_players: PalPlayerList = raw_players.into();
                let mut lock = players_state.pal_player_list.write().unwrap();
                *lock = clean_players.clone();
                let _ = tx_p.send(SsePayload::PalPlayerListPayload(clean_players));
            } else {
                // 仅当 metrics 认为在线但 players 拉取失败时，推送空数据
                let _ = tx_p.send(SsePayload::PalPlayerListPayload(PalPlayerList::default()));
            }
            sleep(Duration::from_secs(1)).await;
        }
    });

    // 3) Server Info
    let info_state = app_state.clone();
    let tx_i = sse_tx;
    tokio::spawn(async move {
        loop {
            if let Some(raw_info) = fetch_info(&ip, rest_port, &username, &password).await {
                let clean_info: PalInfo = raw_info.into();
                let mut lock = info_state.pal_info.write().unwrap();
                *lock = clean_info.clone();
                let _ = tx_i.send(SsePayload::PalInfoPayload(clean_info));
            }
            sleep(Duration::from_secs(1)).await;
        }
    });
}

pub fn start_system_metrics_fetch(app_state: &AppState) {
    let arc_sys_metrics = app_state.sys_metrics.clone();
    let sse_tx = app_state.sse_tx.clone();

    tokio::spawn(async move {
        loop {
            let sys_metrics = collect_system_metrics().await;
            {
                let mut lock = arc_sys_metrics.write().unwrap();
                *lock = sys_metrics.clone();
            }
            let _ = sse_tx.send(SsePayload::SystemMetricsPayload(sys_metrics));

            sleep(Duration::from_secs(1)).await;
        }
    });
}

/// 启动 Palguard 进程状态监控后台任务。
///
/// 每 1 秒查询一次 Palguard 的进程状态，通过 SSE 推送至前端。
/// 仅在状态发生变化时发送事件，避免无谓的前端重渲染。
pub fn start_palguard_process_fetch(app_state: &AppState) {
    let palguard = app_state.palguard.clone();
    let arc_status = app_state.palguard_process.clone();
    let sse_tx = app_state.sse_tx.clone();

    tokio::spawn(async move {
        let mut last_status = PalguardProcessStatus::default();
        loop {
            let raw = palguard.status().await;
            let current: PalguardProcessStatus = raw.into();

            if current != last_status {
                last_status = current.clone();

                {
                    let mut lock = arc_status.write().unwrap();
                    *lock = current.clone();
                }
                let _ = sse_tx.send(SsePayload::PalguardProcessPayload(current));
            }

            sleep(Duration::from_secs(1)).await;
        }
    });
}
