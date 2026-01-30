use leptos::prelude::*;
use leptos_router::{lazy_route, LazyRoute};

use crate::app::components::{MessageBox, PostMetaListGroupByTime};
use crate::app::server::get_published_posts_meta;
use crate::MSG_LOAD_DATA_FAILURE;

pub struct HomePageRoute {
    data: Resource<Result<Vec<aftershock_bridge::PostMeta>, ()>>,
    msg: ReadSignal<String>,
}

#[lazy_route]
impl LazyRoute for HomePageRoute {
    fn data() -> Self {
        let (msg, _) = signal(String::from(MSG_LOAD_DATA_FAILURE));

        let data = Resource::new(
            || (),
            |_| async move { get_published_posts_meta().await.map_err(|_| ()) },
        );

        Self { data, msg }
    }

    fn view(this: Self) -> AnyView {
        let HomePageRoute { data, msg } = this;

        view! {
            <Suspense>
                {move || {
                    data.get()
                        .map(|result| match result {
                            Ok(posts) if !posts.is_empty() => {
                                view! {
                                    <PostMetaListGroupByTime
                                        post_meta_list=posts.clone()
                                        with_summary=true
                                    />
                                }
                                    .into_any()
                            }
                            Ok(_) => view! { <MessageBox msg=msg /> }.into_any(),
                            Err(_) => view! { <MessageBox msg=msg /> }.into_any(),
                        })
                }}
            </Suspense>
        }
        .into_any()
    }
}
