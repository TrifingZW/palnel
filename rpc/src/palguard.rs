use leptos::prelude::*;

/// 启动 Palworld 服务器进程。
#[server]
pub async fn palguard_start() -> Result<(), ServerFnError> {
    let state = match use_context::<common::state::AppState>() {
        Some(s) => s,
        None => return Err(ServerFnError::ServerError("AppState 上下文缺失".to_string())),
    };

    match state.palguard.start().await {
        Ok(()) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

/// 停止 Palworld 服务器进程。
#[server]
pub async fn palguard_stop() -> Result<(), ServerFnError> {
    let state = match use_context::<common::state::AppState>() {
        Some(s) => s,
        None => return Err(ServerFnError::ServerError("AppState 上下文缺失".to_string())),
    };

    match state.palguard.stop().await {
        Ok(()) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

/// 重启 Palworld 服务器进程（优雅停止 + 重新启动）。
#[server]
pub async fn palguard_restart() -> Result<(), ServerFnError> {
    let state = match use_context::<common::state::AppState>() {
        Some(s) => s,
        None => return Err(ServerFnError::ServerError("AppState 上下文缺失".to_string())),
    };

    match state.palguard.restart().await {
        Ok(()) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}
