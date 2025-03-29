use leptos::prelude::*;

#[component]
pub fn ContentSerif(children: Children) -> impl IntoView {
    view! { <div class="font-af-serif font-medium text-xl">{children()}</div> }
}

#[component]
pub fn ContentSans(children: Children) -> impl IntoView {
    view! { <div class="font-af-sans font-medium text-xl">{children()}</div> }
}
