use leptos::prelude::*;

use crate::{
    app::components::TagListWithoutUl,
    utils::{datetime::PreformattedDateTime, group_by},
};

#[component]
pub fn PostMetaList(post_meta_list: Vec<aftershock_bridge::PostMeta>) -> impl IntoView {
    let posts = post_meta_list
        .into_iter()
        .map(|post| (PreformattedDateTime::from_timestamp(post.created_at), post))
        .collect::<Vec<_>>();
    let posts = group_by(posts, |post| post.0.year, |post| post.clone());
    let mut posts = posts.into_iter().collect::<Vec<_>>();
    posts.sort_by(|lhs, rhs| rhs.0.cmp(&lhs.0));

    view! {
        <div class="flex flex-col gap-4 font-af-serif">
            {posts
                .into_iter()
                .map(|(year, x)| view! { <PostMetaSection year=year post_meta_list=x /> })
                .collect_view()}
        </div>
    }
}

#[component]
pub fn PostMetaSection(
    year: i32,
    post_meta_list: Vec<(PreformattedDateTime, aftershock_bridge::PostMeta)>,
) -> impl IntoView {
    view! {
        <section class="flex flex-col gap-4">
            <h1 class="font-bold text-4xl">{year}</h1>
            {post_meta_list
                .into_iter()
                .map(|(time, meta)| view! { <PostMeta time=time post_meta=meta /> })
                .collect_view()}
        </section>
    }
}

#[component]
pub fn PostMeta(
    time: PreformattedDateTime,
    post_meta: aftershock_bridge::PostMeta,
) -> impl IntoView {
    let url = format!("/posts/{}", post_meta.uid);
    let human_time = format!("{} {}", time.month_to_abbr(), time.day);
    let machine_time = time.machine_friendly;

    view! {
        <div class="flex flex-row items-center gap-4 w-full font-semibold">
            <time datetime=machine_time class="md:pr-8 lg:pr-16 text-right w-fit flex-shrink-0">
                {human_time}
            </time>
            <h2 class="flex-grow">
                <a href=url>{post_meta.title}</a>
            </h2>
            <ul class="flex flex-row gap-1 ml-auto font-medium">
                <TagListWithoutUl tags=post_meta.tags />
            </ul>
        </div>
    }
}
