use leptos::prelude::*;

use crate::app::components::MessageBox;

#[component]
pub fn ErrorPage(msg: ReadSignal<String>) -> impl IntoView {
    view! { <MessageBox msg=msg /> }
}
