use common::pal::PalPlayer;
use leptos::prelude::*;

use crate::components::{
    icon_button::IconButton,
    ping_badge::PingBadge,
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

    view! {
        <div class="pal-player-list">
            {move || {
                players.get().into_iter().enumerate().map(|(i, p)| {
                    let initial = p.name.chars().next().unwrap_or('?').to_uppercase().to_string();
                    let hue = p.name.bytes().fold(0u32, |acc, b| acc.wrapping_add(b as u32)) % 360;

                    let ip_icon = view! {
                        <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
                            <path fill-rule="evenodd" d="M7.646 1.146a.5.5 0 0 1 .708 0l6 6a.5.5 0 0 1 0 .708l-6 6a.5.5 0 0 1-.708-.708L12.793 8 7.646 2.854a.5.5 0 0 1 0-.708z"/>
                            <path fill-rule="evenodd" d="M1.646 1.146a.5.5 0 0 1 .708 0l6 6a.5.5 0 0 1 0 .708l-6 6a.5.5 0 0 1-.708-.708L6.793 8 1.646 2.854a.5.5 0 0 1 0-.708z"/>
                        </svg>
                    }.into_any();

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
                                <IconButton on_click=move |_| {}>
                                    <svg width="18" height="18" viewBox="0 0 16 16" fill="currentColor">
                                        <path fill-rule="evenodd" d="M6 8a.5.5 0 0 0 .5.5h5.793l-2.147 2.146a.5.5 0 0 0 .708.708l3-3a.5.5 0 0 0 0-.708l-3-3a.5.5 0 0 0-.708.708L12.293 7.5H6.5A.5.5 0 0 0 6 8zm-2.5 7a.5.5 0 0 1-.5-.5v-13a.5.5 0 0 1 1 0v13a.5.5 0 0 1-.5.5z"/>
                                    </svg>
                                </IconButton>
                                <IconButton on_click=move |_| {}>
                                    <svg width="18" height="18" viewBox="0 0 16 16" fill="currentColor">
                                        <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zm0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16z"/>
                                        <path d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708z"/>
                                    </svg>
                                </IconButton>
                                <IconButton on_click=move |_| {}>
                                    <svg width="18" height="18" viewBox="0 0 16 16" fill="currentColor">
                                        <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zm0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16z"/>
                                        <path d="M10.97 4.97a.235.235 0 0 0-.02.022L7.477 9.417 5.384 7.323a.75.75 0 0 0-1.06 1.06L6.97 11.03a.75.75 0 0 0 1.079-.02l3.992-4.99a.75.75 0 0 0-1.071-1.05z"/>
                                    </svg>
                                </IconButton>
                            </span>
                        </div>
                    }
                }).collect_view()
            }}
        </div>
    }
}
