use common::{
    pal::{PalInfo, PalMetrics, PalPlayerList, PalguardProcessStatus},
    sysinfo::SystemMetrics,
};
use leptos::prelude::*;
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{EventSource, MessageEvent};

/// 所有 SSE 推送数据的信号集合，供视图直接消费。
#[derive(Clone)]
pub struct SseData {
    pub pal_info: RwSignal<PalInfo>,
    pub pal_metrics: RwSignal<PalMetrics>,
    pub pal_player_list: RwSignal<PalPlayerList>,
    pub sys_metrics: RwSignal<SystemMetrics>,
    pub palguard_process: RwSignal<PalguardProcessStatus>,
    pub connected: RwSignal<bool>,
}

/// 创建信号集合，SSR 阶段从 `AppState` 读取真实状态初始化，客户端通过 SSE 持续更新。
pub fn create_sse() -> SseData {
    // SSR 阶段从 AppState 读取后台轮询器已采集的真实状态，确保首屏渲染不为空
    let (pal_info, pal_metrics, pal_player_list, sys_metrics, palguard_process) = {
        #[cfg(feature = "ssr")]
        {
            if let Some(state) = use_context::<common::state::AppState>() {
                (
                    state.pal_info.read().unwrap().clone(),
                    state.pal_metrics.read().unwrap().clone(),
                    state.pal_player_list.read().unwrap().clone(),
                    state.sys_metrics.read().unwrap().clone(),
                    state.palguard_process.read().unwrap().clone(),
                )
            } else {
                Default::default()
            }
        }
        #[cfg(not(feature = "ssr"))]
        {
            Default::default()
        }
    };

    let data = SseData {
        pal_info: RwSignal::new(pal_info),
        pal_metrics: RwSignal::new(pal_metrics),
        pal_player_list: RwSignal::new(pal_player_list),
        sys_metrics: RwSignal::new(sys_metrics),
        palguard_process: RwSignal::new(palguard_process),
        connected: RwSignal::new(false),
    };

    // 捕获信号副本，移入 Effect（`RwSignal` 是 Copy）
    let pi = data.pal_info;
    let pm = data.pal_metrics;
    let ppl = data.pal_player_list;
    let sm = data.sys_metrics;
    let pgp = data.palguard_process;
    let conn = data.connected;

    // Effect 仅在浏览器端执行，SSR 阶段自动跳过
    Effect::new(move || {
        let es = match EventSource::new("/api/sse") {
            Ok(es) => es,
            Err(_) => return,
        };

        // 1) 连接状态回调
        {
            let c = conn;
            let on_open = Closure::wrap(Box::new(move || c.set(true)) as Box<dyn FnMut()>);
            es.set_onopen(Some(on_open.as_ref().unchecked_ref()));
            on_open.forget();
        }
        {
            let c = conn;
            let on_error = Closure::wrap(Box::new(move || c.set(false)) as Box<dyn FnMut()>);
            es.set_onerror(Some(on_error.as_ref().unchecked_ref()));
            on_error.forget();
        }

        // 2) palinfo
        {
            let sig = pi;
            let on_msg: Closure<dyn FnMut(MessageEvent)> =
                Closure::wrap(Box::new(move |e: MessageEvent| {
                    if let Some(json) = e.data().as_string() {
                        if let Ok(val) = serde_json::from_str::<PalInfo>(&json) {
                            sig.set(val);
                        }
                    }
                }));
            let _ = es.add_event_listener_with_callback("palinfo", on_msg.as_ref().unchecked_ref());
            on_msg.forget();
        }

        // 3) palmetrics
        {
            let sig = pm;
            let on_msg: Closure<dyn FnMut(MessageEvent)> =
                Closure::wrap(Box::new(move |e: MessageEvent| {
                    if let Some(json) = e.data().as_string() {
                        if let Ok(val) = serde_json::from_str::<PalMetrics>(&json) {
                            sig.set(val);
                        }
                    }
                }));
            let _ =
                es.add_event_listener_with_callback("palmetrics", on_msg.as_ref().unchecked_ref());
            on_msg.forget();
        }

        // 4) palplayerlist
        {
            let sig = ppl;
            let on_msg: Closure<dyn FnMut(MessageEvent)> =
                Closure::wrap(Box::new(move |e: MessageEvent| {
                    if let Some(json) = e.data().as_string() {
                        if let Ok(val) = serde_json::from_str::<PalPlayerList>(&json) {
                            sig.set(val);
                        }
                    }
                }));
            let _ = es
                .add_event_listener_with_callback("palplayerlist", on_msg.as_ref().unchecked_ref());
            on_msg.forget();
        }

        // 5) systemmetrics
        {
            let sig = sm;
            let on_msg: Closure<dyn FnMut(MessageEvent)> =
                Closure::wrap(Box::new(move |e: MessageEvent| {
                    if let Some(json) = e.data().as_string() {
                        if let Ok(val) = serde_json::from_str::<SystemMetrics>(&json) {
                            sig.set(val);
                        }
                    }
                }));
            let _ = es
                .add_event_listener_with_callback("systemmetrics", on_msg.as_ref().unchecked_ref());
            on_msg.forget();
        }

        // 6) palguardprocess
        {
            let sig = pgp;
            let on_msg: Closure<dyn FnMut(MessageEvent)> =
                Closure::wrap(Box::new(move |e: MessageEvent| {
                    if let Some(json) = e.data().as_string() {
                        if let Ok(val) = serde_json::from_str::<PalguardProcessStatus>(&json) {
                            sig.set(val);
                        }
                    }
                }));
            let _ = es.add_event_listener_with_callback(
                "palguardprocess",
                on_msg.as_ref().unchecked_ref(),
            );
            on_msg.forget();
        }

        on_cleanup(move || es.close());
    });

    data
}
