use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header>
            <a href="/" title="灾后重建">
                "灾后重建"
            </a>
            <nav>
                <ul>
                    <li>
                        <a href="/">"主页"</a>
                    </li>
                    <li>
                        <a href="/about">"关于"</a>
                    </li>
                </ul>
            </nav>
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
