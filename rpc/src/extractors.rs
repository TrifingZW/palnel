use axum_extra::extract::CookieJar;
use common::{claims::Claims, state::AppState};
use leptos::{config::LeptosOptions, context::use_context, prelude::ServerFnError};
use leptos_axum::extract;
use sqlx::SqlitePool;

pub fn leptos_options() -> Result<LeptosOptions, ServerFnError> {
    let state = match use_context::<AppState>() {
        Some(s) => s,
        None => {
            return Err(ServerFnError::ServerError("AppState not found in context".to_string()));
        }
    };

    Ok(state.leptos_options)
}

pub async fn auth_claims() -> Result<Claims, ServerFnError> {
    let claims = extract::<Claims>().await?;

    Ok(claims)
}

pub async fn cookie_jar() -> Result<CookieJar, ServerFnError> {
    let jar = extract::<CookieJar>().await?;

    Ok(jar)
}

pub fn pool() -> Result<SqlitePool, ServerFnError> {
    // 1) 获取应用状态上下文，若失败抛出明确的 MissingContext 错误
    let state = match use_context::<AppState>() {
        Some(s) => s,
        None => {
            return Err(ServerFnError::ServerError("AppState not found in context".to_string()));
        }
    };

    // 2) 提取并返回数据库连接池实例
    Ok(state.pool)
}
