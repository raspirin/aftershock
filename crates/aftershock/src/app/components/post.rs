use leptos::prelude::*;

#[component]
pub fn Post(post: aftershock_bridge::Post) -> impl IntoView {
    view! {
        <div class:post_container>
            <article class:post_article>
                <h1 class:post_title>{post.title}</h1>
                <p>{post.body}</p>
            </article>
        </div>
    }
}
