#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub role: String,
}
