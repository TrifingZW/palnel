use common::pal::{PalInfo, PalMetrics, PalPlayer as CommonPlayer, PalPlayerList};
use common::sysinfo::{DiskInfo, NetworkInfo, SystemMetrics};
use services::pal_rest::{PalInfoResponse, PalMetricsResponse, PalPlayer, PalPlayerListResponse};

/// 将 Palworld REST API 响应转为前端数据类型。
pub fn pal_info_from_response(res: PalInfoResponse) -> PalInfo {
    PalInfo {
        version: res.version,
        server_name: res.servername,
        description: res.description,
        world_guid: res.worldguid,
    }
}

/// 将 Palworld REST API 指标响应转为前端数据类型。
pub fn pal_metrics_from_response(res: PalMetricsResponse) -> PalMetrics {
    PalMetrics {
        server_fps: res.serverfps,
        current_player_num: res.currentplayernum,
        max_player_num: res.maxplayernum,
        server_frame_time: res.serverframetime,
        uptime: res.uptime,
        base_camp_num: res.basecampnum,
        days: res.days,
    }
}

/// 将 Palworld REST API 玩家数据转为前端数据类型。
pub fn pal_player_from_response(res: PalPlayer) -> CommonPlayer {
    CommonPlayer {
        name: res.name,
        account_name: res.account_name,
        player_id: res.player_id,
        user_id: res.user_id,
        ip: res.ip,
        ping: res.ping,
        location_x: res.location_x,
        location_y: res.location_y,
        level: res.level,
        building_count: res.building_count,
    }
}

/// 将 Palworld REST API 玩家列表响应转为前端数据类型。
pub fn pal_player_list_from_response(res: PalPlayerListResponse) -> PalPlayerList {
    PalPlayerList {
        players: res.players.into_iter().map(pal_player_from_response).collect(),
    }
}

/// 将 `services` 系统指标转为前端通用类型。
pub fn system_metrics_from_svc(m: services::sys::SystemMetrics) -> SystemMetrics {
    SystemMetrics {
        cpu_usage: m.cpu_usage,
        cpu_cores: m.cpu_cores,
        cpu_model: m.cpu_model,
        memory_total: m.memory_total,
        memory_used: m.memory_used,
        swap_total: m.swap_total,
        swap_used: m.swap_used,
        uptime: m.uptime,
        load_avg_one: m.load_avg_one,
        load_avg_five: m.load_avg_five,
        load_avg_fifteen: m.load_avg_fifteen,
        disks: m
            .disks
            .into_iter()
            .map(|d| DiskInfo {
                name: d.name,
                mount_point: d.mount_point,
                total: d.total,
                used: d.used,
            })
            .collect(),
        networks: m
            .networks
            .into_iter()
            .map(|n| NetworkInfo {
                name: n.name,
                rx_bytes: n.rx_bytes,
                tx_bytes: n.tx_bytes,
            })
            .collect(),
        process_count: m.process_count,
        os_name: m.os_name,
        os_version: m.os_version,
        kernel_version: m.kernel_version,
        hostname: m.hostname,
        collected_at: m.collected_at,
    }
}
