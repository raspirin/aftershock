use leptos::{either::Either, prelude::*};

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

#[component]
pub fn AboutPage() -> impl IntoView {
    let (msg, _) = signal(MSG_DATA_NOT_FOUND.to_string());

    view! {
        <Await future=get_post("about") let:data>
            {match data {
                Some(page) => {
                    let body = page.clone().body;
                    Either::Right(
                        view! {
                            <ContentSerif>
                                <ProseContent body=body />
                            </ContentSerif>
                        },
                    )
                }
                None => Either::Left(view! { <MessageBox msg=msg /> }),
            }}
        </Await>
    }
}
