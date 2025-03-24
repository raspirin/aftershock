use leptos::prelude::*;

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
                                    <PostMetaTag tag=tag />
                                </li>
                            }
                        })
                        .collect::<Vec<_>>()}
                </ul>
            </div>
        </div>
    }
}

#[component]
fn PostMetaTag<'a>(tag: &'a str) -> impl IntoView {
    let tag = tag.to_string();
    view! { <div class:post_meta_tag>{tag}</div> }
}
