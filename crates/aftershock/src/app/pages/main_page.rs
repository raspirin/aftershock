use crate::app::components::*;
use leptos::prelude::*;

#[component]
pub fn MainPage(children: Children) -> impl IntoView {
    view! {
        <div class="bg-site-bright text-site-dark min-h-screen">
            <div class="mx-auto max-w-screen-md flex flex-col h-dvh px-2">
                <Header />
                <div class="my-3"></div>
                <main>{children()}</main>
            // <div class="my-6"></div>
            // <Footer />
            </div>
        </div>
    }
}
