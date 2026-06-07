use common::pal::PalPlayer;
use leptos::prelude::*;
use leptos::task::spawn_local;
use rpc::palguard::{palguard_ban, palguard_kick};

use crate::components::{
    hint::Hint,
    icon_button::IconButton,
    ping_badge::PingBadge,
    snackbar::SnackbarVariant,
    snackbar_state::SnackbarState,
    tag::{Tag, TagColor, TagSize},
};

/// 帕鲁玩家列表，点击行选中，信息左右分布，右侧置操作图标按钮。
#[component]
pub fn PalPlayerList(
    #[prop(into)] players: Signal<Vec<PalPlayer>>,
    #[prop(optional)] selected: Option<RwSignal<Vec<usize>>>,
) -> impl IntoView {
    let inner_selected = RwSignal::new(Vec::<usize>::new());
    let selected = selected.unwrap_or(inner_selected);

    let toggle = move |idx: usize| {
        selected.update(|sel| {
            if let Some(pos) = sel.iter().position(|&x| x == idx) {
                sel.remove(pos);
            } else {
                sel.push(idx);
            }
        });
    };

    let snackbar = SnackbarState::use_state();

    view! {
        <div class="pal-player-list">
            {move || {
                let mut all = vec![
                    PalPlayer {
                        name: "TestPlayer_Alpha".into(),
                        account_name: "alpha_account".into(),
                        player_id: "test_alpha_001".into(),
                        user_id: "steam_test_alpha_001".into(),
                        ip: "192.168.1.100".into(),
                        ping: 25.0,
                        location_x: -12345.0,
                        location_y: 67890.0,
                        level: 42,
                        building_count: 3,
                    },
                    PalPlayer {
                        name: "TestPlayer_Beta".into(),
                        account_name: "beta_account".into(),
                        player_id: "test_beta_002".into(),
                        user_id: "steam_test_beta_002".into(),
                        ip: "192.168.1.101".into(),
                        ping: 68.0,
                        location_x: 54321.0,
                        location_y: -9876.0,
                        level: 18,
                        building_count: 1,
                    },
                ];
                all.extend(players.get());
                all.into_iter().enumerate().map(|(i, p)| {
                    let initial = p.name.chars().next().unwrap_or('?').to_uppercase().to_string();
                    let hue = p.name.bytes().fold(0u32, |acc, b| acc.wrapping_add(b as u32)) % 360;

                    let ip_icon = view! {
                        <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
                            <path fill-rule="evenodd" d="M7.646 1.146a.5.5 0 0 1 .708 0l6 6a.5.5 0 0 1 0 .708l-6 6a.5.5 0 0 1-.708-.708L12.793 8 7.646 2.854a.5.5 0 0 1 0-.708z"/>
                            <path fill-rule="evenodd" d="M1.646 1.146a.5.5 0 0 1 .708 0l6 6a.5.5 0 0 1 0 .708l-6 6a.5.5 0 0 1-.708-.708L6.793 8 1.646 2.854a.5.5 0 0 1 0-.708z"/>
                        </svg>
                    }.into_any();

                    // 1) 为各按钮捕获必要的玩家数据副本
                    let uid = p.user_id.clone();
                    let name = p.name.clone();
                    let coords = format!("X:{:.0} Y:{:.0}", p.location_x, p.location_y);

                    // 2) 传送到玩家（复制坐标）
                    let tele_name = name.clone();
                    let tele_coords = coords.clone();
                    let tele_snack = snackbar;
                    let on_teleport = move |_| {
                        tele_snack.show(
                            format!("{} 坐标: {}", tele_name, tele_coords),
                            SnackbarVariant::Info,
                        );
                    };

                    // 3) 踢出玩家
                    let kick_uid = uid.clone();
                    let kick_name = name.clone();
                    let kick_snack = snackbar;
                    let on_kick = move |_| {
                        let uid = kick_uid.clone();
                        let name = kick_name.clone();
                        let snack = kick_snack;
                        spawn_local(async move {
                            match palguard_kick(uid, String::new()).await {
                                Ok(()) => snack.show(
                                    format!("已踢出玩家 {}", name),
                                    SnackbarVariant::Success,
                                ),
                                Err(e) => snack.show(e.to_string(), SnackbarVariant::Danger),
                            }
                        });
                    };

                    // 4) 封禁玩家
                    let ban_uid = uid.clone();
                    let ban_name = name.clone();
                    let ban_snack = snackbar;
                    let on_ban = move |_| {
                        let uid = ban_uid.clone();
                        let name = ban_name.clone();
                        let snack = ban_snack;
                        spawn_local(async move {
                            match palguard_ban(uid, String::new()).await {
                                Ok(()) => snack.show(
                                    format!("已封禁玩家 {}", name),
                                    SnackbarVariant::Success,
                                ),
                                Err(e) => snack.show(e.to_string(), SnackbarVariant::Danger),
                            }
                        });
                    };

                    view! {
                        <div
                            class="pal-player-list__item"
                            class:pal-player-list__item--selected=move || selected.get().contains(&i)
                            on:click=move |_| toggle(i)
                        >
                            <span class="pal-player-list__avatar" style=format!("background:hsl({},55%,70%)", hue)>
                                {initial}
                            </span>
                            <div class="pal-player-list__body">
                                <div class="pal-player-list__row">
                                    <span class="pal-player-list__group">
                                        <span class="pal-player-list__name">{p.name.clone()}</span>
                                        <Tag text=format!("Lv.{}", p.level) size=TagSize::Small color=TagColor::Info />
                                    </span>
                                </div>
                                <div class="pal-player-list__row">
                                    <span class="pal-player-list__group">
                                        <span class="pal-player-list__ip">
                                            {ip_icon}
                                            {p.ip.clone()}
                                        </span>
                                        <PingBadge ping=p.ping />
                                    </span>
                                </div>
                            </div>
                            <span class="pal-player-list__actions">
                                <Hint text="坐标信息".to_string()>
                                    <IconButton on_click=Callback::new(on_teleport)>
                                        <svg width="18" height="18" viewBox="0 0 16 16" fill="currentColor">
                                            <path fill-rule="evenodd" d="M6 8a.5.5 0 0 0 .5.5h5.793l-2.147 2.146a.5.5 0 0 0 .708.708l3-3a.5.5 0 0 0 0-.708l-3-3a.5.5 0 0 0-.708.708L12.293 7.5H6.5A.5.5 0 0 0 6 8zm-2.5 7a.5.5 0 0 1-.5-.5v-13a.5.5 0 0 1 1 0v13a.5.5 0 0 1-.5.5z"/>
                                        </svg>
                                    </IconButton>
                                </Hint>
                                <Hint text="踢出玩家".to_string()>
                                    <IconButton on_click=Callback::new(on_kick)>
                                        <svg width="18" height="18" viewBox="0 0 16 16" fill="currentColor">
                                            <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zm0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16z"/>
                                            <path d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708z"/>
                                        </svg>
                                    </IconButton>
                                </Hint>
                                <Hint text="封禁玩家".to_string()>
                                    <IconButton on_click=Callback::new(on_ban)>
                                        <svg width="18" height="18" viewBox="0 0 16 16" fill="currentColor">
                                            <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zm0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16z"/>
                                            <path d="M10.97 4.97a.235.235 0 0 0-.02.022L7.477 9.417 5.384 7.323a.75.75 0 0 0-1.06 1.06L6.97 11.03a.75.75 0 0 0 1.079-.02l3.992-4.99a.75.75 0 0 0-1.071-1.05z"/>
                                        </svg>
                                    </IconButton>
                                </Hint>
                            </span>
                        </div>
                    }
                }).collect_view()
            }}
        </div>
    }
}
