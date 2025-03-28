use leptos::prelude::*;

#[component]
pub fn Tag(tag: String) -> impl IntoView {
    let url = "/";
    view! {
        <div class:post_tag>
            <a href=url>#{tag}</a>
        </div>
    }
}
