use std::collections::HashMap;

use components::sidebar::{Footer, Header};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    path, StaticSegment,
};
use model::PostsIndex;

mod components;
mod model;
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
                    <Route path=path!("") view=|| "HomePage Content" />
                    <Route path=path!("posts") view=|| "Posts" />
                    <Route path=path!("about") view=|| "About" />
                </ParentRoute>
            </Routes>
        </Router>
    }
}

// #[server]
// async fn get_posts_index() -> Result<PostsIndex, ServerFnError> {
//     let mut map = HashMap::new();

//     PostsIndex {
//         map
//     }
// }

/// Renders the home page of your application.
#[component]
fn MainPage() -> impl IntoView {
    view! {
        <Header />
        <main>
            <Outlet />
        </main>
        <Footer />
    }
}
