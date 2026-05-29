use leptos::prelude::*;

// ========= 数据模型（来自 common）========
pub use common::user::User;

// 统一管理 SSR 端的依赖导入，保持跨文件架构一致性
#[cfg(feature = "ssr")]
mod ssr_imports {
    pub use crate::extractors::auth_claims;
}

// ==========================================
// Server Functions (RPC)
// ==========================================

/// 获取当前登录用户。未认证时返回 Ok(None)，避免 500。
#[server(GetUser, "/api")]
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
    #[cfg(feature = "ssr")]
    use ssr_imports::*;

    // 1) 提取 JWT 声明，未认证时优雅返回 None
    let current_claims = match auth_claims().await {
        Ok(claims) => claims,
        Err(_) => return Ok(None),
    };

    // 2) 映射内部 Claims 为前端安全的数据模型
    Ok(Some(User {
        username: current_claims.username,
        role: current_claims.role,
    }))
}
