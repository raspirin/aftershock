use crate::{
    app::{
        components::{MessageBox, Post},
        server::get_post_by_uid,
    },
    MSG_LOAD_DATA_FAILURE,
};
use leptos::either::Either;
use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};

#[derive(Params, PartialEq)]
struct PostParams {
    pub uid: Option<String>,
}

async fn get_post() -> Option<aftershock_bridge::Post> {
    let params = use_params::<PostParams>();
    let uid = params
        .read()
        .as_ref()
        .ok()
        .map(|p| p.uid.clone())
        .unwrap_or_default();
    match uid {
        None => None,
        Some(empty) if empty.is_empty() => None,
        Some(uid) => get_post_by_uid(uid).await.ok(),
    }
}

#[component]
pub fn PostPage() -> impl IntoView {
    let (msg, _) = signal(String::from(MSG_LOAD_DATA_FAILURE));

    view! {
        <Await future=get_post() let:data>
            {match data {
                Some(post) => Either::Right(view! { <Post post=post.clone() /> }),
                None => Either::Left(view! { <MessageBox msg=msg /> }),
            }}
        </Await>
    }
}
