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

/// 创建信号集合并**延迟**到浏览器端建立 `/api/sse` 的 EventSource 连接。
///
/// 必须从 `#[component]`（或其它 reactive owner）内调用，
/// 因为内部使用 `Effect::new` 将连接发起时机限制在客户端水合阶段。
pub fn create_sse() -> SseData {
    let data = SseData {
        pal_info: RwSignal::new(PalInfo::default()),
        pal_metrics: RwSignal::new(PalMetrics::default()),
        pal_player_list: RwSignal::new(PalPlayerList::default()),
        sys_metrics: RwSignal::new(SystemMetrics::default()),
        palguard_process: RwSignal::new(PalguardProcessStatus::default()),
        connected: RwSignal::new(false),
    };

    // 捕获信号副本，移入 Effect（`RwSignal` 是 Copy）
    let pal_info = data.pal_info;
    let pal_metrics = data.pal_metrics;
    let pal_player_list = data.pal_player_list;
    let sys_metrics = data.sys_metrics;
    let palguard_process = data.palguard_process;
    let connected = data.connected;

    // Effect 仅在浏览器端执行，SSR 阶段自动跳过
    Effect::new(move || {
        let es = match EventSource::new("/api/sse") {
            Ok(es) => es,
            Err(_) => return,
        };

        // 1) 连接状态回调
        {
            let c = connected;
            let on_open = Closure::wrap(Box::new(move || c.set(true)) as Box<dyn FnMut()>);
            es.set_onopen(Some(on_open.as_ref().unchecked_ref()));
            on_open.forget();
        }
        {
            let c = connected;
            let on_error = Closure::wrap(Box::new(move || c.set(false)) as Box<dyn FnMut()>);
            es.set_onerror(Some(on_error.as_ref().unchecked_ref()));
            on_error.forget();
        }

        // 2) palinfo
        {
            let sig = pal_info;
            let on_msg: Closure<dyn FnMut(MessageEvent)> =
                Closure::wrap(Box::new(move |e: MessageEvent| {
                    if let Some(json) = e.data().as_string() {
                        if let Ok(val) = serde_json::from_str::<PalInfo>(&json) {
                            let val2 = val.clone();
                            sig.set(val);
                            web_sys::console::log_1(
                                &format!("SSE palinfo received: {val2:?}").into(),
                            );
                        }
                    }
                }));
            let _ = es.add_event_listener_with_callback("palinfo", on_msg.as_ref().unchecked_ref());
            on_msg.forget();
        }

        // 3) palmetrics
        {
            let sig = pal_metrics;
            let on_msg: Closure<dyn FnMut(MessageEvent)> =
                Closure::wrap(Box::new(move |e: MessageEvent| {
                    if let Some(json) = e.data().as_string() {
                        match serde_json::from_str::<PalMetrics>(&json) {
                            Ok(val) => {
                                web_sys::console::log_1(&"SSE palmetrics received".into());
                                sig.set(val);
                            }
                            Err(err) => {
                                web_sys::console::log_1(
                                    &format!("SSE palmetrics parse error: {err}").into(),
                                );
                            }
                        }
                    }
                }));
            let _ =
                es.add_event_listener_with_callback("palmetrics", on_msg.as_ref().unchecked_ref());
            on_msg.forget();
        }

        // 4) palplayerlist
        {
            let sig = pal_player_list;
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
            let sig = sys_metrics;
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
            let sig = palguard_process;
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

        // 组件卸载时关闭连接
        on_cleanup(move || es.close());
    });

    data
}
