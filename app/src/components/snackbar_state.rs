use leptos::prelude::*;

use crate::components::snackbar::SnackbarVariant;

/// 全局 Snackbar 通知状态，通过 Leptos context 向整个组件树提供。
///
/// 调用 `show` 推送消息；`SnackbarHost` 负责挂载渲染与自动关闭。
#[derive(Clone, Copy)]
pub struct SnackbarState {
    pub current: RwSignal<Option<(String, SnackbarVariant)>>,
}

impl SnackbarState {
    /// 创建新的全局状态实例。
    pub fn new() -> Self {
        Self {
            current: RwSignal::new(None),
        }
    }

    /// 推送一条 Snackbar 通知。若已有消息，将被替换（单条覆盖策略）。
    pub fn show(&self, text: impl Into<String>, variant: SnackbarVariant) {
        self.current.set(Some((text.into(), variant)));
    }

    /// 主动清除当前消息。
    pub fn dismiss(&self) {
        self.current.set(None);
    }

    /// 从当前 reactive context 中获取全局 `SnackbarState`。
    ///
    /// 调用方必须位于已通过 `provide_context` 注入该状态的组件子树内。
    pub fn use_state() -> Self {
        use_context::<Self>()
            .expect("SnackbarState 未在 context 中提供，请确保已在祖先组件调用 provide_context")
    }
}
