use crate::app::components::*;
use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn MainPage() -> impl IntoView {
    view! {
        <Header />
        <main>
            <Outlet />
        </main>
        <Footer />
    }
}
