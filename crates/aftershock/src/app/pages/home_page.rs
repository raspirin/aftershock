use leptos::prelude::*;

use crate::app::components::{MessageBox, PostMetaList};
use crate::app::server::get_published_posts_meta;
use crate::{MSG_ARCHIVE_PLACEHOLDER, MSG_LOAD_DATA_FAILURE};

#[component]
pub fn HomePage() -> impl IntoView {
    let (msg, set_msg) = signal(String::from(MSG_LOAD_DATA_FAILURE));

    view! {
        <div class:post_meta_container>
            <Await future=get_published_posts_meta() let:data>
                {match data {
                    Ok(s) if !s.is_empty() => {
                        let posts = s.clone();
                        view! { <PostMetaList post_meta_list=posts /> }.into_any()
                    }
                    Ok(_) => {
                        *set_msg.write() = MSG_ARCHIVE_PLACEHOLDER.into();
                        {
                            view! { <MessageBox msg=msg /> }
                        }
                            .into_any()
                    }
                    Err(_) => {
                        *set_msg.write() = MSG_LOAD_DATA_FAILURE.into();
                        view! { <MessageBox msg=msg /> }.into_any()
                    }
                }}
            </Await>
        </div>
    }
}
