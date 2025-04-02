use leptos::prelude::*;

#[component]
pub fn ContentSerif(children: Children) -> impl IntoView {
    view! { <div class="font-af-serif font-medium text-xl">{children()}</div> }
}

#[component]
pub fn ContentSans(children: Children) -> impl IntoView {
    view! { <div class="font-af-sans font-medium text-xl">{children()}</div> }
}

#[component]
pub fn ProseContent(body: String) -> impl IntoView {
    view! {
        <div
            class="max-w-none prose md:prose-lg prose-stone dark:prose-invert prose-table:mx-2 prose-pre:font-af-mono prose-a:no-underline prose-a:text-blue-500 prose-a:hover:underline prose-table:overflow-x-auto prose-table:block prose-table:md:table"
            inner_html=body
        />
    }
}
