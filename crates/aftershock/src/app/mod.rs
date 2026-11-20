use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path, Lazy,
};
use pages::{
    about_page::AboutPageRoute, error_page::ErrorPage, home_page::HomePageRoute,
    main_page::MainPage,
};

use crate::{
    app::pages::{archive_page::ArchivePageRoute, post_page::PostPageRoute},
    MSG_DATA_NOT_FOUND, TITLE,
};

mod components;
mod pages;
mod server;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = window, js_name = aftershockSetAnubisDestination)]
    fn set_anubis_destination(
        read_destination: wasm_bindgen::JsValue,
    ) -> Result<(), wasm_bindgen::JsValue>;
}

#[component]
fn AnubisRecovery() -> impl IntoView {
    #[cfg(feature = "hydrate")]
    {
        let url = leptos_router::hooks::use_url();
        let read_destination =
            wasm_bindgen::closure::Closure::<dyn Fn() -> String>::new(move || {
                let url = url.read_untracked();
                let mut path = url.path().to_owned();
                if !url.search().is_empty() {
                    path.push('?');
                    path.push_str(url.search());
                }
                let hash = url.hash();
                if !hash.is_empty() && !hash.starts_with('#') {
                    path.push('#');
                }
                path.push_str(hash);
                path
            });
        let _ = set_anubis_destination(read_destination.into_js_value());
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    let aftershock_version = env!("CARGO_PKG_VERSION");
    #[cfg(feature = "ssr")]
    let anubis_nonce = leptos::nonce::use_nonce();
    #[cfg(not(feature = "ssr"))]
    let anubis_nonce = None::<String>;

    view! {
        <!DOCTYPE html>
        <html lang="zh-CN">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <meta name="generator" content=format!("aftershock v{aftershock_version}")/>
                <AutoReload options=options.clone() />
                <script nonce=anubis_nonce inner_html=include_str!("anubis_recovery.js")></script>
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body class="bg-site-bg dark:bg-stone-800">
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let (error_msg, _) = signal(String::from(MSG_DATA_NOT_FOUND));

    view! {
        <Stylesheet id="leptos" href="/pkg/aftershock.css" />

        <Title text=TITLE />

        <Router>
            <AnubisRecovery />
            <MainPage>
                <Routes fallback=move || {
                    view! { <ErrorPage msg=error_msg /> }
                }>
                    <Route path=path!("/") view={Lazy::<HomePageRoute>::new()} />
                    <Route path=path!("/about") view={Lazy::<AboutPageRoute>::new()} />
                    <Route path=path!("/posts/:uid") view={Lazy::<PostPageRoute>::new()} />
                    <Route path=path!("/tags/:tag") view={Lazy::<ArchivePageRoute>::new()} />
                </Routes>
            </MainPage>
        </Router>
    }
}
