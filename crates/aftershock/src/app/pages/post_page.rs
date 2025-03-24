use crate::app::{components::Post, server::get_post_by_uid};
use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};

#[derive(Params, PartialEq)]
struct PostParams {
    pub uid: Option<String>,
}

#[component]
pub fn PostPage() -> impl IntoView {
    let params = use_params::<PostParams>();
    let post = Resource::new(
        move || {
            params
                .read()
                .as_ref()
                .ok()
                .map(|p| p.uid.clone())
                .unwrap_or_default()
        },
        move |uid| async move {
            match uid {
                None => None,
                Some(empty) if empty.is_empty() => None,
                Some(uid) => get_post_by_uid(uid).await.ok(),
            }
        },
    );

    view! {
        // TODO: replace this with Await
        <Suspense fallback=move || {
            view! { "Loading..." }
        }>
            {move || Suspend::new(async move {
                match post.await.clone() {
                    None => Either::Left(view! { "Post not found." }),
                    Some(post) => Either::Right(view! { <Post post=post /> }),
                }
            })}
        </Suspense>
    }
}
