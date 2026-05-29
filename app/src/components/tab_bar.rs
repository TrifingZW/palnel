use leptos::prelude::*;

/// 面板顶部导航标签页。
#[derive(Clone, Copy, PartialEq)]
pub enum PanelTab {
    Overview,
    Saves,
    Map,
    Settings,
}

impl PanelTab {
    pub fn label(&self) -> &'static str {
        match self {
            PanelTab::Overview => "概览",
            PanelTab::Saves => "存档",
            PanelTab::Map => "地图",
            PanelTab::Settings => "设置",
        }
    }
}

/// 导航标签栏，高亮当前激活标签。
#[component]
pub fn TabBar(
    active_tab: ReadSignal<PanelTab>,
    #[prop(into)] on_change: Callback<PanelTab>,
) -> impl IntoView {
    let tabs = [PanelTab::Overview, PanelTab::Saves, PanelTab::Map, PanelTab::Settings];

    view! {
        <nav class="tab-bar" role="tablist">
            {tabs.into_iter().map(|tab| {
                view! {
                    <button
                        class="tab-bar__tab"
                        class:tab-bar__tab--active=move || active_tab.get() == tab
                        role="tab"
                        aria-selected=move || active_tab.get() == tab
                        on:click=move |_| on_change.run(tab)
                    >
                        {tab.label()}
                    </button>
                }
            }).collect_view()}
        </nav>
    }
}
