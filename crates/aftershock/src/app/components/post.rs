use leptos::prelude::*;

use crate::app::components::AfTime;

#[component]
pub fn Post(post: aftershock_bridge::Post) -> impl IntoView {
    view! {
        <div class:post_container>
            <article class:post_article>
                <h1 class:post_title>{post.title}</h1>
                <AfTime timestamp=post.created_at />
                <TagList tags=post.tags />
                <div class:post_content inner_html=post.body />
            </article>
        </div>
    }
}

#[component]
pub fn TagList(tags: Vec<String>) -> impl IntoView {
    view! {
        <div class:post_tag_list>
            <ul>
                {tags
                    .into_iter()
                    .map(|tag| {
                        view! {
                            <li>
                                <Tag tag=tag />
                            </li>
                        }
                    })
                    .collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

#[component]
pub fn Tag(tag: String) -> impl IntoView {
    let url = "/";
    view! {
        <div class:post_tag>
            <a href=url>{tag}</a>
        </div>
    }
}
