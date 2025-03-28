use leptos::prelude::*;

use crate::app::components::PostMetaList;
use crate::app::server::get_published_posts_meta;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class:post_meta_container>
            <Await future=get_published_posts_meta() let:data>
                {match data {
                    Ok(s) => {
                        let posts = s.clone();
                        view! { <PostMetaList post_meta_list=posts /> }.into_any()
                    }
                    Err(_) => view! { "Fail to load posts." }.into_any(),
                }}
            </Await>
        </div>
    }
}
