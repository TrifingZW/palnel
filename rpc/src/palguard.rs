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

/// 向 Palworld 服务器发送全服公告。
#[server]
pub async fn palguard_announce(message: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let cfg = match pal_cfg() {
            Ok(c) => c,
            Err(e) => return Err(e),
        };
        match services::pal_rest::announce(&cfg.0, cfg.1, &cfg.2, &cfg.3, &message).await {
            Some(()) => {}
            None => return Err(ServerFnError::ServerError("公告发送失败".to_string())),
        }
    }
    Ok(())
}

/// 手动保存 Palworld 世界存档。
#[server]
pub async fn palguard_save() -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let cfg = match pal_cfg() {
            Ok(c) => c,
            Err(e) => return Err(e),
        };
        match services::pal_rest::save_world(&cfg.0, cfg.1, &cfg.2, &cfg.3).await {
            Some(()) => {}
            None => return Err(ServerFnError::ServerError("存档保存失败".to_string())),
        }
    }
    Ok(())
}

/// 优雅关闭 Palworld 服务器（等待指定秒数并广播公告）。
#[server]
pub async fn palguard_shutdown(waittime: u32, message: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let cfg = match pal_cfg() {
            Ok(c) => c,
            Err(e) => return Err(e),
        };
        match services::pal_rest::shutdown_server(&cfg.0, cfg.1, &cfg.2, &cfg.3, waittime, &message)
            .await
        {
            Some(()) => {}
            None => return Err(ServerFnError::ServerError("关服指令发送失败".to_string())),
        }
    }
    Ok(())
}

/// 强制立即停止 Palworld 服务器。
#[server]
pub async fn palguard_force_stop() -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let cfg = match pal_cfg() {
            Ok(c) => c,
            Err(e) => return Err(e),
        };
        match services::pal_rest::force_stop_server(&cfg.0, cfg.1, &cfg.2, &cfg.3).await {
            Some(()) => {}
            None => return Err(ServerFnError::ServerError("强停指令发送失败".to_string())),
        }
    }
    Ok(())
}

/// 提取 Palworld REST API 配置，返回 (ip, port, user, pass)。
#[cfg(feature = "ssr")]
fn pal_cfg() -> Result<(String, u16, String, String), ServerFnError> {
    let state = match use_context::<common::state::AppState>() {
        Some(s) => s,
        None => return Err(ServerFnError::ServerError("AppState 上下文缺失".to_string())),
    };
    let c = &state.config.palworld;
    Ok((c.ip.clone(), c.rest_port, c.username.clone(), c.password.clone()))
}
