/// 客户端与服务端共享的当前用户状态。
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub username: String,
    pub role: String,
}
