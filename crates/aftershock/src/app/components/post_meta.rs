use leptos::prelude::*;

use crate::app::components::Tag;

#[component]
pub fn PostMeta(post_meta: aftershock_bridge::PostMeta) -> impl IntoView {
    let url = format!("/posts/{}", post_meta.uid);
    view! {
        <div class:post_meta>
            <h2>
                <a href=url>{post_meta.title}</a>
            </h2>
            <div class:post_meta_tags>
                <ul>
                    {post_meta
                        .tags
                        .iter()
                        .map(|tag| {
                            view! {
                                <li>
                                    <Tag tag=tag.clone() />
                                </li>
                            }
                        })
                        .collect::<Vec<_>>()}
                </ul>
            </div>
        </div>
    }
}
