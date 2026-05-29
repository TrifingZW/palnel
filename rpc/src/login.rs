use leptos::prelude::*;

// 统一管理 SSR 端的依赖导入，保持代码整洁
#[cfg(feature = "ssr")]
mod ssr_imports {
    pub use crate::extractors::pool;
    pub use axum::http::header::{self, HeaderValue};
    pub use axum_extra::extract::cookie::{Cookie, SameSite};
    pub use common::{
        claims::{Claims, KEYS},
        tables::User,
    };
    pub use jsonwebtoken::{Header, encode};
    pub use std::time::{SystemTime, UNIX_EPOCH};
    pub use time::Duration;
}

#[server]
pub async fn login(username: String, password: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    use ssr_imports::*;

    // 1) 校验输入合法性
    if username.is_empty() || password.is_empty() {
        return Err(ServerFnError::ServerError("Username or password cannot be empty".into()));
    }

    // 2) 获取 Leptos 响应上下文与数据库连接池
    let response = expect_context::<leptos_axum::ResponseOptions>();
    let pool = pool()?;

    // 3) 从数据库查询目标用户
    let user_result = sqlx::query_as::<_, User>(
        "SELECT id, username, password, role FROM user WHERE username = ?",
    )
    .bind(&username)
    .fetch_optional(&pool)
    .await?;

    // 4) 检查用户是否存在
    let user = match user_result {
        Some(user) => user,
        None => return Err(ServerFnError::ServerError("User err".to_string())),
    };

    // 5) 校验用户密码 (⚠️ 强烈建议在生产环境使用 argon2 或 bcrypt 校验哈希)
    if user.password != password {
        return Err(ServerFnError::ServerError("Invalid credentials".into()));
    }

    // 6) 获取当前统一基准时间戳
    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs()
        as usize;

    // 7) 创建 JWT 声明 (Claims)
    let claims = Claims {
        sub: user.id.to_string(),
        username: user.username,
        role: user.role,
        iat: now,
        exp: now + 24 * 60 * 60, // 当前时间 + 24小时
        iss: "TrifingZW".into(),
    };

    // 8) 签发 JWT 令牌并注入响应 Cookie
    let token = encode(&Header::default(), &claims, &KEYS.encoding)?;
    set_cookie_header(&response, &token, Duration::hours(24))?;

    Ok(())
}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    use ssr_imports::*;

    // 1) 获取 Leptos 响应上下文
    let response = expect_context::<leptos_axum::ResponseOptions>();

    // 2) 注入失效的 Cookie 清除客户端凭证
    set_cookie_header(&response, "", Duration::ZERO)?;

    Ok(())
}

// ==========================================
// SSR 辅助函数区
// ==========================================

/// 统一的 Cookie 注入函数，确保前后端认证状态管理的结构一致性
#[cfg(feature = "ssr")]
fn set_cookie_header(
    response: &leptos_axum::ResponseOptions,
    token: &str,
    max_age: ssr_imports::Duration,
) -> Result<(), ServerFnError> {
    use ssr_imports::*;

    // 1) 构建安全的 HttpOnly Cookie（debug 构建适配 HTTP localhost）
    #[cfg(debug_assertions)]
    let secure = false;
    #[cfg(not(debug_assertions))]
    let secure = true;

    let auth_cookie = Cookie::build(("auth_token", token.to_string()))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(secure)
        .max_age(max_age)
        .build();

    // 2) 转换为 HeaderValue 并追加到 HTTP 响应中
    let header_value = HeaderValue::from_str(&auth_cookie.to_string())?;
    response.append_header(header::SET_COOKIE, header_value);

    Ok(())
}
