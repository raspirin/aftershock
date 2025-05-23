use crate::utils::datetime::{CommonTime, UnixTimestampConverter};
use leptos::{either::Either, prelude::*};

use crate::{
    app::components::TagListWithoutUl,
    utils::{datetime::StaticFormattedDateTime, group_by},
};

#[component]
pub fn PostMetaList(
    post_meta_list: Vec<aftershock_bridge::PostMeta>,
    with_summary: bool,
) -> impl IntoView {
    let posts = post_meta_list
        .into_iter()
        .map(|post| {
            (
                // StaticFormattedDateTime::from_timestamp(post.created_at),
                CommonTime::<StaticFormattedDateTime>::from_timestamp(post.created_at),
                post,
            )
        })
        .collect::<Vec<_>>();
    let posts = group_by(posts, |post| post.0.date_triplet.year, |post| post.clone());
    let mut posts = posts.into_iter().collect::<Vec<_>>();
    posts.sort_by(|lhs, rhs| rhs.0.cmp(&lhs.0));

    view! {
        <div class="flex flex-col gap-4 font-af-serif">
            {posts
                .into_iter()
                .map(|(year, x)| {
                    view! {
                        <PostMetaSection year=year post_meta_list=x with_summary=with_summary />
                    }
                })
                .collect_view()}
        </div>
    }
}

#[component]
pub fn PostMetaSection<C: UnixTimestampConverter>(
    year: i32,
    post_meta_list: Vec<(CommonTime<C>, aftershock_bridge::PostMeta)>,
    with_summary: bool,
) -> impl IntoView {
    view! {
        <section class="flex flex-col gap-4">
            <h1 class="font-bold text-4xl">{year}</h1>
            {post_meta_list
                .into_iter()
                .map(|(time, meta)| {
                    view! { <PostMeta time=time post_meta=meta with_summary=with_summary /> }
                })
                .collect_view()}
        </section>
    }
}

#[component]
pub fn PostMeta<C: UnixTimestampConverter>(
    time: CommonTime<C>,
    post_meta: aftershock_bridge::PostMeta,
    with_summary: bool,
) -> impl IntoView {
    let url = format!("/posts/{}", post_meta.uid);
    let human_time = format!(
        "{} {}",
        time.date_triplet.month_symbol().unwrap(),
        time.date_triplet.day
    );
    let machine_time = time.machine_friendly;

    view! {
        <div class="flex flex-col gap-1 sm:gap-2 md:gap-4">
            <div class="flex flex-row items-center gap-4 w-full font-semibold">
                <time datetime=machine_time class="sm:pr-8 md:pr-16 text-right w-fit flex-shrink-0">
                    {human_time}
                </time>
                <h2 class="flex-grow">
                    <a href=url>{post_meta.title}</a>
                </h2>
                <ul class="flex flex-row gap-1 ml-auto font-medium">
                    <TagListWithoutUl tags=post_meta.tags />
                </ul>
            </div>
            {match with_summary {
                true => {
                    Either::Right(view! { <PostMetaSummary>{post_meta.summary}</PostMetaSummary> })
                }
                false => Either::Left(view! {}),
            }}
        </div>
    }
}

#[component]
pub fn PostMetaSummary(children: Children) -> impl IntoView {
    view! { <div class="font-medium mx-1">{children()}</div> }
}
