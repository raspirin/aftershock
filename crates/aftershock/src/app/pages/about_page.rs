use leptos::prelude::*;
use leptos_router::{lazy_route, LazyRoute};

use crate::{
    app::{
        components::{ContentSerif, MessageBox, ProseContent},
        server::get_page,
    },
    MSG_DATA_NOT_FOUND,
};

async fn get_post(name: &str) -> Option<aftershock_bridge::Post> {
    get_page(name.into()).await.ok()
}

pub struct AboutPageRoute {
    data: Resource<Option<aftershock_bridge::Post>>,
    msg: ReadSignal<String>,
}

#[lazy_route]
impl LazyRoute for AboutPageRoute {
    fn data() -> Self {
        let (msg, _) = signal(MSG_DATA_NOT_FOUND.to_string());

        let data = Resource::new(|| (), |_| async move { get_post("about").await });

        Self { data, msg }
    }

    fn view(this: Self) -> AnyView {
        let AboutPageRoute { data, msg } = this;

        view! {
            <Suspense>
                {move || {
                    data.get()
                        .map(|result| {
                            match result {
                                Some(page) => {
                                    let body = page.clone().body;
                                    view! {
                                        <ContentSerif>
                                            <ProseContent body=body />
                                        </ContentSerif>
                                    }
                                        .into_any()
                                }
                                None => view! { <MessageBox msg=msg /> }.into_any(),
                            }
                        })
                }}
            </Suspense>
        }
        .into_any()
    }
}
