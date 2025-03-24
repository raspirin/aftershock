use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path, StaticSegment,
};
use pages::main_page::MainPage;
use pages::{home_page::HomePage, post_page::PostPage};

mod components;
mod pages;
mod server;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
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
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    // let posts_index = OnceResource::new_blocking();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/aftershock.css" />

        // sets the document title
        <Title text="Aftershock" />

        // content for this welcome page
        <Router>
            <Routes fallback=|| "Page not found.".into_view()>
                <ParentRoute path=StaticSegment("/") view=MainPage>
                    <Route path=path!("") view=HomePage />
                    <Route path=path!("about") view=|| "About" />
                    <Route path=path!("posts/:uid") view=PostPage />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
