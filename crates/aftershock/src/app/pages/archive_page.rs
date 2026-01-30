use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};

use crate::{
    app::{
        components::{MessageBox, PostMetaListGroupByTag},
        server::get_posts_meta_by_tag,
    },
    MSG_ARCHIVE_PLACEHOLDER,
};

#[derive(Params, PartialEq)]
struct TagParams {
    tag: Option<String>,
}

#[component]
pub fn ArchivePage() -> impl IntoView {
    let params = use_params::<TagParams>();
    let (msg, _set_msg) = signal(String::from(MSG_ARCHIVE_PLACEHOLDER));

    let data = Resource::new(
        move || params.read().as_ref().ok().and_then(|p| p.tag.clone()),
        |tag| async move {
            match tag {
                None => Err(()),
                Some(ref empty) if empty.is_empty() => Err(()),
                Some(tag) => get_posts_meta_by_tag(tag).await.map_err(|_| ()),
            }
        },
    );

    view! {
        <Suspense>
            {move || {
                data.get()
                    .map(|result| match result {
                        Ok(posts) => {
                            view! {
                                <PostMetaListGroupByTag
                                    post_meta_list=posts.clone()
                                    primary_tag=params
                                        .read()
                                        .as_ref()
                                        .ok()
                                        .and_then(|p| p.tag.clone())
                                        .unwrap_or_default()
                                />
                            }
                                .into_any()
                        }
                        Err(_) => view! { <MessageBox msg=msg /> }.into_any(),
                    })
            }}
        </Suspense>
    }
}
