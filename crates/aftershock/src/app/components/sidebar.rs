use leptos::prelude::*;

use crate::TITLE;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="grid grid-flow-row gap-2 font-af-serif pt-4">
            <a href="/" title=TITLE class="text-2xl font-bold">
                {TITLE}
            </a>
            <nav>
                <ul class="grid grid-flow-col gap-4 justify-end font-semibold">
                    <li class="max-w-fit">
                        <a href="/">"主页"</a>
                    </li>
                    <li class="max-w-fit">
                        <a href="/about">"关于"</a>
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
