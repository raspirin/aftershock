use leptos::prelude::*;
use leptos_router::components::A;

use crate::TITLE;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="grid grid-flow-row gap-2 font-af-serif pt-4">
            <A href="/" attr:title=TITLE attr:class="text-2xl font-bold">
                {TITLE}
            </A>
            <nav>
                <ul class="grid grid-flow-col gap-4 justify-end font-semibold">
                    <li class="max-w-fit">
                        <A href="/">"主页"</A>
                    </li>
                    <li class="max-w-fit">
                        <A href="/about">"关于"</A>
                    </li>
                </ul>
            </nav>
            <div class="header-line w-full border border-site-dark"></div>
        </header>
    }
}

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer>
            <div>"Powered by Aftershock"</div>
            <div>"(c) 2025 Aspirin"</div>
        </footer>
    }
}
