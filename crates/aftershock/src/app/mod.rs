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

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="zh-CN">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
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
