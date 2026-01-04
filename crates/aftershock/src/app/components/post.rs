use leptos::prelude::*;
use leptos_meta::Title;

use crate::app::components::{
    content::ContentSerif, AfTime, License, ProseContent, TagListWithoutUl,
};

#[component]
pub fn Post(post: aftershock_bridge::Post) -> impl IntoView {
    view! {
        <Title text=format!("{} - {}", post.title, crate::consts::TITLE) />
        <article class="flex flex-col gap-0">
            <h1 class="font-af-serif text-3xl font-bold">{post.title}</h1>
            <div class="grid grid-flow-col gap-2 justify-start font-af-serif font-medium">
                <AfTime timestamp=post.created_at />
                <TagList tags=post.tags />
            </div>
            <div class="my-5"></div>
            <ContentSerif>
                <ProseContent body=post.body />
            </ContentSerif>
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
