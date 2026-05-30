pub mod components;
pub mod pages;
pub mod sse;
pub mod views;

use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    path,
};

use crate::{
    components::{snackbar_host::SnackbarHost, snackbar_state::SnackbarState},
    pages::panel::Panel,
};

/// 应用 HTML shell，注入全局资源与元数据。
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" data-theme="light">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="preconnect" href="https://fonts.googleapis.com"/>
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
                <link href="https://fonts.googleapis.com/css2?family=Noto+Sans+SC:wght@400;500;600;700&display=swap" rel="stylesheet"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

/// 应用根组件，配置路由、样式、元数据及全局 Snackbar 通知系统。
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_context(SnackbarState::new());

    view! {
        <Stylesheet id="leptos" href="/pkg/palnel.css"/>

        <Title text="Palnel"/>

        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <ParentRoute path=path!("/") view=Main>
                        <Route path=path!("/") view=Panel />
                    </ParentRoute>
                </Routes>
            </main>
        </Router>

        <SnackbarHost />
    }
}

#[component]
fn Main() -> impl IntoView {
    view! {
        <Outlet />
    }
}
