use crate::app::components::*;
use leptos::prelude::*;

#[component]
pub fn MainPage(children: Children) -> impl IntoView {
    view! {
        <div class="bg-site-bg dark:bg-stone-800 text-site-text dark:text-stone-300 min-h-dvh">
            <div class="mx-auto sm:w-auto md:max-w-screen-md flex flex-col h-fit min-h-dvh px-2">
                <Header />
                <div class="my-3"></div>
                <main>{children()}</main>
            // <div class="my-6"></div>
            // <Footer />
            </div>
        </div>
    }
}
