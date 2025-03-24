use leptos::prelude::*;

use crate::app::components::PostMeta;
use crate::app::server::get_published_posts_meta;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class:post_meta_container>
            <Await future=get_published_posts_meta() let:data>
                {match data {
                    Ok(s) => {
                        view! {
                            <ul class:post_meta_list>
                                {s
                                    .iter()
                                    .map(|meta| {
                                        view! {
                                            <li>
                                                <PostMeta post_meta=meta.clone() />
                                            </li>
                                        }
                                    })
                                    .collect::<Vec<_>>()}
                            </ul>
                        }
                            .into_any()
                    }
                    Err(_) => view! { "Fail to load posts." }.into_any(),
                }}
            </Await>
        </div>
    }
}
