use reqwest::Client;
use serde::{Deserialize, Serialize};

fn build_client() -> reqwest::Result<Client> {
    Client::builder().timeout(std::time::Duration::from_secs(5)).build()
}

/// 构造带 Basic Auth 的 GET 请求并反序列化 JSON 响应体，网络错误时返回 `None`。
async fn get<T: for<'de> Deserialize<'de>>(
    base: &str,
    path: &str,
    username: &str,
    password: &str,
) -> Option<T> {
    let client = build_client().ok()?;
    client
        .get(format!("{base}{path}"))
        .basic_auth(username, Some(password))
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()
}

/// 构造带 Basic Auth 的 POST 请求（JSON body），不关心响应体，网络错误时返回 `None`。
async fn post(
    base: &str,
    path: &str,
    username: &str,
    password: &str,
    body: &impl Serialize,
) -> Option<()> {
    let client = build_client().ok()?;
    let resp = client
        .post(format!("{base}{path}"))
        .basic_auth(username, Some(password))
        .json(body)
        .send()
        .await
        .ok()?;
    resp.error_for_status().ok()?;
    Some(())
}

async fn build_base(rest_ip: &str, rest_port: u16) -> String {
    format!("http://{rest_ip}:{rest_port}")
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PalInfoResponse {
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub servername: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub worldguid: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PalMetricsResponse {
    #[serde(default)]
    pub serverfps: u32,
    #[serde(default)]
    pub currentplayernum: u32,
    #[serde(default)]
    pub serverframetime: f64,
    #[serde(default)]
    pub maxplayernum: u32,
    #[serde(default)]
    pub uptime: u64,
    #[serde(default)]
    pub basecampnum: u32,
    #[serde(default)]
    pub days: u32,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PalPlayerListResponse {
    #[serde(default)]
    pub players: Vec<PalPlayer>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PalPlayer {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub account_name: String,
    #[serde(default)]
    pub player_id: String,
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub ip: String,
    #[serde(default)]
    pub ping: f64,
    #[serde(default)]
    pub location_x: f64,
    #[serde(default)]
    pub location_y: f64,
    #[serde(default)]
    pub level: u32,
    #[serde(default)]
    pub building_count: u32,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PalSettingsResponse {
    #[serde(default)]
    pub difficulty: String,
    #[serde(default)]
    pub server_name: String,
    #[serde(default)]
    pub server_description: String,
    #[serde(default)]
    pub death_penalty: String,
    #[serde(default)]
    pub public_ip: String,
    #[serde(default)]
    pub region: String,
    #[serde(default)]
    pub ban_list_url: String,
    #[serde(default)]
    pub allow_connect_platform: String,
    #[serde(default)]
    pub log_format_type: String,
    #[serde(default)]
    pub day_time_speed_rate: f64,
    #[serde(default)]
    pub night_time_speed_rate: f64,
    #[serde(default)]
    pub exp_rate: f64,
    #[serde(default)]
    pub pal_capture_rate: f64,
    #[serde(default)]
    pub pal_spawn_num_rate: f64,
    #[serde(default)]
    pub pal_damage_rate_attack: f64,
    #[serde(default)]
    pub pal_damage_rate_defense: f64,
    #[serde(default)]
    pub player_damage_rate_attack: f64,
    #[serde(default)]
    pub player_damage_rate_defense: f64,
    #[serde(default)]
    pub player_stomach_decreace_rate: f64,
    #[serde(default)]
    pub player_stamina_decreace_rate: f64,
    #[serde(default)]
    #[serde(rename = "PlayerAutoHPRegeneRate")]
    pub player_auto_hp_regene_rate: f64,
    #[serde(default)]
    pub player_auto_hp_regene_rate_in_sleep: f64,
    #[serde(default)]
    pub pal_stomach_decreace_rate: f64,
    #[serde(default)]
    pub pal_stamina_decreace_rate: f64,
    #[serde(default)]
    #[serde(rename = "PalAutoHPRegeneRate")]
    pub pal_auto_hp_regene_rate: f64,
    #[serde(default)]
    pub pal_auto_hp_regene_rate_in_sleep: f64,
    #[serde(default)]
    pub build_object_damage_rate: f64,
    #[serde(default)]
    pub build_object_deterioration_damage_rate: f64,
    #[serde(default)]
    pub collection_drop_rate: f64,
    #[serde(default)]
    pub collection_object_hp_rate: f64,
    #[serde(default)]
    pub collection_object_respawn_speed_rate: f64,
    #[serde(default)]
    pub enemy_drop_item_rate: f64,
    #[serde(default)]
    pub work_speed_rate: f64,
    #[serde(default)]
    pub drop_item_max_num: u32,
    #[serde(default)]
    #[serde(rename = "DropItemMaxNum_UNKO")]
    pub drop_item_max_num_unko: u32,
    #[serde(default)]
    pub base_camp_max_num: u32,
    #[serde(default)]
    pub base_camp_worker_max_num: u32,
    #[serde(default)]
    pub drop_item_alive_max_hours: u32,
    #[serde(default)]
    pub auto_reset_guild_time_no_online_players: u32,
    #[serde(default)]
    pub guild_player_max_num: u32,
    #[serde(default)]
    pub pal_egg_default_hatching_time: u32,
    #[serde(default)]
    pub coop_player_max_num: u32,
    #[serde(default)]
    pub server_player_max_num: u32,
    #[serde(default)]
    pub public_port: u32,
    #[serde(default)]
    pub rcon_port: u32,
    #[serde(default)]
    pub rest_api_port: u32,
    #[serde(default)]
    pub b_enable_player_to_player_damage: bool,
    #[serde(default)]
    pub b_enable_friendly_fire: bool,
    #[serde(default)]
    pub b_enable_invader_enemy: bool,
    #[serde(default)]
    #[serde(rename = "bActiveUNKO")]
    pub b_active_unko: bool,
    #[serde(default)]
    pub b_enable_aim_assist_pad: bool,
    #[serde(default)]
    pub b_enable_aim_assist_keyboard: bool,
    #[serde(default)]
    pub b_auto_reset_guild_no_online_players: bool,
    #[serde(default)]
    pub b_is_multiplay: bool,
    #[serde(default)]
    pub b_is_pv_p: bool,
    #[serde(default)]
    pub b_can_pickup_other_guild_death_penalty_drop: bool,
    #[serde(default)]
    pub b_enable_non_login_penalty: bool,
    #[serde(default)]
    pub b_enable_fast_travel: bool,
    #[serde(default)]
    pub b_is_start_location_select_by_map: bool,
    #[serde(default)]
    pub b_exist_player_after_logout: bool,
    #[serde(default)]
    pub b_enable_defense_other_guild_player: bool,
    #[serde(default)]
    pub rcon_enabled: bool,
    #[serde(default)]
    pub rest_api_enabled: bool,
    #[serde(default)]
    pub b_show_player_list: bool,
    #[serde(default)]
    pub b_use_auth: bool,
    #[serde(default)]
    pub b_is_use_backup_save_data: bool,
}

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AnnounceRequest {
    #[serde(default)]
    pub message: String,
}

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct KickRequest {
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub message: String,
}

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct BanRequest {
    #[serde(default)]
    pub user_id: String,
    #[serde(default)]
    pub message: String,
}

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnbanRequest {
    #[serde(default)]
    pub user_id: String,
}

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ShutdownRequest {
    #[serde(default)]
    pub waittime: u32,
    #[serde(default)]
    pub message: String,
}

/// 获取服务器基本信息（版本、名称、描述、世界 GUID）。
pub async fn fetch_info(
    rest_ip: &str,
    rest_port: u16,
    username: &str,
    password: &str,
) -> Option<PalInfoResponse> {
    let base = build_base(rest_ip, rest_port).await;
    get::<PalInfoResponse>(&base, "/v1/api/info", username, password).await
}

/// 获取服务器实时性能指标（FPS、玩家数、帧时间、运行时长等）。
pub async fn fetch_metrics(
    rest_ip: &str,
    rest_port: u16,
    username: &str,
    password: &str,
) -> Option<PalMetricsResponse> {
    let base = build_base(rest_ip, rest_port).await;
    get::<PalMetricsResponse>(&base, "/v1/api/metrics", username, password).await
}

/// 获取当前在线玩家列表。
pub async fn fetch_players(
    rest_ip: &str,
    rest_port: u16,
    username: &str,
    password: &str,
) -> Option<PalPlayerListResponse> {
    let base = build_base(rest_ip, rest_port).await;
    get::<PalPlayerListResponse>(&base, "/v1/api/players", username, password).await
}

/// 获取服务器完整设置（难度、倍率、端口、开关等）。
pub async fn fetch_settings(
    rest_ip: &str,
    rest_port: u16,
    username: &str,
    password: &str,
) -> Option<PalSettingsResponse> {
    let base = build_base(rest_ip, rest_port).await;
    get::<PalSettingsResponse>(&base, "/v1/api/settings", username, password).await
}

/// 向所有在线玩家发送全服公告。
pub async fn announce(
    rest_ip: &str,
    rest_port: u16,
    username: &str,
    password: &str,
    message: &str,
) -> Option<()> {
    let base = build_base(rest_ip, rest_port).await;
    let body = AnnounceRequest {
        message: message.to_string(),
    };
    post(&base, "/v1/api/announce", username, password, &body).await
}

/// 踢出指定玩家。
pub async fn kick_player(
    rest_ip: &str,
    rest_port: u16,
    username: &str,
    password: &str,
    user_id: &str,
    message: &str,
) -> Option<()> {
    let base = build_base(rest_ip, rest_port).await;
    let body = KickRequest {
        user_id: user_id.to_string(),
        message: message.to_string(),
    };
    post(&base, "/v1/api/kick", username, password, &body).await
}

/// 封禁指定玩家。
pub async fn ban_player(
    rest_ip: &str,
    rest_port: u16,
    username: &str,
    password: &str,
    user_id: &str,
    message: &str,
) -> Option<()> {
    let base = build_base(rest_ip, rest_port).await;
    let body = BanRequest {
        user_id: user_id.to_string(),
        message: message.to_string(),
    };
    post(&base, "/v1/api/ban", username, password, &body).await
}

/// 解封指定玩家。
pub async fn unban_player(
    rest_ip: &str,
    rest_port: u16,
    username: &str,
    password: &str,
    user_id: &str,
) -> Option<()> {
    let base = build_base(rest_ip, rest_port).await;
    let body = UnbanRequest {
        user_id: user_id.to_string(),
    };
    post(&base, "/v1/api/unban", username, password, &body).await
}

/// 手动保存世界存档。
pub async fn save_world(
    rest_ip: &str,
    rest_port: u16,
    username: &str,
    password: &str,
) -> Option<()> {
    let base = build_base(rest_ip, rest_port).await;
    post(&base, "/v1/api/save", username, password, &serde_json::Value::Null).await
}

/// 优雅关闭服务器（等待指定秒数后关闭，并发送公告）。
pub async fn shutdown_server(
    rest_ip: &str,
    rest_port: u16,
    username: &str,
    password: &str,
    waittime: u32,
    message: &str,
) -> Option<()> {
    let base = build_base(rest_ip, rest_port).await;
    let body = ShutdownRequest {
        waittime,
        message: message.to_string(),
    };
    post(&base, "/v1/api/shutdown", username, password, &body).await
}

/// 强制立即停止服务器（不等待，不广播）。
pub async fn force_stop_server(
    rest_ip: &str,
    rest_port: u16,
    username: &str,
    password: &str,
) -> Option<()> {
    let base = build_base(rest_ip, rest_port).await;
    post(&base, "/v1/api/stop", username, password, &serde_json::Value::Null).await
}
