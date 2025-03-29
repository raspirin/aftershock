use leptos::prelude::*;

use crate::{
    app::components::{ContentSerif, MessageBox},
    MSG_ABOUT_PLACEHOLDER,
};

#[component]
pub fn AboutPage() -> impl IntoView {
    let (msg, _) = signal(MSG_ABOUT_PLACEHOLDER.to_string());

    view! { <MessageBox msg=msg /> }
}
