use leptos::prelude::*;
use leptos_router::{
    hooks::use_params,
    lazy_route,
    params::{Params, ParamsError},
    LazyRoute,
};

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

pub struct ArchivePageRoute {
    params: Memo<Result<TagParams, ParamsError>>,
    data: Resource<Result<Vec<aftershock_bridge::PostMeta>, ()>>,
    msg: ReadSignal<String>,
}

#[lazy_route]
impl LazyRoute for ArchivePageRoute {
    fn data() -> Self {
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

        Self { params, data, msg }
    }

    fn view(this: Self) -> AnyView {
        let ArchivePageRoute { params, data, msg } = this;
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
        .into_any()
    }
}