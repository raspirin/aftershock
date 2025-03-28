use leptos::prelude::*;

use crate::app::components::{MessageBox, PostMetaList};
use crate::app::server::get_published_posts_meta;
use crate::MSG_LOAD_DATA_FAILURE;

#[component]
pub fn HomePage() -> impl IntoView {
    let (msg, _) = signal(String::from(MSG_LOAD_DATA_FAILURE));

    view! {
        <div class:post_meta_container>
            <Await future=get_published_posts_meta() let:data>
                {match data {
                    Ok(s) => {
                        let posts = s.clone();
                        view! { <PostMetaList post_meta_list=posts /> }.into_any()
                    }
                    Err(_) => view! { <MessageBox msg=msg /> }.into_any(),
                }}
            </Await>
        </div>
    }
}
