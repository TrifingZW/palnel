// ========= 当前用户状态模型（客户端安全投影，不含密码）========

/// 客户端与服务端共享的当前用户状态。
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub username: String,
    pub role: String,
}
