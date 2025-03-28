use leptos::prelude::*;

use crate::app::components::{AfTime, License, TagListWithoutUl};

#[component]
pub fn Post(post: aftershock_bridge::Post) -> impl IntoView {
    view! {
        <article class="flex flex-col gap-0">
            <h1 class="font-af-serif text-3xl font-bold">{post.title}</h1>
            <div class="grid grid-flow-col gap-2 justify-start font-af-serif font-medium">
                <AfTime timestamp=post.created_at />
                <TagList tags=post.tags />
            </div>
            <div class="my-5"></div>
            <div class="font-af-serif">
                <div
                    class="prose prose-stone max-w-none prose-table:mx-2 prose-pre:font-af-mono prose-a:no-underline prose-a:text-blue-500 prose-a:hover:underline"
                    inner_html=post.body
                />
            </div>
            <div class="flex flex-col justify-center items-center">
                <div class="my-4" />
                <div class="font-af-serif font-medium italic justify-center max-w-fit">fin</div>
            </div>
            <License />
        </article>
    }
}

#[component]
pub fn TagList(tags: Vec<String>) -> impl IntoView {
    view! {
        <ul class="grid grid-flow-col gap-0 justify-start max-w-fit italic">
            <TagListWithoutUl tags=tags />
        </ul>
    }
}
