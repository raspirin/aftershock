use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes}, path, StaticSegment
};

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
                </ParentRoute>
            </Routes>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn MainPage() -> impl IntoView {
    // Creates a reactive value to update the button

    view! {
        <nav>
            <ul>
                <li>
                    <a href="/" title="Home">
                        "Home"
                    </a>
                </li>
                <li>
                    <a href="/posts" title="Posts">
                        "Posts"
                    </a>
                </li>
            </ul>
        </nav>
        <main>
            <Outlet />
        </main>
    }
}
