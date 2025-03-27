use crate::app::components::*;
use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn MainPage() -> impl IntoView {
    view! {
        <div class="bg-site-bright text-site-dark min-h-screen">
            <div class="mx-auto max-w-screen-sm flex flex-col">
                <Header />
                <div class="my-4"></div>
                <main>
                    <Outlet />
                </main>
                <Footer />
            </div>
        </div>
    }
}
