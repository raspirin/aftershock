use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use pages::{
    about_page::AboutPage, archive_page::ArchivePage, error_page::ErrorPage, home_page::HomePage,
    main_page::MainPage, post_page::PostPage,
};

use crate::{MSG_DATA_NOT_FOUND, TITLE};

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
            <body>
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
            <Routes fallback=move || {
                view! {
                    <MainPage>
                        <ErrorPage msg=error_msg />
                    </MainPage>
                }
            }>
                <Route
                    path=path!("/")
                    view=move || {
                        view! {
                            <MainPage>
                                <HomePage />
                            </MainPage>
                        }
                    }
                />
                <Route
                    path=path!("/about")
                    view=move || {
                        view! {
                            <MainPage>
                                <AboutPage />
                            </MainPage>
                        }
                    }
                />
                <Route
                    path=path!("/posts/:uid")
                    view=move || {
                        view! {
                            <MainPage>
                                <PostPage />
                            </MainPage>
                        }
                    }
                />
                <Route
                    path=path!("/tags/:tag")
                    view=move || {
                        view! {
                            <MainPage>
                                <ArchivePage />
                            </MainPage>
                        }
                    }
                />
            </Routes>
        </Router>
    }
}
