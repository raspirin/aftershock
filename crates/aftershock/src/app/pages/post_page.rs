use crate::{
    app::{
        components::{MessageBox, Post},
        server::get_post_by_uid,
    },
    MSG_LOAD_DATA_FAILURE,
};
use leptos::prelude::*;
use leptos_router::{
    hooks::use_params,
    lazy_route,
    params::Params,
    LazyRoute,
};

#[derive(Params, PartialEq)]
struct PostParams {
    pub uid: Option<String>,
}

pub struct PostPageRoute {
    data: Resource<Result<aftershock_bridge::Post, ()>>,
    msg: ReadSignal<String>,
}

#[lazy_route]
impl LazyRoute for PostPageRoute {
    fn data() -> Self {
        let params = use_params::<PostParams>();
        let (msg, _) = signal(String::from(MSG_LOAD_DATA_FAILURE));

        let data = Resource::new(
            move || params.read().as_ref().ok().and_then(|p| p.uid.clone()),
            |uid| async move {
                match uid {
                    None => Err(()),
                    Some(empty) if empty.is_empty() => Err(()),
                    Some(uid) => get_post_by_uid(uid).await.map_err(|_| ()),
                }
            },
        );

        Self { data, msg }
    }

    fn view(this: Self) -> AnyView {
        let PostPageRoute { data, msg, .. } = this;

        view! {
            <Suspense>
                {move || {
                    data.get()
                        .map(|result| match result {
                            Ok(post) => view! { <Post post=post.clone() /> }.into_any(),
                            Err(_) => view! { <MessageBox msg=msg /> }.into_any(),
                        })
                }}
            </Suspense>
        }
        .into_any()
    }
}
