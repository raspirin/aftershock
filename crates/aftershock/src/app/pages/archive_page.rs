use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::{hooks::use_params, params::Params};

use crate::{MSG_ARCHIVE_PLACEHOLDER, app::{components::{MessageBox, PostMetaListGroupByTag}, server::get_posts_meta_by_tag}};

#[derive(Params, PartialEq)]
struct TagParams {
    tag: Option<String>,
}

#[derive(Clone)]
struct TagData {
    tag: String,
    posts: Vec<aftershock_bridge::PostMeta>,
}

#[derive(Clone, Copy, PartialEq)]
enum LoadingState {
    Initial,
    Loading,
    Success,
    Error,
}

#[component]
pub fn ArchivePage() -> impl IntoView {
    let _params = use_params::<TagParams>();
    let (msg, _set_msg) = signal(String::from(MSG_ARCHIVE_PLACEHOLDER));
    let (data, set_data) = signal::<Option<TagData>>(None);
    let (loading_state, set_loading_state) = signal(LoadingState::Initial);

    Effect::new(move |_| {
        let tag_opt = _params.read().as_ref().ok().and_then(|p| p.tag.clone());
        
        match tag_opt {
            None => {
                set_loading_state.set(LoadingState::Error);
                set_data.set(None);
            }
            Some(ref empty) if empty.is_empty() => {
                set_loading_state.set(LoadingState::Error);
                set_data.set(None);
            }
            Some(tag) => {
                let tag_clone = tag.clone();
                set_loading_state.set(LoadingState::Loading);
                
                spawn_local(async move {
                    match get_posts_meta_by_tag(tag_clone.clone()).await {
                        Ok(posts) => {
                            set_data.set(Some(TagData {
                                tag: tag_clone,
                                posts,
                            }));
                            set_loading_state.set(LoadingState::Success);
                        }
                        Err(_) => {
                            set_loading_state.set(LoadingState::Error);
                            set_data.set(None);
                        }
                    }
                });
            }
        }
    });

    view! {
        {move || {
            match (loading_state.get(), data.get()) {
                (LoadingState::Success, Some(ref data)) => {
                    view! { <PostMetaListGroupByTag post_meta_list=data.posts.clone() primary_tag=data.tag.clone() /> }.into_any()
                }
                (LoadingState::Error, None) => {
                    view! { <MessageBox msg=msg /> }.into_any()
                }
                _ => {
                    view! { <div></div> }.into_any()
                }
            }
        }}
    }
}
