use axum::{
    Json,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use axum_extra::{
    TypedHeader,
    extract::CookieJar,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::LazyLock;

/// 密钥全局单例
pub static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    // 1) 获取密钥配置 (注：生产环境推荐从环境变量读取，此处为演示固定值)
    let secret = "OsKu4G6J+dR8iE+1SB06sqsiyg1X84TJhfGmZSZNe8I=";

    // 2) 初始化并返回全局唯一的密钥实例
    Keys::new(secret.as_bytes())
});

/// 编码与解码密钥集
pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

/// 统一授权错误枚举
#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    MissingToken,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        // 1) 映射错误枚举到 HTTP 状态码和直观的提示信息
        let (status, error_message) = match self {
            Self::WrongCredentials => (StatusCode::UNAUTHORIZED, "用户名或密码错误"),
            Self::MissingCredentials => (StatusCode::BAD_REQUEST, "用户名和密码不能为空"),
            Self::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "令牌创建失败"),
            Self::MissingToken => (StatusCode::UNAUTHORIZED, "未提供授权令牌"),
            Self::InvalidToken => (StatusCode::UNAUTHORIZED, "无效的授权令牌"),
        };

        // 2) 构建标准化的 JSON 错误响应体
        let body = Json(json!({
            "error": error_message,
        }));

        // 3) 组合状态码与响应体并转化为标准化 Response
        (status, body).into_response()
    }
}

/// JWT 声明数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub username: String,
    pub role: String,
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 1) 提取请求中的授权令牌，若缺失则快速失败
        let token = extract_token_from_request(parts).await.ok_or(AuthError::MissingToken)?;

        // 2) 配置 JWT 验证规则并执行解码验证
        let validation = Validation::default();
        let token_data = decode::<Claims>(&token, &KEYS.decoding, &validation)
            .map_err(|_| AuthError::InvalidToken)?;

        // 3) 提取并返回合法的声明数据
        Ok(token_data.claims)
    }
}

/// 辅助函数：从请求中安全提取授权令牌
async fn extract_token_from_request(parts: &mut Parts) -> Option<String> {
    // 1) 尝试从 CookieJar 提取 (优先用于 Web 前端调用)
    match CookieJar::from_request_parts(parts, &()).await {
        Ok(cookie_jar) => {
            if let Some(cookie) = cookie_jar.get("auth_token") {
                return Some(cookie.value().to_string());
            }
        }
        _ => (),
    }

    // 2) 尝试从 Authorization Bearer Header 提取 (优先用于 API 调用)
    if let Ok(bearer) = TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, &()).await {
        return Some(bearer.token().to_string());
    }

    // 3) 所有途径均未匹配时，返回 None
    None
}
