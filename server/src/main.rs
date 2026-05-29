#![recursion_limit = "1024"]

pub mod db;
pub mod fetch;
pub mod logger;
pub mod pal_rest;
pub mod routes;
pub mod server;
pub mod sys_poller;

use std::sync::{Arc, RwLock};

use app::{App, shell};
use axum::Router;
use common::{
    config::AppConfig,
    pal::{PalInfo, PalMetrics, PalPlayerList, PalguardProcessStatus},
    state::{AppState, SsePayload},
    sysinfo::SystemMetrics,
};
use leptos::{config::get_configuration, prelude::provide_context};
use leptos_axum::{LeptosRoutes, generate_route_list};
use tokio::sync::broadcast;

#[cfg(not(debug_assertions))]
use crate::server::{https, redirect};
use crate::{
    fetch::{start_palguard_process_fetch, start_palworld_fetch, start_system_metrics_fetch},
    routes::WitiumRoutes,
    server::http,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1) 初始化日志系统并持有 WorkerGuard
    let _guard = logger::init();

    // 2) 加载 Leptos 配置
    let conf = get_configuration(None)?;
    let leptos_options = conf.leptos_options;

    // 3) 生成路由列表
    let routes = generate_route_list(App);

    // 4) 加载应用配置
    let config = AppConfig::load()?;

    // 5) 连接数据库
    let pool = db::connect_database().await?;

    // 6) 创建 Palguard 进程守护器实例
    let palguard = palguard::Palguard::new(
        config.palworld.executable.clone(),
        config.palworld.workspace.clone(),
        config.palworld.args.clone(),
    );

    // 7) 构建全局应用状态
    let (sse_tx, _sse_rx) = broadcast::channel::<SsePayload>(64);
    let app_state = AppState {
        leptos_options: leptos_options.clone(),
        routes: routes.clone(),
        config: config.clone(),
        pool,
        palguard,
        sys_metrics: Arc::new(RwLock::new(SystemMetrics::default())),
        pal_info: Arc::new(RwLock::new(PalInfo::default())),
        pal_metrics: Arc::new(RwLock::new(PalMetrics::default())),
        pal_player_list: Arc::new(RwLock::new(PalPlayerList::default())),
        palguard_process: Arc::new(RwLock::new(PalguardProcessStatus::default())),
        sse_tx,
    };

    // 8) 启动后台轮询任务
    start_palworld_fetch(&app_state);
    start_system_metrics_fetch(&app_state);
    start_palguard_process_fetch(&app_state);

    // 9) 构建 Axum 路由并注入上下文
    let app = Router::new()
        .witium_routes()
        .leptos_routes_with_context(
            &app_state,
            routes,
            {
                let app_state = app_state.clone();
                move || provide_context(app_state.clone())
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(app_state);

    // 10) 启动服务器
    cfg_select! {
        debug_assertions => {
            http(app, config.server.http_port).await?;
        }
        _ => {
            match config.server.tls {
                Some(tls) if tls.enabled => {
                    tokio::try_join!(
                        redirect(config.server.http_port),
                        https(app, config.server.https_port, tls)
                    )?;
                }
                _ => {
                    http(app, config.server.http_port).await?;
                }
            }
        }
    }

    Ok(())
}
